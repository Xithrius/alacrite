use clap::Parser;

const DEFAULT_MDNS_PORT: &str = "8080";
const DEFAULT_WEBSOCKET_PORT: &str = "9090";

#[derive(Parser)]
#[command(name = "alacrite")]
#[command(about = "P2P LAN-bound file sharing")]
#[command(version)]
pub struct Args {
    /// mDNS discovery port
    #[arg(short, long, env = "ALACRITE_MDNS_PORT", default_value = DEFAULT_MDNS_PORT)]
    pub mdns_port: u16,

    /// WebSocket transfer port
    #[arg(short, long, env = "ALACRITE_WS_PORT", default_value = DEFAULT_WEBSOCKET_PORT)]
    pub ws_port: u16,

    /// Log level (error, warn, info, debug, trace)
    #[arg(short, long, env = "ALACRITE_LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    /// Config file path
    #[arg(short, long, env = "ALACRITE_CONFIG")]
    pub config: Option<String>,

    /// Download directory
    #[arg(short, long, env = "ALACRITE_DOWNLOAD_DIR")]
    pub download_dir: Option<String>,

    /// Enable local mode for testing (disables mDNS discovery)
    /// Format: IP:PORT (e.g., 127.0.0.1:3000)
    #[arg(long, env = "ALACRITE_LOCAL")]
    pub local: Option<String>,
}
