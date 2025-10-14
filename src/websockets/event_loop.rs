use color_eyre::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{
    accept_async, connect_async,
    tungstenite::{Bytes, Message},
};
use tracing::{error, info, warn};

use crate::websockets::{handle_message::handle_binary_data, messages::WebSocketMessage};

pub async fn run_event_loop(websocket_client_url: Option<String>, ws_port: u16) -> Result<()> {
    let mut create_server = true;

    if let Some(websocket_client_url) = websocket_client_url {
        let url = format!("ws://{websocket_client_url}");
        match connect_async(url.clone()).await {
            Ok((ws_stream, _)) => {
                create_server = false;
                info!("Connected as client to {}", url);
                handle_outgoing_connection(ws_stream).await?;
            }
            Err(e) => {
                info!("Failed to connect as client: {}. Becoming server...", e);
            }
        }
    }

    if create_server {
        host_server(ws_port).await?;
    }

    Ok(())
}

async fn host_server(ws_port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{ws_port}")).await?;
    info!("Server listening on port {}", ws_port);

    while let Ok((stream, addr)) = listener.accept().await {
        info!("Incoming connection from {}", addr);

        match accept_async(stream).await {
            Ok(ws_stream) => {
                info!("WebSocket connection established with {}", addr);
                handle_incoming_connection(ws_stream).await?;
            }
            Err(e) => {
                error!("Failed to accept WebSocket connection: {}", e);
            }
        }
    }

    Ok(())
}

async fn handle_incoming_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
) -> Result<()> {
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => {
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
            Message::Ping(_) => {
                info!("Received ping message, sending pong");
                write
                    .send(Message::Pong(Bytes::from_static(b"pong")))
                    .await?;
            }
            Message::Pong(_) => {
                info!("Received pong message");
            }
            Message::Binary(data) => {
                info!("Received binary: {} bytes", data.len());
                handle_binary_data(&data);
            }
            _ => {
                warn!("Received unknown message type");
            }
        }
    }

    Ok(())
}

async fn handle_outgoing_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Result<()> {
    let (mut write, mut read) = ws_stream.split();

    // Send initial ping
    write
        .send(Message::Ping(Bytes::from_static(b"ping")))
        .await?;
    info!("Sent ping message");

    while let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => {
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
            Message::Ping(_) => {
                info!("Received ping message, sending pong");
                write
                    .send(Message::Pong(Bytes::from_static(b"pong")))
                    .await?;
            }
            Message::Pong(_) => {
                info!("Received pong message");
            }
            Message::Binary(data) => {
                info!("Received binary: {} bytes", data.len());
                handle_binary_data(&data);
            }
            _ => {
                warn!("Received unknown message type");
            }
        }
    }

    Ok(())
}

fn handle_websocket_message(msg: WebSocketMessage) {
    match msg {
        WebSocketMessage::FileOffer { filename, size, .. } => {
            info!("File offer received: {} ({} bytes)", filename, size);
            // TODO: Implement file offer handling
        }
        WebSocketMessage::Error { message } => {
            error!("Received error message: {}", message);
        }
        _ => {
            info!("Unhandled message type: {:?}", msg);
        }
    }
}
