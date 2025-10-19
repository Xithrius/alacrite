use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use uuid::Uuid;

pub type PeerId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: PeerId,
    pub hostname: String,
    pub ip: IpAddr,
    pub port: u16,
    /// When the peer was last seen, as a Unix timestamp
    pub last_seen: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BroadcastMessage {
    /// Announce presence on the network
    Announce { peer: PeerInfo },
    /// Request for other peers to announce themselves
    DiscoveryRequest { from: PeerInfo },
    /// Response to discovery request
    DiscoveryResponse { peer: PeerInfo },
}

pub struct UdpBroadcastDiscovery {
    socket: UdpSocket,
    broadcast_port: u16,
    local_info: PeerInfo,
    known_peers: HashMap<PeerId, PeerInfo>,
}

impl UdpBroadcastDiscovery {
    pub fn new(port: u16, hostname: String) -> Result<Self> {
        // Bind to the specific broadcast port to listen for incoming messages
        let socket = UdpSocket::bind(format!("0.0.0.0:{port}"))?;
        socket.set_broadcast(true)?;

        info!("Bound UDP socket to port {}", port);

        let id = Uuid::new_v4().to_string();
        let local_ip = local_ip_address::local_ip()?;
        let last_seen = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let local_info = PeerInfo {
            id,
            hostname,
            ip: local_ip,
            port,
            last_seen,
        };

        Ok(Self {
            socket,
            broadcast_port: port,
            local_info,
            known_peers: HashMap::new(),
        })
    }

    pub fn start_listening(&mut self) -> Result<()> {
        info!("Starting UDP broadcast discovery...");
        info!(
            "Local peer: {} ({})",
            self.local_info.hostname, self.local_info.id
        );

        self.send_discovery_request()?;
        self.announce_presence()?;

        let mut buffer = [0; 1024];
        let mut last_announcement = std::time::Instant::now();

        loop {
            // Set a timeout for receiving
            self.socket.set_read_timeout(Some(Duration::from_secs(1)))?;

            match self.socket.recv_from(&mut buffer) {
                Ok((len, addr)) => {
                    let data = &buffer[..len];
                    debug!("Received {} bytes from {}", len, addr);

                    if let Ok(message) = serde_json::from_slice::<BroadcastMessage>(data) {
                        debug!("Parsed message: {:?}", message);
                        self.handle_broadcast_message(message, addr)?;
                    } else {
                        warn!("Failed to parse message from {}", addr);
                        debug!("Raw data: {:?}", &data[..std::cmp::min(len, 100)]);
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Timeout - send periodic announcements every 5 seconds
                    if last_announcement.elapsed() >= Duration::from_secs(5) {
                        self.announce_presence()?;
                        last_announcement = std::time::Instant::now();
                    }
                }
                Err(e) => {
                    warn!("Error receiving UDP broadcast: {}", e);
                }
            }
        }
    }

    /// Send a discovery request to find other peers
    fn send_discovery_request(&self) -> Result<()> {
        let message = BroadcastMessage::DiscoveryRequest {
            from: self.local_info.clone(),
        };

        let data = serde_json::to_vec(&message)?;
        let broadcast_addr = SocketAddr::new(Ipv4Addr::BROADCAST.into(), self.broadcast_port);

        match self.socket.send_to(&data, broadcast_addr) {
            Ok(bytes_sent) => {
                info!(
                    "Sent discovery request to broadcast address ({} bytes)",
                    bytes_sent
                );
            }
            Err(e) => {
                warn!("Failed to send discovery request: {}", e);
                return Err(e.into());
            }
        }

        Ok(())
    }

    /// Announce our presence to the network
    fn announce_presence(&self) -> Result<()> {
        let mut updated_info = self.local_info.clone();
        updated_info.last_seen = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let message = BroadcastMessage::Announce { peer: updated_info };

        let data = serde_json::to_vec(&message)?;
        let broadcast_addr = SocketAddr::new(Ipv4Addr::BROADCAST.into(), self.broadcast_port);

        match self.socket.send_to(&data, broadcast_addr) {
            Ok(bytes_sent) => {
                debug!(
                    "Announced presence to broadcast address ({} bytes)",
                    bytes_sent
                );
            }
            Err(e) => {
                warn!("Failed to announce presence: {}", e);
                return Err(e.into());
            }
        }

        Ok(())
    }

    /// Handle incoming broadcast messages
    fn handle_broadcast_message(
        &mut self,
        message: BroadcastMessage,
        from_addr: SocketAddr,
    ) -> Result<()> {
        match message {
            BroadcastMessage::DiscoveryRequest { from } => {
                info!("Received discovery request from {}", from.hostname);

                // Respond with our info
                let response = BroadcastMessage::DiscoveryResponse {
                    peer: self.local_info.clone(),
                };

                let data = serde_json::to_vec(&response)?;
                self.socket.send_to(&data, from_addr)?;
                info!("Sent discovery response to {}", from.hostname);
            }
            BroadcastMessage::DiscoveryResponse { peer } => {
                info!(
                    "Received discovery response from {} at {}",
                    peer.hostname, peer.ip
                );
                self.known_peers.insert(peer.id.clone(), peer);
            }
            BroadcastMessage::Announce { peer } => {
                if peer.id != self.local_info.id {
                    if self.known_peers.contains_key(&peer.id) {
                        debug!("Already know peer: {} at {}", peer.hostname, peer.ip);
                    } else {
                        info!("Discovered peer: {} at {}", peer.hostname, peer.ip);
                        self.known_peers.insert(peer.id.clone(), peer);
                    }
                }
            }
        }

        Ok(())
    }

    /// Get list of discovered peers
    #[must_use]
    pub fn get_known_peers(&self) -> Vec<PeerInfo> {
        self.known_peers.values().cloned().collect()
    }

    /// Send a message to a specific peer
    pub fn send_to_peer(&self, peer: &PeerInfo, message: &BroadcastMessage) -> Result<()> {
        let data = serde_json::to_vec(message)?;
        let peer_addr = SocketAddr::new(peer.ip, peer.port);
        self.socket.send_to(&data, peer_addr)?;
        Ok(())
    }
}

pub fn run_udp_discovery(port: u16, hostname: String) -> Result<()> {
    let mut discovery = UdpBroadcastDiscovery::new(port, hostname)?;

    discovery.start_listening()?;

    Ok(())
}
