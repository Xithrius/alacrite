#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub mod config;
pub mod service;

use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::service::AlacriteService;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = AlacriteService::new(PORT)?;

    let discovered_services: Arc<Mutex<HashMap<String, String>>> = Arc::default();

    tokio::spawn({
        async move {
            if let Err(e) = service.start_listening(discovered_services).await {
                eprintln!("Error occurred while listening for services: {e}");
            }
        }
    });

    Ok(())
}
