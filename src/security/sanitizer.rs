pub struct InputSanitizer {
    max_username_length: usize,
    max_message_length: usize,
}

impl InputSanitizer {
    pub fn new(max_username_length: usize, max_message_length: usize) -> Self {
        Self {
            max_username_length,
            max_message_length,
        }
    }

    pub fn sanitize_username(&self, username: &str) -> Result<String, String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        if username.len() > self.max_username_length {
            return Err(format!("Username too long (max {} chars)", self.max_username_length));
        }

        let sanitized: String = username
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect();

        if sanitized.is_empty() {
            return Err("Username contains no valid characters".to_string());
        }

        Ok(sanitized)
    }

    pub fn sanitize_message(&self, message: &str) -> Result<String, String> {
        if message.len() > self.max_message_length {
            return Err(format!("Message too long (max {} chars)", self.max_message_length));
        }

        Ok(message.to_string())
    }

    pub fn validate_command(&self, command: &str) -> Result<(), String> {
        if command.is_empty() {
            return Err("Empty command".to_string());
        }

        if command.len() > self.max_message_length {
            return Err(format!("Command too long (max {} chars)", self.max_message_length));
        }

        if command.contains('\0') {
            return Err("Null bytes not allowed".to_string());
        }

        Ok(())
    }

    pub fn strip_control_chars(s: &str) -> String {
        s.chars().filter(|c| !c.is_control()).collect()
    }
}