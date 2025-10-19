use std::time::{Duration, Instant};

use color_eyre::Result;
use futures::{SinkExt, StreamExt};
use tokio::time::interval;
use tokio_tungstenite::tungstenite::{Bytes, Message};
use tracing::{error, info, warn};

use crate::websockets::messages::WebSocketMessage;

pub async fn handle_server_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
) -> Result<()> {
    let (mut write, mut read) = ws_stream.split();
    let mut last_pong = Instant::now();
    let mut ping_interval = interval(Duration::from_secs(30)); // Ping every 30 seconds

    // Send initial ping to establish connection
    write
        .send(Message::Ping(Bytes::from_static(b"ping")))
        .await?;
    info!("Sent initial ping to incoming connection");

    loop {
        tokio::select! {
            // Handle incoming messages
            msg = read.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        info!("Received text message: {text}");
                        match WebSocketMessage::from_json(&text) {
                            Ok(ws_msg) => {
                                info!("Parsed WebSocket message: {:?}", ws_msg);
                                handle_websocket_message(ws_msg);
                            }
                            Err(e) => {
                                error!("Failed to parse JSON message: {}", e);
                                info!("Raw message: {text}");
                            }
                        }
                    }
                    Some(Ok(Message::Ping(_))) => {
                        info!("Received ping, sending pong");
                        write
                            .send(Message::Pong(Bytes::from_static(b"pong")))
                            .await?;
                    }
                    Some(Ok(Message::Pong(_))) => {
                        last_pong = Instant::now();
                        info!("Received pong - connection healthy");
                    }
                    Some(Ok(Message::Binary(data))) => {
                        info!("Received binary: {} bytes", data.len());
                        handle_binary_data(&data);
                    }
                    Some(Ok(_)) => {
                        warn!("Received unknown message type");
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    None => {
                        info!("Connection closed by peer");
                        break;
                    }
                }
            }
            // Send periodic pings
            _ = ping_interval.tick() => {
                let time_since_last_pong = last_pong.elapsed();
                if time_since_last_pong > Duration::from_secs(60) {
                    warn!("No pong received for {}s, connection may be dead", time_since_last_pong.as_secs());
                    break;
                }

                write
                    .send(Message::Ping(Bytes::from_static(b"ping")))
                    .await?;
                info!("Sent periodic ping");
            }
        }
    }

    info!("Incoming connection closed");
    Ok(())
}

fn handle_websocket_message(msg: WebSocketMessage) {
    match msg {
        WebSocketMessage::Auth { token } => {
            info!("Authentication received: {token}");
            // TODO: Implement authentication handling
        }
        WebSocketMessage::AuthResponse { success, message } => {
            info!("Authentication response: {success} - {message:?}");
            // TODO: Implement authentication response handling
        }
        _ => {}
    }
}

fn handle_binary_data(data: &[u8]) {
    info!("Handling binary data: {} bytes", data.len());
    // TODO: Implement binary data handling (file chunks)
}
