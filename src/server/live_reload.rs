use crate::error::{IoError, IoErrorKind, KrikError, KrikResult};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn inject_live_reload_script(output_dir: &Path, _port: u16) -> KrikResult<()> {
    let live_reload_script = r#"
<script>
(function() {
  // Krik Live Reload
  if (typeof window !== 'undefined') {
    var reconnectDelayMs = 1000;
    function connect() {
      try {
        var protocol = (window.location.protocol === 'https:') ? 'wss' : 'ws';
        var url = protocol + '://' + window.location.host + '/__krik_reload';
        var ws = new WebSocket(url);

        ws.onopen = function() {
          console.log('🔄 Krik live reload connected');
          reconnectDelayMs = 1000; // reset backoff
        };

        ws.onmessage = function(event) {
          if (event.data === 'reload') {
            console.log('🔄 Reloading page...');
            window.location.reload();
          }
        };

        ws.onclose = function() {
          console.log('🔄 Live reload disconnected. Retrying in ' + reconnectDelayMs + 'ms');
          setTimeout(connect, reconnectDelayMs);
          reconnectDelayMs = Math.min(reconnectDelayMs * 2, 10000);
        };

        ws.onerror = function() {
          console.log('🔄 Live reload error. Closing and retrying...');
          try { ws.close(); } catch (e) {}
        };
      } catch (e) {
        console.log('🔄 Live reload init failed. Retrying...', e);
        setTimeout(connect, reconnectDelayMs);
        reconnectDelayMs = Math.min(reconnectDelayMs * 2, 10000);
      }
    }
    connect();
  }
})();
</script>
</body>"#;

    // Find all HTML files and inject the script
    for entry in WalkDir::new(output_dir) {
        let entry = entry.map_err(|e| {
            KrikError::Io(Box::new(IoError {
                kind: IoErrorKind::ReadFailed(e.into_io_error().unwrap_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "walkdir error")
                })),
                path: output_dir.to_path_buf(),
                context: "Walking output directory for live-reload injection".to_string(),
            }))
        })?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("html") {
            let content = fs::read_to_string(path).map_err(|e| {
                KrikError::Io(Box::new(IoError {
                    kind: IoErrorKind::ReadFailed(e),
                    path: path.to_path_buf(),
                    context: "Reading generated HTML for live-reload injection".to_string(),
                }))
            })?;

            // Only inject if not already present
            if !content.contains("Krik Live Reload") {
                let modified_content = content.replace("</body>", live_reload_script);
                fs::write(path, modified_content).map_err(|e| {
                    KrikError::Io(Box::new(IoError {
                        kind: IoErrorKind::WriteFailed(e),
                        path: path.to_path_buf(),
                        context: "Writing HTML with live-reload script".to_string(),
                    }))
                })?;
            }
        }
    }

    Ok(())
}
