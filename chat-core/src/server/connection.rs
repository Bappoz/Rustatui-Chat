use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::client::client_manager::ClientManager;
use crate::message::chat_message::ChatMessage;
use crate::message::chat_message::MessageType;
use crate::server::room_manager::RoomManager;

struct MessageLoopContext {
    client_manager: ClientManager,
    room_manager: RoomManager,
    addr: SocketAddr,
    message_sender: Sender<ChatMessage>,
}

pub struct ClientConnection {
    stream: TcpStream,
    addr: SocketAddr,
    client_manager: ClientManager,
    room_manager: RoomManager,
    message_sender: Sender<ChatMessage>,
    message_receiver: Receiver<ChatMessage>,
    anonymous_id: u32,
}

impl ClientConnection {
    pub fn new(
        stream: TcpStream,
        addr: SocketAddr,
        client_manager: ClientManager,
        room_manager: RoomManager,
        message_sender: Sender<ChatMessage>,
        message_receiver: Receiver<ChatMessage>,
        anonymous_id: u32,
    ) -> Self {
        Self {
            stream, addr, client_manager, room_manager, message_sender, message_receiver, anonymous_id
        }
    }

    async fn register_client_name(
        client_manager: &ClientManager,
        room_manager: &RoomManager,
        addr: SocketAddr,
        anonymous_id: u32,
        buf_reader: &mut BufReader<tokio::net::tcp::ReadHalf<'_>>,
        writer: &mut tokio::net::tcp::WriteHalf<'_>,
        input: &mut String,
        message_sender: &Sender<ChatMessage>,
    ) -> Result<(), Box<dyn std::error::Error>>{
        writer.write_all(b"\n=== Welcome to Rusty Chat ===\n").await?;
        writer.write_all(b"Type /help for commands\n\n").await?;

        loop {
            writer.write_all(format!("Name (press Enter for Anonymous#{}): ", anonymous_id).as_bytes()).await?;

            input.clear();
            buf_reader.read_line(input).await?;

            let name = input.trim().to_string();

            let final_name = if name.is_empty() {
                format!("Anonymous#{}", anonymous_id)
            } else {
                name
            };

            if final_name.len() < 2 || final_name.len() > 20 {
                writer.write_all(b"Name must be between 2-20 characters. Try again.\n").await?;
                continue;
            }

            if client_manager.is_name_available(&final_name).await {
                client_manager.register_client(addr, final_name.clone()).await;
                let _ = room_manager.join_room("general", addr, None).await;

                writer.write_all(format!("✓ Welcome, {}!\n", final_name).as_bytes()).await?;
                writer.write_all(b" Joined room: general\n\n").await?;

                // Broadcast user list to all users in the room
                Self::broadcast_user_list(room_manager, client_manager, "general", message_sender).await;

                break;
            }
            writer.write_all(b"The name is taken. Choose another name.\n").await?;
        }
        Ok(())
    }

    async fn broadcast_user_list(
        room_manager: &RoomManager,
        client_manager: &ClientManager,
        room_name: &str,
        message_sender: &Sender<ChatMessage>,
    ) {
        let member_addrs = room_manager.get_room_members(room_name).await;
        let mut usernames = Vec::new();
        
        for addr in member_addrs {
            if let Some(username) = client_manager.get_clients_name(&addr).await {
                usernames.push(username);
            }
        }

        let user_list_msg = ChatMessage::user_list(usernames, room_name.to_string());
        let _ = message_sender.send(user_list_msg);
    }

    async fn message_loop_static(
        ctx: MessageLoopContext,
        message_receiver: &mut Receiver<ChatMessage>,
        buf_reader: &mut BufReader<tokio::net::tcp::ReadHalf<'_>>,
        writer: &mut tokio::net::tcp::WriteHalf<'_>,
        input: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            select! {
                result = buf_reader.read_line(input) => {
                    match result {
                        Ok(0) => break,
                        Ok(_) => {
                            let message = input.trim().to_string();
                            if !message.is_empty() {
                                // Normal message (TUI-focused)
                                if let Some(sender_name) = ctx.client_manager.get_clients_name(&ctx.addr).await {
                                    if let Some(room) = ctx.room_manager.get_user_room(&ctx.addr).await {
                                        // Message to others
                                        let chat_msg = ChatMessage::new(
                                            message.clone(),
                                            ctx.addr,
                                            sender_name.clone(),
                                            room,
                                        );
                                        let _ = ctx.message_sender.send(chat_msg);
                                    }
                                }
                            }
                            input.clear();
                        },

                        Err(e) => {
                            eprintln!("Error trying to read message: {}", e);
                            break;
                        }
                    }
                },
                Ok(chat_msg) = message_receiver.recv() => {
                    match chat_msg.message_type {
                        MessageType::UserList => {
                            // Send user list to TUI client
                            if let Some(my_room) = ctx.room_manager.get_user_room(&ctx.addr).await {
                                if chat_msg.room == my_room {
                                    writer.write_all(format!("{}\n", chat_msg.content).as_bytes()).await?;
                                }
                            }
                        }
                        MessageType::Chat => {
                            // Only receives from the same room
                            if let Some(my_room) = ctx.room_manager.get_user_room(&ctx.addr).await {
                                if chat_msg.room == my_room {
                                    // Send structured message: "CHAT|<timestamp>|<sender>|<color>|<content>"
                                    let formatted = format!("CHAT|{}|{}|{}|{}\n",
                                        chat_msg.timestamp.format("%Y-%m-%d %H:%M:%S"),
                                        chat_msg.sender_name,
                                        chat_msg.color,
                                        chat_msg.content
                                    );
                                    writer.write_all(formatted.as_bytes()).await?;
                                }
                            }
                        }
                        MessageType::System => {
                            writer.write_all(format!("SYSTEM|{}\n", chat_msg.content).as_bytes()).await?;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }


    pub async fn handler(mut self) {
        let (reader, mut writer) = self.stream.split();
        let mut buf_reader = BufReader::new(reader);
        let mut input = String::new();

        // Register the client
        if let Err(e) = Self::register_client_name(
            &self.client_manager,
            &self.room_manager,
            self.addr,
            self.anonymous_id,
            &mut buf_reader,
            &mut writer,
            &mut input,
            &self.message_sender,
        ).await {
            eprintln!("Error trying to register client: {}", e);
            return;
        }
        input.clear();

        let _ = writer.write_all(b"\n").await;

        // Create context with the right fields
        let ctx = MessageLoopContext {
            client_manager: self.client_manager.clone(),
            room_manager: self.room_manager.clone(),
            addr: self.addr,
            message_sender: self.message_sender.clone(),
        };
        let mut message_receiver = self.message_receiver;

        // Message loop
        if let Err(e) = Self::message_loop_static(
            ctx,
            &mut message_receiver,
            &mut buf_reader,
            &mut writer,
            &mut input
        ).await {
            eprintln!("Error in the message loop: {}", e);
        }

        // Cleanup
        if let Some(room) = self.room_manager.get_user_room(&self.addr).await {
            self.room_manager.leave_room(&room, &self.addr).await;
            self.client_manager.remove_client(&self.addr).await;
            
            // Broadcast updated user list
            Self::broadcast_user_list(&self.room_manager, &self.client_manager, &room, &self.message_sender).await;
        }
    }
}