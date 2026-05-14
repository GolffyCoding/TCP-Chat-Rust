use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use tokio::sync::broadcast;
use dashmap::DashMap;

pub struct ClientInfo {
    pub id: usize,
    pub username: String,
    pub sender: broadcast::Sender<String>,
}

impl ClientInfo {
    pub fn new(id: usize, username: String, sender: broadcast::Sender<String>) -> Self {
        Self { id, username, sender }
    }
}

#[derive(Clone)]
pub struct ClientRegistry {
    clients: Arc<DashMap<usize, ClientInfo>>,
    next_client_id: Arc<AtomicUsize>,
    message_sender: broadcast::Sender<String>,
}

impl ClientRegistry {
    pub fn new(message_capacity: usize) -> (Self, broadcast::Receiver<String>) {
        let (tx, rx) = broadcast::channel(message_capacity);
        (
            Self {
                clients: Arc::new(DashMap::new()),
                next_client_id: Arc::new(AtomicUsize::new(1)),
                message_sender: tx,
            },
            rx,
        )
    }

    pub async fn add_client(&self, username: String, sender: broadcast::Sender<String>) -> usize {
        let id = self.next_client_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let client = ClientInfo::new(id, username, sender);
        self.clients.insert(id, client);
        id
    }

    pub async fn remove_client(&self, id: usize) {
        self.clients.remove(&id);
    }

    pub async fn update_username(&self, id: usize, new_username: String) {
        if let Some(mut client) = self.clients.get_mut(&id) {
            client.username = new_username;
        }
    }

    pub async fn get_username(&self, id: usize) -> String {
        self.clients
            .get(&id)
            .map(|c| c.username.clone())
            .unwrap_or_else(|| format!("user_{}", id))
    }

    pub async fn get_all_usernames(&self) -> Vec<String> {
        self.clients
            .iter()
            .map(|entry| entry.username.clone())
            .collect()
    }

    pub fn get_client_count(&self) -> usize {
        self.clients.len()
    }

    pub fn get_message_sender(&self) -> broadcast::Sender<String> {
        self.message_sender.clone()
    }

    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
        self.clients
            .iter()
            .map(|r| ClientInfo {
                id: r.id,
                username: r.username.clone(),
                sender: r.sender.clone(),
            })
            .collect()
    }
}