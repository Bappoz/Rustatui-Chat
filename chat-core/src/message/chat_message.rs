use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub enum MessageType {
    Chat,
    Whisper,
    System,
    Command,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub sender_addr: SocketAddr,
    pub sender_name: String,
    pub room: String,
    pub message_type: MessageType,
    pub target: Option<SocketAddr>,
    pub color: String,
    pub timestamp: DateTime<Utc>,
}

impl ChatMessage {
    pub fn new(
        content: String,
        sender_addr: SocketAddr,
        sender_name: String,
        room: String,
    ) -> Self {
        Self {
            content, 
            sender_addr,
            sender_name,
            room,
            message_type: MessageType::Chat,
            target: None,
            color: "white".to_string(),
            timestamp: Utc::now()
        }
    }

    pub fn system(content: String, room: String) -> Self {
        Self {
            content,
            sender_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 0),
            sender_name: "SYSTEM".to_string(),
            room,
            message_type: MessageType::System,
            target: None,
            color: "yellow".to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn whisper(
        content: String,
        sender_addr: SocketAddr,
        sender_name: String,
        target: SocketAddr
    ) -> Self {
        Self {
            content,
            sender_addr,
            sender_name,
            room: String::new(),
            message_type: MessageType::Whisper,
            target: Some(target),
            color: "magenta".to_string(),
            timestamp: Utc::now(),
        }
    }


}