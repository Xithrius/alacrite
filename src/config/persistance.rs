use std::{env, path::PathBuf};

use color_eyre::Result;
use config::{Config, File};

use crate::config::core::CoreConfig;

const BINARY_NAME: &str = env!("CARGO_BIN_NAME");

#[must_use]
pub fn config_path(file: &str) -> PathBuf {
    match env::consts::OS {
        "linux" | "macos" => {
            let home = env::var("HOME").unwrap();
            PathBuf::from(home)
                .join(".config")
                .join(BINARY_NAME)
                .join(file)
        }
        "windows" => {
            let appdata = env::var("APPDATA").unwrap();
            PathBuf::from(appdata).join(BINARY_NAME).join(file)
        }
        _ => unimplemented!(),
    }
}

pub fn load_config(path: &str) -> Result<CoreConfig> {
    let settings = Config::builder()
        .add_source(File::with_name(path))
        .build()?;

    let config: CoreConfig = settings.try_deserialize()?;

    Ok(config)
}
