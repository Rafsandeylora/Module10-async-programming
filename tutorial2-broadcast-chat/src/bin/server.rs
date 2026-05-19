use futures_util::{SinkExt, StreamExt};
use std::{error::Error, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Sender};
use tokio_tungstenite::{accept_async, tungstenite::Message};

const ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind(ADDR).await?;
    let (tx, _rx) = broadcast::channel::<String>(100);

    println!("Rafsan's Computer: WebSocket chat server running on ws://{ADDR}");

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        println!("Rafsan's Computer: new TCP connection from {peer_addr}");

        let tx = tx.clone();

        tokio::spawn(async move {
            if let Err(error) = handle_connection(peer_addr, stream, tx).await {
                eprintln!("Rafsan's Computer: error with {peer_addr}: {error}");
            }
        });
    }
}

async fn handle_connection(
    peer_addr: SocketAddr,
    stream: TcpStream,
    tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    println!("Rafsan's Computer: WebSocket connection established with {peer_addr}");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            incoming_message = ws_receiver.next() => {
                match incoming_message {
                    Some(Ok(Message::Text(text))) => {
                        let decorated_message = format!("Rafsan's Computer - from {peer_addr}: {text}");

                        println!("{decorated_message}");

                        let _ = tx.send(decorated_message);
                    }
                    Some(Ok(Message::Close(_))) => {
                        println!("Rafsan's Computer: {peer_addr} closed the connection");
                        break;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(error)) => {
                        eprintln!("Rafsan's Computer: websocket error from {peer_addr}: {error}");
                        break;
                    }
                    None => {
                        println!("Rafsan's Computer: {peer_addr} disconnected");
                        break;
                    }
                }
            }

            broadcast_message = rx.recv() => {
                match broadcast_message {
                    Ok(text) => {
                        ws_sender.send(Message::Text(text.into())).await?;
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        eprintln!("Rafsan's Computer: client {peer_addr} lagged and skipped {skipped} messages");
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        }
    }

    println!("Rafsan's Computer: connection handler ended for {peer_addr}");
    Ok(())
}