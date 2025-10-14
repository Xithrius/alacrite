use color_eyre::Result;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Bytes, Message},
};
use tracing::{error, info, warn};

use crate::websockets::{
    handle_message::{handle_binary_data, handle_message},
    messages::WebSocketMessage,
};

pub async fn run_event_loop(ws_url: &str) -> Result<()> {
    let (ws_stream, _) = connect_async(ws_url).await?;
    let (mut write, mut read) = ws_stream.split();

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
                        handle_message(ws_msg, &mut write).await?;
                    }
                    Err(e) => {
                        error!("Failed to parse JSON message: {}", e);
                        info!("Raw message: {text}");
                    }
                }
            }
            Message::Ping(_) => {
                info!("Received ping message");
                write
                    .send(Message::Pong(Bytes::from_static(b"pong")))
                    .await?;
            }
            Message::Pong(_) => {
                info!("Received pong message");
            }
            Message::Binary(data) => {
                info!("Received binary: {} bytes", data.len());
                // Handle binary data (could be file chunks)
                handle_binary_data(&data);
            }
            _ => {
                warn!("Received unknown message type");
            }
        }
    }

    Ok(())
}
