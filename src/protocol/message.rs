use chrono::DateTime;
use chrono::Local;

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Name(String),
    Quit,
    Users,
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: DateTime<Local>,
}

impl ChatMessage {
    pub fn new(sender: String, content: String) -> Self {
        Self {
            sender,
            content,
            timestamp: Local::now(),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "[{}] {}: {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.sender,
            self.content
        )
    }

    pub fn system_message(content: String) -> String {
        format!(
            "[{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            content
        )
    }
}

#[derive(Debug, Clone)]
pub enum ParsedMessage {
    Command(CommandType),
    Chat(String),
}