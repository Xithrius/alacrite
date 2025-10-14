use serde::{Deserialize, Serialize};

/// WebSocket message types for file sharing
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    /// Authentication message
    Auth {
        token: String,
    },
    /// Response to authentication
    AuthResponse {
        success: bool,
        message: Option<String>,
    },

    /// File offer from sender
    FileOffer {
        filename: String,
        size: u64,
        hash: String,
        mime_type: String,
    },
    /// Response to file offer
    FileAccept {
        accept: bool,
        reason: Option<String>,
    },

    /// Transfer control messages
    TransferStart {
        chunk_size: usize,
    },
    TransferChunk {
        data: Vec<u8>,
        offset: u64,
    },
    TransferComplete {
        hash: String,
    },

    /// Error message
    Error {
        message: String,
    },
}

impl WebSocketMessage {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
