use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub sender_addr: SocketAddr,
}

impl ChatMessage {
    pub fn new(content: String, sender_addr: SocketAddr) -> Self {
        Self {
            content, 
            sender_addr,
        }
    }
}