use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::error::{KrikError, KrikResult, IoError, IoErrorKind};

pub fn inject_live_reload_script(output_dir: &Path, port: u16) -> KrikResult<()> {
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
        let entry = entry.map_err(|e| KrikError::Io(IoError { kind: IoErrorKind::ReadFailed(e.into_io_error().unwrap_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "walkdir error"))), path: output_dir.to_path_buf(), context: "Walking output directory for live-reload injection".to_string() }))?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("html") {
            let content = fs::read_to_string(path).map_err(|e| KrikError::Io(IoError { kind: IoErrorKind::ReadFailed(e), path: path.to_path_buf(), context: "Reading generated HTML for live-reload injection".to_string() }))?;
            
            // Only inject if not already present
            if !content.contains("Krik Live Reload") {
                let modified_content = content.replace("</body>", &live_reload_script);
                fs::write(path, modified_content).map_err(|e| KrikError::Io(IoError { kind: IoErrorKind::WriteFailed(e), path: path.to_path_buf(), context: "Writing HTML with live-reload script".to_string() }))?;
            }
        }
    }
    
    Ok(())
}