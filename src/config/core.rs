use serde::Deserialize;

use crate::config::{downloads::DownloadsConfig, sharing::SharingConfig};

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CoreConfig {
    downloads: DownloadsConfig,
    sharing: SharingConfig,
}
