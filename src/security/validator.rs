pub struct ProtocolValidator;

impl ProtocolValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_utf8(data: &[u8]) -> Result<&str, String> {
        std::str::from_utf8(data)
            .map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    pub fn validate_line(line: &str) -> Result<(), String> {
        if line.is_empty() {
            return Err("Empty line".to_string());
        }

        if line.len() > 65536 {
            return Err("Line too long".to_string());
        }

        if line.contains('\0') {
            return Err("Null bytes not allowed".to_string());
        }

        Ok(())
    }

    pub fn parse_line(data: &[u8]) -> Result<String, String> {
        let line = Self::validate_utf8(data)?;
        let trimmed = line.trim_end();
        
        if trimmed.is_empty() {
            return Err("Empty line".to_string());
        }

        Ok(trimmed.to_string())
    }

    pub fn is_command(line: &str) -> bool {
        line.starts_with('/')
    }

    pub fn parse_command(line: &str) -> Option<(&str, Option<&str>)> {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        let cmd = parts[0];
        let arg = parts.get(1).copied();
        Some((cmd, arg))
    }
}

impl Default for ProtocolValidator {
    fn default() -> Self {
        Self::new()
    }
}