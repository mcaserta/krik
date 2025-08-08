use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn inject_live_reload_script(output_dir: &Path, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let live_reload_script = format!(r#"
<script>
(function() {{
    // Krik Live Reload
    if (typeof window !== 'undefined') {{
        const ws = new WebSocket('ws://localhost:{port}/__krik_reload');
        
        ws.onopen = function() {{
            console.log('🔄 Krik live reload connected');
        }};
        
        ws.onmessage = function(event) {{
            if (event.data === 'reload') {{
                console.log('🔄 Reloading page...');
                window.location.reload();
            }}
        }};
        
        ws.onclose = function() {{
            console.log('🔄 Krik live reload disconnected, attempting to reconnect...');
            setTimeout(function() {{
                window.location.reload();
            }}, 1000);
        }};
        
        ws.onerror = function() {{
            console.log('🔄 Krik live reload error, attempting to reconnect...');
            setTimeout(function() {{
                window.location.reload();
            }}, 5000);
        }};
    }}
}})();
</script>
</body>"#);

    // Find all HTML files and inject the script
    for entry in WalkDir::new(output_dir) {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("html") {
            let content = fs::read_to_string(path)?;
            
            // Only inject if not already present
            if !content.contains("Krik Live Reload") {
                let modified_content = content.replace("</body>", &live_reload_script);
                fs::write(path, modified_content)?;
            }
        }
    }
    
    Ok(())
}