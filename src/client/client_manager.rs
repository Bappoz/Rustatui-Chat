use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

pub type ClientMap = Arc<RwLock<HashMap<SocketAddr, String>>>;

#[derive(Debug, Clone)]
pub struct ClientManager{
    clients: ClientMap,
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_clients(&self) -> ClientMap {
        self.clients.clone()
    }

    pub async fn is_name_available(&self, name: &str) -> bool {
        let clients_read = self.clients.read().await;
        clients_read.values().all(|existing_name| existing_name != name) 
    }

    pub async fn register_client(&self, addr: SocketAddr, name: String) {
        let mut clients_write = self.clients.write().await;
        clients_write.insert(addr, name);
    }

    pub async fn get_clients_name(&self, addr: &SocketAddr) -> Option<String> {
        let clients_read = self.clients.read().await;
        clients_read.get(addr).cloned()
    }

    pub async fn remove_client(&self, addr: &SocketAddr) {
        let mut clients_write = self.clients.write().await;
        clients_write.remove(addr);
    }
}