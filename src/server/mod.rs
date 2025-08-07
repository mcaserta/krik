use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::broadcast;
use warp::Filter;
use notify::{Watcher, RecursiveMode, Event, EventKind};
use crate::generator::SiteGenerator;
use tracing::{info, error};

pub mod websocket;
pub mod static_files;
pub mod live_reload;

use websocket::*;
use live_reload::*;

pub struct DevServer {
    input_dir: PathBuf,
    output_dir: PathBuf,
    theme_dir: Option<PathBuf>,
    port: u16,
    live_reload: bool,
    reload_tx: broadcast::Sender<()>,
}

impl DevServer {
    pub fn new(
        input_dir: PathBuf,
        output_dir: PathBuf,
        theme_dir: Option<PathBuf>,
        port: u16,
        live_reload: bool,
    ) -> Self {
        let (reload_tx, _) = broadcast::channel(100);
        
        Self {
            input_dir,
            output_dir,
            theme_dir,
            port,
            live_reload,
            reload_tx,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Initial site generation
        self.generate_site()?;
        
        // Start file watcher
        self.start_file_watcher().await?;
        
        // Get network interfaces
        let interfaces = get_network_interfaces();
        
        // Setup static file serving
        let output_dir = self.output_dir.clone();

        // Build routes based on live_reload setting
        if self.live_reload {
            // Setup with WebSocket for live reload
            let static_route = warp::fs::dir(output_dir.clone())
                .or(warp::path::end().and(warp::fs::file(output_dir.join("index.html"))));
            
            let reload_tx = self.reload_tx.clone();
            let ws_route = warp::path("__krik_reload")
                .and(warp::ws())
                .map(move |ws: warp::ws::Ws| {
                    let tx = reload_tx.clone();
                    ws.on_upgrade(move |websocket| handle_websocket(websocket, tx))
                });
            
            let routes = ws_route.or(static_route);
            
            info!("🚀 Krik development server started!");
            info!("📁 Serving: {}", self.output_dir.display());
            info!("👀 Watching: {}", self.input_dir.display());
            if let Some(ref theme_dir) = self.theme_dir {
                info!("👀 Watching theme: {}", theme_dir.display());
            }
            info!("🌐 Available on:");
            
            for interface in &interfaces {
                info!("   http://{}:{}", interface, self.port);
            }
            
            info!("✅ Live reload enabled");
            info!("\n💡 Press Ctrl+C to stop");

            // Start server with live reload
            warp::serve(routes)
                .bind(([0, 0, 0, 0], self.port))
                .await;
        } else {
            // Setup without WebSocket for static serving only
            let static_route = warp::fs::dir(output_dir.clone())
                .or(warp::path::end().and(warp::fs::file(output_dir.join("index.html"))));
            
            info!("🚀 Krik development server started!");
            info!("📁 Serving: {}", self.output_dir.display());
            info!("👀 Watching: {}", self.input_dir.display());
            if let Some(ref theme_dir) = self.theme_dir {
                info!("👀 Watching theme: {}", theme_dir.display());
            }
            info!("🌐 Available on:");
            
            for interface in &interfaces {
                info!("   http://{}:{}", interface, self.port);
            }
            
            info!("❌ Live reload disabled");
            info!("\n💡 Press Ctrl+C to stop");

            // Start server without live reload
            warp::serve(static_route)
                .bind(([0, 0, 0, 0], self.port))
                .await;
        }

        Ok(())
    }

    fn generate_site(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut generator = SiteGenerator::new(&self.input_dir, &self.output_dir, self.theme_dir.as_ref())?;
        generator.scan_files()?;
        generator.generate_site()?;
        
        // Conditionally inject live reload script into HTML files
        if self.live_reload {
            inject_live_reload_script(&self.output_dir, self.port)?;
        }
        
        Ok(())
    }

    async fn start_file_watcher(&self) -> Result<(), Box<dyn std::error::Error>> {
        let input_dir = self.input_dir.clone();
        let output_dir = self.output_dir.clone();
        let theme_dir = self.theme_dir.clone();
        let reload_tx = self.reload_tx.clone();
        let port = self.port;
        let live_reload = self.live_reload;

        tokio::spawn(async move {
            let (tx, mut rx) = tokio::sync::mpsc::channel(100);
            
            let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)) {
                        let _ = tx.blocking_send(event);
                    }
                }
            }).expect("Failed to create file watcher");

            watcher.watch(&input_dir, RecursiveMode::Recursive)
                .expect("Failed to watch input directory");

            if let Some(ref theme_dir) = theme_dir {
                watcher.watch(theme_dir, RecursiveMode::Recursive)
                    .expect("Failed to watch theme directory");
            }

            let mut last_generation = std::time::Instant::now();
            
            while let Some(_event) = rx.recv().await {
                // Debounce rapid file changes
                let now = std::time::Instant::now();
                if now.duration_since(last_generation) < Duration::from_millis(100) {
                    continue;
                }
                last_generation = now;

                info!("📝 File changed, regenerating site...");
                
                // Regenerate site
                if let Ok(mut generator) = SiteGenerator::new(&input_dir, &output_dir, theme_dir.as_ref()) {
                    if let Err(e) = generator.scan_files() {
                        error!("❌ Error scanning files: {}", e);
                        continue;
                    }
                    if let Err(e) = generator.generate_site() {
                        error!("❌ Error generating site: {}", e);
                        continue;
                    }
                    
                    // Conditionally inject live reload script
                    if live_reload {
                        if let Err(e) = inject_live_reload_script(&output_dir, port) {
                            error!("❌ Error injecting live reload script: {}", e);
                            continue;
                        }
                    }
                    
                    info!("✅ Site regenerated");
                    
                    // Notify connected clients to reload
                    let _ = reload_tx.send(());
                }
            }
        });

        Ok(())
    }
}

fn get_network_interfaces() -> Vec<String> {
    let mut interfaces = vec!["127.0.0.1".to_string()];
    
    // Try to get local network IP
    if let Ok(local_ip) = local_ip_address::local_ip() {
        if local_ip.to_string() != "127.0.0.1" {
            interfaces.push(local_ip.to_string());
        }
    }
    
    // Try to get all network interfaces
    if let Ok(network_interfaces) = local_ip_address::list_afinet_netifas() {
        for (_name, ip) in network_interfaces {
            let ip_str = ip.to_string();
            if !ip_str.starts_with("127.") && !ip_str.starts_with("169.254.") && !interfaces.contains(&ip_str) {
                interfaces.push(ip_str);
            }
        }
    }
    
    interfaces
}