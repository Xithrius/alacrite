#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::struct_excessive_bools
)]

pub mod cli;
pub mod config;
pub mod logging;
pub mod network_discovery;
pub mod websockets;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tracing::{info, warn};

use crate::{
    cli::{Args, Command},
    config::core::CoreConfig,
    logging::init_logging,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let _config = CoreConfig::default();

    init_logging(&args.log_level).map_err(|e| eyre!("Failed to initialize logging: {}", e))?;

    if let Some(command) = args.command {
        match command {
            Command::Peers { verbose: _ } => {
                info!("Peers (verbose): listing discovered/known peers...");
            }
            Command::Send { to, paths } => {
                info!("Send to {to}: {:?}", paths);
            }
        }

        return Ok(());
    }

    // Default: run websocket event loop
    websockets::event_loop::run_event_loop(args.local, args.ws_port).await?;

    Ok(())
}
