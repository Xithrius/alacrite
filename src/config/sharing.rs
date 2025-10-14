use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SharingConfig {
    /// Maximum number of files in the confirmation queue
    pub max_queue_length: u32,
    /// User needs to confirm the download within this time
    pub confirmation_timeout_seconds: u32,
}

impl Default for SharingConfig {
    fn default() -> Self {
        Self {
            max_queue_length: 10,
            confirmation_timeout_seconds: 10,
        }
    }
}
