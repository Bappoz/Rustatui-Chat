use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use clap::builder::Str;
use tokio::sync::RwLock;
use crate::server::room::Room;


pub type RoomMap = Arc<RwLock<HashMap<String, Room>>>;

#[derive(Clone)]
pub struct RoomManager {
    rooms: RoomMap
}

impl RoomManager {
    pub fn new() -> Self {
        let mut rooms = HashMap::new();
        rooms.insert(
            "general".to_string(),
            Room::new("general".to_string(), None)
        );

        Self {
            rooms: Arc::new(RwLock::new(rooms)),
        }
    }

    pub async fn create_room(
        &self,
        name: String,
        password: Option<String>
    ) -> Result<(), String> {
        let mut rooms = self.rooms.write().await;

        if rooms.contains_key(&name) {
            return Err(format!("Room '{}' already exists", name));
        }
        rooms.insert(name.clone(), Room::new(name, password));
        Ok(())
    }

    pub async fn join_room(
        &self,
        room_name: &str,
        addr: SocketAddr,
        password: Option<&str>,
    ) -> Result<(), String> {
        let mut rooms = self.rooms.write().await;
        let room = rooms.get_mut(room_name)
            .ok_or_else(|| format!("Room '{}' does not exist", room_name))?;

        // Verify if password is necessary
        if room.is_password_protected() {
            match password {
                Some(pwd) if room.verify_password(pwd) => {},
                Some(_) => return Err("Incorret password!".to_string()),
                None => return Err("Room requires a password!".to_string()),
            }
        }

        room.add_member(addr);
        Ok(())
    }

    pub async fn leave_room(&self, room_name: &str, addr: &SocketAddr) {
        let mut rooms = self.rooms.write().await;
        if let Some(room) = rooms.get_mut(room_name) {
            room.remove_member(addr);
        }
    }

    pub async fn get_room_members(&self, room_name: &str) -> Vec<SocketAddr> {
        let rooms = self.rooms.read().await;
        rooms.get(room_name)
            .map(|room| room.members.clone())
            .unwrap_or_default()
    }

    pub async fn list_rooms(&self) -> Vec<(String, usize, bool)> {
        let rooms = self.rooms.read().await;
        rooms.values()
            .map(|room| (
                room.name.clone(),
                room.members.len(),
                room.is_password_protected(),
            ))
            .collect()
    }

    pub async fn get_user_room(&self, addr: &SocketAddr) -> Option<String> {
        let rooms = self.rooms.read().await;
        for (name, room) in rooms.iter() {
            if room.members.contains(addr){
               return Some(name.clone());
            }
        }
        None
    }
}