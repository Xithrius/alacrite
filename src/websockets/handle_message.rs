use color_eyre::Result;
use tracing::{error, info};

use crate::websockets::{WriteSink, messages::WebSocketMessage};

#[allow(unused_variables)]
pub fn handle_message(msg: WebSocketMessage, write: &mut WriteSink) -> Result<()> {
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
    Ok(())
}

pub fn handle_binary_data(data: &[u8]) {
    info!("Handling binary data: {} bytes", data.len());
    // TODO: Implement binary data handling (file chunks)
}
