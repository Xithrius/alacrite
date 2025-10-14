use serde::Deserialize;

use crate::config::{
    downloads::DownloadsConfig, notifications::NotificationsConfig, sharing::SharingConfig,
};

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CoreConfig {
    pub downloads: DownloadsConfig,
    pub sharing: SharingConfig,
    pub notifications: NotificationsConfig,
}
