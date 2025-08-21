use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};

pub async fn handle_websocket(ws: WebSocket, reload_tx: broadcast::Sender<()>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut reload_rx = reload_tx.subscribe();

    // Handle incoming messages (though we don't expect any for live reload)
    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            if msg.is_close() {
                break;
            }
        }
    });

    // Send reload messages to client
    while (reload_rx.recv().await).is_ok() {
        if ws_tx.send(Message::text("reload")).await.is_err() {
            break;
        }
    }
}
