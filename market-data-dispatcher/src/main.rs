use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("WebSocket服务器已启动，监听地址：{}", addr);

    let clients = Arc::new(Mutex::new(HashMap::new()));
    let (tx, _rx) = broadcast::channel::<String>(100);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let rx = tx.subscribe();
        let clients = clients.clone();

        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                let id = uuid::Uuid::new_v4().to_string();
                clients.lock().unwrap().insert(id.clone(), tx.clone());

                let mut rx = rx;
                tokio::spawn(async move {
                    while let Ok(msg) = rx.recv().await {
                        println!("Received Message: {}", msg);
                        ws_sender.send(Message::Text(msg)).await.unwrap();
                    }
                });

                while let Some(Ok(msg)) = ws_receiver.next().await {
                    if let Message::Text(text) = msg {
                        tx.send(text).unwrap();
                    }
                }

                clients.lock().unwrap().remove(&id);
            }
        });
    }
}

