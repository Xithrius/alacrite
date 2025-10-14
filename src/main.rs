#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub mod cli;
pub mod config;
pub mod logging;
pub mod network_discovery;
pub mod websockets;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tracing::warn;

use crate::{cli::Args, config::core::CoreConfig, logging::init_logging};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = CoreConfig::default();

    init_logging(&args.log_level).map_err(|e| eyre!("Failed to initialize logging: {}", e))?;

    // if let Some(local_addr) = &args.local {
    // discover::start_network_discovery(args.ws_port).await?;
    // return Ok(());
    // }

    websockets::event_loop::run_event_loop(args.local, args.ws_port).await?;

    Ok(())
}
