use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpStream;
use crate::config::Config;
use crate::security::RateLimiter;
use crate::state::ClientRegistry;
use crate::message_bus::MessageBus;
use crate::session::SessionHandler;
use crate::networking::TcpListenerWrapper;

pub struct ChatServer {
    config: Arc<Config>,
    state: Arc<ClientRegistry>,
    rate_limiter: RateLimiter,
    message_bus: MessageBus,
}

impl ChatServer {
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        let (state, _) = ClientRegistry::new(config.message_buffer_size);
        let state = Arc::new(state);
        let rate_limiter = RateLimiter::new(config.rate_limit_msgs, config.rate_limit_window_secs);
        let message_bus = MessageBus::new(state.get_message_sender());

        Self {
            config,
            state,
            rate_limiter,
            message_bus,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListenerWrapper::bind(
            &format!("{}:{}", self.config.host, self.config.port),
            self.config.backlog,
        ).await?;

        tracing::info!(
            "Chat server listening on {}:{}",
            self.config.host,
            self.config.port
        );
        tracing::info!("Max connections: {}", self.config.max_connections);
        tracing::info!("Rate limit: {} msgs/sec", self.config.rate_limit_msgs);

        loop {
            match self.accept_connection(&listener).await {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    async fn accept_connection(&self, listener: &TcpListenerWrapper) -> Result<()> {
        let (stream, peer_addr) = listener.accept().await?;
        let sender = self.state.get_message_sender();

        let config = self.config.clone();
        let state = self.state.clone();
        let rate_limiter = self.rate_limiter.clone();
        let message_bus = self.message_bus.clone();

        tokio::spawn(async move {
            let client_id = state.add_client(format!("user_{}", peer_addr.port()), sender).await;
            if let Err(e) = handle_client(stream, config, state, rate_limiter, message_bus, client_id).await {
                tracing::error!("Client handler error: {}", e);
            }
        });

        Ok(())
    }
}

async fn handle_client(
    stream: TcpStream,
    config: Arc<Config>,
    state: Arc<ClientRegistry>,
    rate_limiter: RateLimiter,
    message_bus: MessageBus,
    client_id: usize,
) -> Result<()> {
    let username = state.get_username(client_id).await;
    let handler = SessionHandler::new(
        config,
        client_id,
        username,
        state,
        rate_limiter,
        message_bus,
    );

    handler.run(stream).await
}