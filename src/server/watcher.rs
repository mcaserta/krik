use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tracing::{debug, error};

pub async fn start_watcher(input_dir: PathBuf, theme_dir: Option<PathBuf>, tx: Sender<Event>) {
    tokio::task::spawn_blocking(move || {
        let mut watcher =
            match notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    if matches!(
                        event.kind,
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                    ) {
                        debug!(
                            "notify event captured: kind={:?}, paths={:?}",
                            event.kind, event.paths
                        );
                        let _ = tx.blocking_send(event);
                    }
                }
            }) {
                Ok(w) => w,
                Err(e) => {
                    error!("file watcher creation failed: {}", e);
                    return;
                }
            };

        if let Err(e) = watcher.watch(&input_dir, RecursiveMode::Recursive) {
            error!(
                "failed to watch input directory {}: {}",
                input_dir.display(),
                e
            );
            return;
        }
        if let Some(ref theme_dir) = theme_dir {
            if let Err(e) = watcher.watch(theme_dir, RecursiveMode::Recursive) {
                error!(
                    "failed to watch theme directory {}: {}",
                    theme_dir.display(),
                    e
                );
            }
        }

        // Block this thread; notify uses blocking callbacks
        loop {
            std::thread::sleep(Duration::from_secs(3600));
        }
    });
}
