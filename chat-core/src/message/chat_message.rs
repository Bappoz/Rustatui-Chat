use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub enum MessageType {
    Chat,
    Whisper,
    System,
    Command,
    UserList,
    RoomList,
    RoomJoin,
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

        use crate::utils::color_manager::ColorGenerator;
        let color = ColorGenerator::generate_user_color(&sender_name);
        Self {
            content, 
            sender_addr,
            sender_name,
            room,
            message_type: MessageType::Chat,
            target: None,
            color,
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
            color: "#808080".to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn whisper(
        content: String,
        sender_addr: SocketAddr,
        sender_name: String,
        target: SocketAddr,
    ) -> Self {

        use crate::utils::color_manager::ColorGenerator;
        let color = ColorGenerator::generate_user_color(&sender_name);

        Self {
            content,
            sender_addr,
            sender_name,
            room: "private".to_string(),
            message_type: MessageType::Whisper,
            target: Some(target),
            color,
            timestamp: Utc::now(),
        }
    }

    pub fn user_list(users: Vec<String>, room: String) -> Self {
        let content = users.join(",");
        Self {
            content,
            sender_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 0),
            sender_name: "SYSTEM".to_string(),
            room,
            message_type: MessageType::UserList,
            target: None,
            color: "#808080".to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn room_list(rooms: Vec<String>) -> Self {
        let content = rooms.join(",");
        Self {
            content,
            sender_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 0),
            sender_name: "SYSTEM".to_string(),
            room: "system".to_string(),
            message_type: MessageType::RoomList,
            target: None,
            color: "#808080".to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn room_joined(room_name: String, addr: SocketAddr) -> Self {
        Self {
            content: room_name.clone(),
            sender_addr: addr,
            sender_name: "SYSTEM".to_string(),
            room: room_name,
            message_type: MessageType::RoomJoin,
            target: Some(addr),
            color: "#808080".to_string(),
            timestamp: Utc::now(),
        }
    }
}