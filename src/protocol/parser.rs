use crate::security::ProtocolValidator;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Name(String),
    Quit,
    Users,
    Unknown(String),
}

impl FromStr for CommandType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ' ').collect();
        let cmd = parts[0];
        let arg = parts.get(1).map(|s| s.to_string()).unwrap_or_default();

        match cmd {
            "/name" => {
                if arg.is_empty() {
                    Err("Usage: /name <username>".to_string())
                } else {
                    Ok(CommandType::Name(arg))
                }
            }
            "/quit" => Ok(CommandType::Quit),
            "/users" => Ok(CommandType::Users),
            _ => Err(format!("Unknown command: {}", cmd)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
}

impl ChatMessage {
    pub fn new(sender: String, content: String) -> Self {
        Self {
            sender,
            content,
            timestamp: chrono::Local::now(),
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
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            content
        )
    }
}

#[derive(Debug, Clone)]
pub enum ParsedMessage {
    Command(CommandType),
    Chat(String),
}

pub struct ProtocolParser;

impl ProtocolParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_line(&self, line: &str) -> Result<ParsedMessage, String> {
        ProtocolValidator::validate_line(line)?;

        if ProtocolValidator::is_command(line) {
            let cmd = CommandType::from_str(line).map_err(|e| e.to_string())?;
            Ok(ParsedMessage::Command(cmd))
        } else {
            Ok(ParsedMessage::Chat(line.to_string()))
        }
    }
}

impl Default for ProtocolParser {
    fn default() -> Self {
        Self::new()
    }
}