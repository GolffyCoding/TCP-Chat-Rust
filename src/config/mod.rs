use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_max_connections")]
    pub max_connections: usize,
    #[serde(default = "default_max_message_size")]
    pub max_message_size: usize,
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout_secs: u64,
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout_secs: u64,
    #[serde(default = "default_rate_limit")]
    pub rate_limit_msgs: usize,
    #[serde(default = "default_rate_limit_window")]
    pub rate_limit_window_secs: u64,
    #[serde(default = "default_max_username_length")]
    pub max_username_length: usize,
    #[serde(default = "default_max_command_length")]
    pub max_command_length: usize,
    #[serde(default = "default_backlog")]
    pub backlog: usize,
    #[serde(default = "default_message_buffer_size")]
    pub message_buffer_size: usize,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_max_connections() -> usize {
    10_000
}

fn default_max_message_size() -> usize {
    4096
}

fn default_idle_timeout() -> u64 {
    300
}

fn default_connection_timeout() -> u64 {
    10
}

fn default_rate_limit() -> usize {
    100
}

fn default_rate_limit_window() -> u64 {
    1
}

fn default_max_username_length() -> usize {
    32
}

fn default_max_command_length() -> usize {
    512
}

fn default_backlog() -> usize {
    1024
}

fn default_message_buffer_size() -> usize {
    65536
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: default_host(),
            port: default_port(),
            max_connections: default_max_connections(),
            max_message_size: default_max_message_size(),
            idle_timeout_secs: default_idle_timeout(),
            connection_timeout_secs: default_connection_timeout(),
            rate_limit_msgs: default_rate_limit(),
            rate_limit_window_secs: default_rate_limit_window(),
            max_username_length: default_max_username_length(),
            max_command_length: default_max_command_length(),
            backlog: default_backlog(),
            message_buffer_size: default_message_buffer_size(),
        }
    }
}