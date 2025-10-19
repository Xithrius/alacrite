use color_eyre::Result;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::{Bytes, Message};
use tracing::{error, info};

use crate::websockets::messages::WebSocketMessage;

pub async fn handle_client_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Result<()> {
    let (mut write, mut read) = ws_stream.split();

    write
        .send(Message::Ping(Bytes::from_static(b"ping")))
        .await?;
    info!("Sent initial ping to outgoing connection");

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received text message: {text}");
            }
            Err(_) => todo!(),
            _ => todo!(),
        }
    }

    Ok(())

    // loop {
    //     let msg = read.next().await?;
    //     match msg {
    //         Some(Ok(Message::Text(text))) => {
    //             info!("Received text message: {text}");
    //             match WebSocketMessage::from_json(&text) {
    //                 Ok(ws_msg) => {
    //                     info!("Parsed WebSocket message: {:?}", ws_msg);
    //                     handle_websocket_message(ws_msg);
    //                 }
    //                 Err(e) => {
    //                     error!("Failed to parse JSON message: {}", e);
    //                     info!("Raw message: {text}");
    //                 }
    //             }
    //         }
    //         Some(Ok(Message::Ping(_))) => {
    //             info!("Received ping, sending pong");
    //             write
    //                 .send(Message::Pong(Bytes::from_static(b"pong")))
    //                 .await?;
    //         }
    //         Some(Ok(Message::Pong(_))) => {
    //             last_pong = Instant::now();
    //             info!("Received pong - connection healthy");
    //         }
    //         Some(Ok(Message::Binary(data))) => {
    //             info!("Received binary: {} bytes", data.len());
    //             handle_binary_data(&data);
    //         }
    //         Some(Ok(_)) => {
    //             warn!("Received unknown message type");
    //         }
    //         Some(Err(e)) => {
    //             error!("WebSocket error: {}", e);
    //             break;
    //         }
    //         None => {
    //             info!("Connection closed by peer");
    //             break;
    //         }
    //     }
    // }
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

fn handle_binary_data(data: &[u8]) {
    info!("Handling binary data: {} bytes", data.len());
    // TODO: Implement binary data handling (file chunks)
}
