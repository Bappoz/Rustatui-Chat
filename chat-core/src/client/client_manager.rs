use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub name: String,
    pub color_index: usize,
}

pub type ClientMap = Arc<RwLock<HashMap<SocketAddr, ClientInfo>>>;

#[derive(Debug, Clone)]
pub struct ClientManager{
    clients: ClientMap,
    color_counter: Arc<RwLock<usize>>,
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            color_counter: Arc::new(RwLock::new(0)),
        }
    }

    pub fn get_clients(&self) -> ClientMap {
        self.clients.clone()
    }

    pub async fn is_name_available(&self, name: &str) -> bool {
        let clients_read = self.clients.read().await;
        clients_read.values().all(|info| info.name != name)
    }

    pub async fn register_client(&self, addr: SocketAddr, name: String) {
        let mut clients_write = self.clients.write().await;
        let mut color_counter = self.color_counter.write().await;

        let client_info = ClientInfo {
            name,
            color_index: *color_counter,
        };

        clients_write.insert(addr, client_info);
        *color_counter += 1;
    }

    pub async fn get_clients_name(&self, addr: &SocketAddr) -> Option<String> {
        let clients_read = self.clients.read().await;
        clients_read.get(addr).map(|info| info.name.clone())
    }

    pub async fn get_client_info(&self, addr: &SocketAddr) -> Option<ClientInfo> {
        let clients_read = self.clients.read().await;
        clients_read.get(addr).cloned()
    }

    pub async fn get_client_by_name(&self, name: &str) -> Option<SocketAddr> {
        let clients_read = self.clients.read().await;
        for (addr, info) in clients_read.iter() {
            if info.name == name {
                return Some(*addr);
            }
        }
        None
    }

    pub async fn remove_client(&self, addr: &SocketAddr) {
        let mut clients_write = self.clients.write().await;
        clients_write.remove(addr);
    }

    pub async fn update_client_name(&self, addr: SocketAddr, new_name: String) {
        let mut clients_write = self.clients.write().await;
        if let Some(client_info) = clients_write.get_mut(&addr) {
            client_info.name = new_name;
        }
    }
}