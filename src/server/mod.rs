use crate::generator::SiteGenerator;
use notify::EventKind;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::{debug, error, info};
use warp::Filter;

pub mod live_reload;
pub mod net;
pub mod static_files;
pub mod watcher;
pub mod websocket;

use live_reload::*;
use net::get_network_interfaces;
use watcher::start_watcher;
use websocket::*;

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
            let ws_route =
                warp::path("__krik_reload")
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
            warp::serve(routes).run(([0, 0, 0, 0], self.port)).await;
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
                .run(([0, 0, 0, 0], self.port))
                .await;
        }

        Ok(())
    }

    fn generate_site(&self) -> Result<(), Box<dyn std::error::Error>> {
        let generator =
            SiteGenerator::new(&self.input_dir, &self.output_dir, self.theme_dir.as_ref())?;
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
            start_watcher(input_dir.clone(), theme_dir.clone(), tx).await;
            // Canonicalize watched roots to compare against canonical event paths
            let canonical_input_dir =
                std::fs::canonicalize(&input_dir).unwrap_or(input_dir.clone());
            let canonical_theme_dir = theme_dir
                .as_ref()
                .and_then(|t| std::fs::canonicalize(t).ok());

            // Persistent generator to preserve document cache across changes
            let mut generator =
                match SiteGenerator::new(&input_dir, &output_dir, theme_dir.as_ref()) {
                    Ok(g) => g,
                    Err(e) => {
                        error!("failed to initialize generator for watcher: {}", e);
                        return;
                    }
                };
            if let Err(e) = generator.scan_files() {
                error!("initial scan failed in watcher: {}", e);
                // continue anyway; incremental may rescan as needed
            }

            loop {
                // Wait for one event
                let event = match rx.recv().await {
                    Some(ev) => ev,
                    None => break,
                };

                // Start a short debounce window to coalesce bursty editor events
                use std::collections::HashMap;
                let mut batched: HashMap<std::path::PathBuf, bool> = HashMap::new(); // path -> is_remove

                let first_is_remove = matches!(event.kind, EventKind::Remove(_));
                for p in event.paths.iter() {
                    let canonical_path = std::fs::canonicalize(p).unwrap_or(p.clone());
                    batched
                        .entry(canonical_path)
                        .and_modify(|r| *r |= first_is_remove)
                        .or_insert(first_is_remove);
                }

                // Collect more events for 250ms of idle
                while let Ok(Some(ev)) =
                    tokio::time::timeout(Duration::from_millis(250), rx.recv()).await
                {
                    let is_remove = matches!(ev.kind, EventKind::Remove(_));
                    for p in ev.paths.iter() {
                        let canonical_path = std::fs::canonicalize(p).unwrap_or(p.clone());
                        batched
                            .entry(canonical_path)
                            .and_modify(|r| *r |= is_remove)
                            .or_insert(is_remove);
                    }
                }

                // Log the batched set
                if !batched.is_empty() {
                    let mut dbg_paths: Vec<String> = batched
                        .iter()
                        .map(|(p, r)| format!("{} (remove={})", p.display(), r))
                        .collect();
                    dbg_paths.sort();
                    debug!("batched paths: {}", dbg_paths.join(", "));
                }
                info!(
                    "📝 {} changed path(s), running incremental build...",
                    batched.len()
                );

                // Run incremental for the batched unique paths using persistent generator/cache
                let mut did_anything = false;
                for (path, is_remove) in batched.into_iter() {
                    // Only handle changes under input_dir or theme_dir
                    let relevant = path.starts_with(&canonical_input_dir)
                        || canonical_theme_dir
                            .as_ref()
                            .map(|t| path.starts_with(t))
                            .unwrap_or(false);
                    if !relevant {
                        debug!("skipping unrelated change: {}", path.display());
                        continue;
                    }
                    debug!(
                        "incremental build for {} (remove={})",
                        path.display(),
                        is_remove
                    );
                    match generator.generate_incremental_for_path(&path, is_remove) {
                        Ok(()) => {
                            did_anything = true;
                        }
                        Err(e) => {
                            error!(
                                "❌ Incremental generation failed for {}: {}",
                                path.display(),
                                e
                            );
                            if let Err(full_err) = generator.generate_site() {
                                error!(
                                    "❌ Full regeneration after failure also failed: {}",
                                    full_err
                                );
                            } else {
                                debug!("fallback full regeneration completed after incremental failure");
                                did_anything = true;
                            }
                        }
                    }
                }

                if !did_anything {
                    let _ = generator.generate_site();
                }

                // Conditionally inject live reload script into generated HTML
                if live_reload {
                    if let Err(e) = inject_live_reload_script(&output_dir, port) {
                        error!("❌ Error injecting live reload script: {}", e);
                    }
                }

                info!("✅ Incremental build complete");
                let _ = reload_tx.send(());
            }
        });

        Ok(())
    }
}

// moved to server/net.rs
