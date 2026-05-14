use crate::state::ClientRegistry;
use std::sync::Arc;

pub struct CommandRegistry {
    #[allow(dead_code)]
    state: Arc<ClientRegistry>,
}

impl CommandRegistry {
    pub fn new(state: Arc<ClientRegistry>) -> Self {
        Self { state }
    }
}