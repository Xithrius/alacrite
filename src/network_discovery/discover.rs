use std::sync::Arc;

use color_eyre::{Result, eyre::Context};
use tracing::{error, info};

use crate::network_discovery::mdns::{DiscoveredServices, NetworkDiscovery};

pub async fn start_network_discovery(port: u16) -> Result<()> {
    let service = NetworkDiscovery::new(port).context("Failed to create network discovery")?;
    info!("Network discovery service created on port {}", port);

    let discovered_services: Arc<DiscoveredServices> = Arc::default();

    tokio::spawn(async move {
        if let Err(e) = service.start_listening(discovered_services).await {
            error!("Error occurred while listening for services: {e}");
        }
    });

    Ok(())
}
