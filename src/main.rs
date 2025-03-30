use std::{net::IpAddr, time::Duration};

use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Error, RecordKind};

const SERVICE_NAME: &str = "_alacrite._tcp.local";

#[async_std::main]
async fn main() -> Result<(), Error> {
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(5))?.listen();
    pin_mut!(stream);

    println!("Discovering services...");

    while let Some(Ok(response)) = stream.next().await {
        let ip_addrs: Vec<IpAddr> = response
            .records()
            .filter_map(|record| match record.kind {
                RecordKind::A(addr) => Some(IpAddr::V4(addr)),
                RecordKind::AAAA(addr) => Some(IpAddr::V6(addr)),
                _ => None,
            })
            .collect();

        println!("Found service:");
        println!("  IP Addresses: {:?}", ip_addrs);
    }

    Ok(())
}
