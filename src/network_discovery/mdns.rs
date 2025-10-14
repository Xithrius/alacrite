use std::{collections::HashMap, sync::Arc};

use color_eyre::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use parking_lot::Mutex;
use tracing::info;

const DOMAIN_LABEL: &str = "_alacrite._tcp.local.";
const INSTANCE_LABEL: &str = "Alacrite";

pub type DiscoveredServices = Mutex<HashMap<String, String>>;

pub struct NetworkDiscovery {
    daemon: ServiceDaemon,
    #[allow(dead_code)]
    service_info: ServiceInfo,
}

impl NetworkDiscovery {
    pub fn new(port: u16) -> Result<Self> {
        let local_ip = local_ip_address::local_ip()?;
        let daemon = ServiceDaemon::new()?;

        let service_info = ServiceInfo::new(
            DOMAIN_LABEL,
            INSTANCE_LABEL,
            &format!("{local_ip}.local."),
            local_ip.to_string(),
            port,
            vec![],
        )?;

        daemon.register(service_info.clone())?;

        Ok(Self {
            daemon,
            service_info,
        })
    }

    pub async fn start_listening(
        &self,
        discovered_services: Arc<DiscoveredServices>,
    ) -> Result<()> {
        let receiver = self.daemon.browse(DOMAIN_LABEL)?;

        info!("Starting service listener...");

        loop {
            let receiver_clone = receiver.clone();

            let event = tokio::task::spawn_blocking(move || receiver_clone.recv()).await?;

            if let Ok(event) = event {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        let service_name = info.get_fullname();
                        let service_host = info.get_hostname();
                        let service_port = info.get_port().to_string();

                        let local_ip = local_ip_address::local_ip()?.to_string();
                        let Some(service_host_ip) = service_host
                            .split_once(".local.")
                            .map(|s| s.0)
                            .filter(|ip| *ip != local_ip)
                        else {
                            continue;
                        };

                        info!("New service discovered:");
                        info!("  Name: {service_name}");
                        info!("  Host: {service_host_ip:?}");
                        info!("  Port: {service_port}");

                        let mut services = discovered_services.lock();
                        services.insert(service_name.to_string(), service_host.to_string());
                    }
                    ServiceEvent::ServiceRemoved(name, _) => {
                        info!("Service removed: {name}");
                    }
                    _ => {}
                }
            }
        }
    }
}
