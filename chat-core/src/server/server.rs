
use tokio::net::TcpListener;
use tokio::sync::broadcast;

use crate::client::client_manager::ClientManager;
use crate::server::server_config::ServerConfig;
use crate::message::chat_message::ChatMessage;
use crate::server::connection::ClientConnection;
use crate::server::room_manager::RoomManager;

pub struct ChatServer {
    config: ServerConfig,
    client_manager: ClientManager,
    room_manager: RoomManager,
}

impl ChatServer {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            client_manager: ClientManager::new(),
            room_manager: RoomManager::new(),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.config.address).await?;
        let (sender, _) = broadcast::channel::<ChatMessage>(self.config.max_clients);

        println!("\n 🦀 Welcome to Rusty Chat");
        println!(" Server listening on {}\n", self.config.address);

        let mut anonymous_counter = 1u32;

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("✅ New connection from: {}", addr);

            let client_manager = self.client_manager.clone();
            let room_manager = self.room_manager.clone();
            let message_sender = sender.clone();
            let message_receiver = sender.subscribe();
            let anon_id = anonymous_counter;
            anonymous_counter += 1;

            tokio::spawn(async move {
                let connection = ClientConnection::new(
                    stream,
                    addr,
                    client_manager,
                    room_manager,
                    message_sender,
                    message_receiver,
                    anon_id,
                );
                connection.handler().await;
                println!("❌ Client disconnected: {}", addr);
            });
        }
    }
}