use clap::{Parser, Subcommand};

const DEFAULT_UDP_PORT: &str = "7070";

#[derive(Parser)]
#[command(name = "alacrite")]
#[command(about = "P2P LAN-bound file sharing")]
#[command(version)]
pub struct Args {
    /// Log level (error, warn, info, debug, trace)
    #[arg(short, long, env = "ALACRITE_LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    /// Config file path
    #[arg(short, long, env = "ALACRITE_CONFIG")]
    pub config: Option<String>,

    /// WebSocket transfer port
    // #[arg(short, long, env = "ALACRITE_WS_PORT", default_value = DEFAULT_WEBSOCKET_PORT)]
    // pub ws_port: u16,

    /// Download directory
    // #[arg(short, long, env = "ALACRITE_DOWNLOAD_DIR")]
    // pub download_dir: Option<String>,

    /// Enable local mode for testing (disables UDP discovery)
    /// Format: IP:PORT (e.g., 127.0.0.1:3000)
    // #[arg(long, env = "ALACRITE_LOCAL")]
    // pub local: Option<String>,

    /// UDP broadcast discovery port
    #[arg(short, long, env = "ALACRITE_UDP_PORT", default_value = DEFAULT_UDP_PORT)]
    pub udp_port: u16,

    /// Peer name for identification
    // #[arg(short, long, env = "ALACRITE_NAME", default_value = "alacrite-peer")]
    // pub name: String,

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Start UDP broadcast discovery
    Discover {
        /// Show detailed information
        #[arg(long)]
        verbose: bool,
    },

    /// List discovered peers and known peers
    Peers {
        /// Show detailed information
        #[arg(long)]
        verbose: bool,
    },

    /// Send files to a specific peer
    Send {
        /// Target peer by ID or name
        #[arg(long, short = 't')]
        to: String,

        /// Files or directories to send
        #[arg(required = true)]
        paths: Vec<String>,
    },
}
