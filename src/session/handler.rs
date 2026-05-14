use anyhow::{Result, anyhow};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::sync::Arc;
use crate::config::Config;
use crate::security::{InputSanitizer, RateLimiter};
use crate::protocol::parser::{CommandType, ChatMessage, ParsedMessage, ProtocolParser};
use crate::state::ClientRegistry;
use crate::session::TimeoutManager;
use crate::message_bus::MessageBus;
use tokio::sync::mpsc;

pub struct SessionHandler {
    #[allow(dead_code)]
    config: Arc<Config>,
    client_id: usize,
    username: String,
    state: Arc<ClientRegistry>,
    sanitizer: InputSanitizer,
    rate_limiter: RateLimiter,
    parser: ProtocolParser,
    #[allow(dead_code)]
    timeout_manager: TimeoutManager,
    message_bus: MessageBus,
}

impl SessionHandler {
    pub fn new(
        config: Arc<Config>,
        client_id: usize,
        username: String,
        state: Arc<ClientRegistry>,
        rate_limiter: RateLimiter,
        message_bus: MessageBus,
    ) -> Self {
        let sanitizer = InputSanitizer::new(config.max_username_length, config.max_message_size);
        let parser = ProtocolParser::new();
        let timeout_manager = TimeoutManager::new(config.idle_timeout_secs);

        Self {
            config,
            client_id,
            username,
            state,
            sanitizer,
            rate_limiter,
            parser,
            timeout_manager,
            message_bus,
        }
    }

    pub async fn run(self, stream: TcpStream) -> Result<()> {
        let peer_addr = stream.peer_addr()?;
        tracing::info!("Client {} connected from {}", self.client_id, peer_addr);

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader).lines();

        self.message_bus.broadcast(ChatMessage::system_message(
            format!("{} joined the chat", self.username)
        )).await?;

        let (tx, mut rx) = mpsc::channel::<String>(100);
        let mut receiver = self.message_bus.subscribe();

        tokio::spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                if tx.send(msg).await.is_err() {
                    break;
                }
            }
        });

        loop {
            tokio::select! {
                result = reader.next_line() => {
                    match result {
                        Ok(Some(line)) => {
                            if let Err(e) = self.handle_line(line.trim()).await {
                                if e.to_string() == "QUIT" {
                                    break;
                                }
                                let _ = writer.write_all(format!("[ERROR] {}\n", e).as_bytes()).await;
                            }
                        }
                        Ok(None) => {
                            tracing::info!("Client {} disconnected (EOF)", self.client_id);
                            break;
                        }
                        Err(e) => {
                            tracing::error!("Stream error for client {}: {}", self.client_id, e);
                            break;
                        }
                    }
                }
                Some(msg) = rx.recv() => {
                    if let Err(e) = writer.write_all(format!("{}\n", msg).as_bytes()).await {
                        tracing::error!("Write error: {}", e);
                        break;
                    }
                }
                else => break,
            }
        }

        self.cleanup().await;
        Ok(())
    }

    async fn handle_line(&self, line: &str) -> Result<()> {
        if line.is_empty() {
            return Ok(());
        }

        if !self.rate_limiter.check_rate(self.client_id).await {
            return Err(anyhow!("Rate limit exceeded"));
        }

        let parsed = self.parser.parse_line(line).map_err(|e| anyhow!(e))?;

        match parsed {
            ParsedMessage::Command(cmd) => {
                self.handle_command(cmd).await?;
            }
            ParsedMessage::Chat(content) => {
                self.handle_chat(content).await?;
            }
        }

        Ok(())
    }

    async fn handle_command(&self, cmd: CommandType) -> Result<()> {
        match cmd {
            CommandType::Name(new_name) => {
                let sanitized = self.sanitizer.sanitize_username(&new_name).map_err(|e| anyhow!(e))?;
                let old = self.state.get_username(self.client_id).await;
                self.state.update_username(self.client_id, sanitized.clone()).await;
                let msg = ChatMessage::system_message(format!("{} is now known as {}", old, sanitized));
                self.message_bus.broadcast(msg).await?;
            }
            CommandType::Quit => {
                return Err(anyhow!("QUIT"));
            }
            CommandType::Users => {
                let users = self.state.get_all_usernames().await;
                let list = users.join(", ");
                let msg = ChatMessage::system_message(format!("Online users: {}", list));
                self.message_bus.broadcast(msg).await?;
            }
            CommandType::Unknown(_) => {
                return Err(anyhow!("Unknown command"));
            }
        }
        Ok(())
    }

    async fn handle_chat(&self, content: String) -> Result<()> {
        let msg = ChatMessage::new(self.username.clone(), content);
        self.message_bus.broadcast(msg.format()).await?;
        Ok(())
    }

    async fn cleanup(&self) {
        self.state.remove_client(self.client_id).await;
        self.message_bus.broadcast(ChatMessage::system_message(
            format!("{} left the chat", self.username)
        )).await.ok();
    }
}