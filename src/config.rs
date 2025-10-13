use anyhow::Result;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CoreConfig {
    asdf: String,
}

pub fn load_config(path: &str) -> Result<CoreConfig> {
    let settings = Config::builder()
        .add_source(File::with_name(path))
        .build()?;

    let config: CoreConfig = settings.try_deserialize()?;

    Ok(config)
}
