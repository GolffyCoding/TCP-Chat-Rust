pub mod config;
pub mod security;
pub mod networking;
pub mod protocol;
pub mod session;
pub mod commands;
pub mod state;
pub mod message_bus;
pub mod logging;
pub mod server;

pub use config::Config;
pub use security::{RateLimiter, InputSanitizer, ProtocolValidator};
pub use networking::TcpListenerWrapper;
pub use protocol::parser::{ParsedMessage, CommandType, ChatMessage, ProtocolParser};
pub use session::{SessionHandler, TimeoutManager};
pub use commands::{CommandRegistry, CommandHandler};
pub use state::ClientRegistry;
pub use message_bus::MessageBus;
pub use server::ChatServer;

pub type MessageResult = Result<String, String>;