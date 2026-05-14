use crate::state::ClientRegistry;
use std::sync::Arc;

pub struct CommandHandler {
    state: Arc<ClientRegistry>,
}

impl CommandHandler {
    pub fn new(state: Arc<ClientRegistry>) -> Self {
        Self { state }
    }

    pub async fn handle_name_change(&self, client_id: usize, new_name: String) -> Result<String, String> {
        if new_name.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        if new_name.len() > 32 {
            return Err("Username too long (max 32 chars)".to_string());
        }
        let old = self.state.get_username(client_id).await;
        self.state.update_username(client_id, new_name.clone()).await;
        Ok(format!("Username changed from '{}' to '{}'", old, new_name))
    }

    pub async fn handle_quit(&self, client_id: usize) -> Result<String, String> {
        let username = self.state.get_username(client_id).await;
        self.state.remove_client(client_id).await;
        Ok(format!("Goodbye, {}!", username))
    }

    pub async fn handle_users_list(&self) -> Result<String, String> {
        let users = self.state.get_all_usernames().await;
        Ok(format!("Online users ({}): {}", users.len(), users.join(", ")))
    }
}