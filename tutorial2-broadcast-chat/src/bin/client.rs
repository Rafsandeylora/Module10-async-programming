use futures_util::{SinkExt, StreamExt};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::Message};

const SERVER_URL: &str = "ws://127.0.0.1:2000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Rafsan's Computer: connecting to {SERVER_URL}");

    let (ws_stream, _) = connect_async(SERVER_URL).await?;
    println!("Rafsan's Computer: connected. Type a message and press Enter.");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    loop {
        tokio::select! {
            line = lines.next_line() => {
                match line? {
                    Some(text) => {
                        let trimmed = text.trim();

                        if trimmed.is_empty() {
                            continue;
                        }

                        ws_sender.send(Message::Text(trimmed.to_string().into())).await?;
                    }
                    None => break,
                }
            }

            incoming_message = ws_receiver.next() => {
                match incoming_message {
                    Some(Ok(Message::Text(text))) => {
                        println!("Rafsan's Computer: message from server: {text}");
                    }
                    Some(Ok(Message::Close(_))) => {
                        println!("Rafsan's Computer: server closed the connection");
                        break;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(error)) => {
                        eprintln!("Rafsan's Computer: websocket error: {error}");
                        break;
                    }
                    None => {
                        println!("Rafsan's Computer: server disconnected");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}