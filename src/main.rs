use std::time::Duration;
use tcp_chat::{ChatServer, Config};
use tokio::signal;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::default();

    tracing::info!("Starting TCP Chat Server");
    tracing::info!("Configuration: host={}, port={}, max_connections={}",
        config.host, config.port, config.max_connections);

    let server = ChatServer::new(config);

    tokio::spawn(async {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            tracing::info!("Server heartbeat - active connections: tracked via state");
        }
    });

    tokio::select! {
        result = server.run() => {
            if let Err(e) = result {
                tracing::error!("Server error: {}", e);
            }
        }
        _ = signal::ctrl_c() => {
            tracing::info!("Received shutdown signal, exiting gracefully...");
        }
    }

    Ok(())
}