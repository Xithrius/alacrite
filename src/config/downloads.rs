use std::{
    collections::{HashMap, HashSet},
    env,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

pub fn download_directory() -> PathBuf {
    match env::consts::OS {
        "linux" => {
            // Linux: Use XDG_DOWNLOAD_DIR if set, otherwise fallback to ~/Downloads
            env::var("XDG_DOWNLOAD_DIR").map_or_else(
                |_| {
                    let home = env::var("HOME").unwrap();
                    PathBuf::from(home).join("Downloads")
                },
                PathBuf::from,
            )
        }
        "macos" => {
            // macOS: ~/Downloads
            let home = env::var("HOME").unwrap();
            PathBuf::from(home).join("Downloads")
        }
        "windows" => {
            // Windows: Use USERPROFILE\Downloads
            let userprofile = env::var("USERPROFILE").unwrap();
            PathBuf::from(userprofile).join("Downloads")
        }
        _ => unimplemented!(),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
/// Default configuration for extensions that don't have a specific configuration
pub struct PrimaryDownloadsConfig {
    /// Directory to store files of this extension
    pub directory: PathBuf,
    /// Maximum file size in bytes for the directory
    pub directory_data_limit: Option<u64>,
    /// Maximum file size in bytes for this extension
    pub data_limit: Option<u64>,
    /// Whether files of this extension should be auto-downloaded
    /// If enabled and the file is below the data limit, it will be downloaded
    pub auto_download: bool,
    /// Whether to download parts of a file instead of all at once
    pub partial_downloads: bool,
    /// Whether to check the hash of the file after download
    /// Partial downloads will also be hashed and checked if this is enabled
    pub hash_checking: bool,
}

impl Default for PrimaryDownloadsConfig {
    fn default() -> Self {
        Self {
            directory: download_directory(),
            directory_data_limit: None,
            data_limit: None,
            auto_download: false,
            partial_downloads: false,
            hash_checking: false,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
/// Extension-specific configuration
pub struct ExtensionConfig {
    /// Directory to store files of this extension
    pub directory: PathBuf,
    /// Maximum file size in bytes for the directory
    pub directory_data_limit: Option<u64>,
    /// Maximum file size in bytes for this extension
    pub file_data_limit: Option<u64>,
    /// Whether files of this extension should be auto-downloaded
    /// If enabled and the file is below the data limit, it will be downloaded
    pub auto_download: bool,
    /// Whether to download parts of a file instead of all at once
    pub partial_downloads: bool,
    /// Whether to check the hash of the file after download
    /// Partial downloads will also be hashed and checked if this is enabled
    pub hash_checking: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DownloadsConfig {
    pub primary: PrimaryDownloadsConfig,
    /// Extension-specific configurations
    /// Key: file extension (e.g., "zip", "jpeg", "dmg")
    /// Value: configuration for that extension
    pub extension_configs: HashMap<String, ExtensionConfig>,

    pub allowed_extensions: HashSet<String>,
    pub blocked_extensions: HashSet<String>,
    /// Maximum number of parallel downloads
    pub max_parallel_downloads: u32,
}

impl Default for DownloadsConfig {
    fn default() -> Self {
        Self {
            primary: PrimaryDownloadsConfig::default(),
            extension_configs: HashMap::default(),
            allowed_extensions: HashSet::default(),
            blocked_extensions: HashSet::default(),
            max_parallel_downloads: 1,
        }
    }
}
