use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::broadcast;

pub async fn start_dashboard_ws(mut rx: broadcast::Receiver<String>) {
    // Bind strictly to local loopback to prevent external network access
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("[*] Dashboard WebSocket active on ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        let mut ws_stream = accept_async(stream).await.expect("Failed WS handshake");
        let mut rx_clone = rx.resubscribe();

        tokio::spawn(async move {
            while let Ok(msg) = rx_clone.recv().await {
                let payload = tokio_tungstenite::tungstenite::Message::Text(msg);
                if ws_stream.send(payload).await.is_err() {
                    break;
                }
            }
        });
    }
}
