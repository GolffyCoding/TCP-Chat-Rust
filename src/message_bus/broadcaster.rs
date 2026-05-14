use std::sync::Arc;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::state::ClientRegistry;

#[derive(Clone)]
pub struct MessageBus {
    sender: Sender<String>,
}

impl MessageBus {
    pub fn new(sender: Sender<String>) -> Self {
        Self { sender }
    }

    pub async fn broadcast(&self, message: String) -> anyhow::Result<()> {
        let _ = self.sender.send(message);
        Ok(())
    }

    pub fn subscribe(&self) -> Receiver<String> {
        self.sender.subscribe()
    }

    pub fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }
}

pub struct MessageBroadcaster {
    receiver: Receiver<String>,
}

impl MessageBroadcaster {
    pub fn new(receiver: Receiver<String>) -> Self {
        Self { receiver }
    }

    pub async fn start(&mut self, state: &Arc<ClientRegistry>) -> anyhow::Result<()> {
        while let Ok(message) = self.receiver.recv().await {
            tracing::debug!("Broadcasting: {}", message);
            let clients = state.get_all_clients();
            for client in clients {
                let _ = client.sender.send(message.clone());
            }
        }
        Ok(())
    }
}