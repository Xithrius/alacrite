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
use gethostname::gethostname;
use tracing::{info, warn};

use crate::{
    cli::{Args, Command},
    config::core::CoreConfig,
    logging::init_logging,
    network_discovery::udp_broadcast,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let _config = CoreConfig::default();

    init_logging(&args.log_level).map_err(|e| eyre!("Failed to initialize logging: {}", e))?;

    // Get hostname for peer identification
    let hostname = gethostname()
        .into_string()
        .unwrap_or_else(|_| "unknown-host".to_string());

    if let Some(command) = args.command {
        match command {
            Command::Peers { verbose: _ } => {
                info!("Peers (verbose): listing discovered/known peers...");
            }
            Command::Send { to, paths } => {
                info!("Send to {to}: {:?}", paths);
            }
            Command::Discover { verbose } => {
                info!("Starting UDP broadcast discovery...");
                if verbose {
                    info!("Verbose mode enabled - showing detailed discovery info");
                }
                udp_broadcast::run_udp_discovery(args.udp_port, hostname)?;
            }
        }

        return Ok(());
    }

    // Default: run UDP broadcast discovery
    info!("Starting Alacrite P2P file sharing...");
    info!("Peer name: {}", hostname);
    info!("UDP broadcast port: {}", args.udp_port);

    udp_broadcast::run_udp_discovery(args.udp_port, hostname)?;

    Ok(())
}
