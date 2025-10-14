use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NotificationsConfig {
    pub enabled: bool,
    pub on_download_accepted: bool,
    pub on_download_error: bool,
    pub on_download_complete: bool,
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            on_download_accepted: true,
            on_download_error: true,
            on_download_complete: true,
        }
    }
}
