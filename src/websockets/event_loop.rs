use std::time::Duration;

use color_eyre::Result;
use tokio::{net::TcpListener, time::sleep};
use tokio_tungstenite::{accept_async, connect_async};
use tracing::{error, info};

use crate::websockets::handlers::{
    client::handle_client_connection, server::handle_server_connection,
};

pub async fn run_event_loop(websocket_client_url: Option<String>, ws_port: u16) -> Result<()> {
    if let Some(websocket_client_url) = websocket_client_url {
        let url = format!("ws://{websocket_client_url}");

        // Try to connect with retries (useful after UDP discovery)
        let mut retries = 5;
        let mut connected = false;

        while retries > 0 && !connected {
            match connect_async(url.clone()).await {
                Ok((ws_stream, _)) => {
                    info!("Connected as client to {}", url);
                    handle_client_connection(ws_stream).await?;
                    connected = true;
                }
                Err(e) => {
                    retries -= 1;
                    if retries > 0 {
                        info!(
                            "Failed to connect as client: {}. Retrying in 2s... ({retries} retries left)",
                            e
                        );
                        sleep(Duration::from_secs(2)).await;
                    } else {
                        info!(
                            "Failed to connect as client after all retries: {}. Becoming server...",
                            e
                        );
                    }
                }
            }
        }

        // If we connected successfully, we're done
        if connected {
            return Ok(());
        }
    }

    // Start server (either because no client URL provided, or connection failed)
    host_server(ws_port).await?;
    Ok(())
}

async fn host_server(ws_port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{ws_port}")).await?;
    info!("Server listening on port {}", ws_port);

    while let Ok((stream, addr)) = listener.accept().await {
        info!("Incoming connection from {}", addr);

        match accept_async(stream).await {
            Ok(ws_stream) => {
                info!("WebSocket connection established with {}", addr);
                handle_server_connection(ws_stream).await?;
            }
            Err(e) => {
                error!("Failed to accept WebSocket connection: {}", e);
            }
        }
    }

    Ok(())
}
