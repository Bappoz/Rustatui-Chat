use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::client::client_manager::ClientManager;
use crate::message::chat_message::ChatMessage;
use crate::message::chat_message::MessageType;
use crate::message::command_processor::{CommandProcessor, CommandResult};
use crate::server::room_manager::RoomManager;
use crate::utils::color::Colors;
use crate::utils::formatter::Formatter;

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

                break;
            }
            writer.write_all(b"The name is taken. Choose another name.\n").await?;
        }
        Ok(())
    }

    async fn message_loop_static(
        ctx: MessageLoopContext,
        message_receiver: &mut Receiver<ChatMessage>,
        buf_reader: &mut BufReader<tokio::net::tcp::ReadHalf<'_>>,
        writer: &mut tokio::net::tcp::WriteHalf<'_>,
        input: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        const TERMINAL_WIDTH: usize = 80;

        loop {
            select! {
                result = buf_reader.read_line(input) => {
                    match result {
                        Ok(0) => break,
                        Ok(_) => {
                            let message = input.trim().to_string();
                            if !message.is_empty() {
                                if let Some(cmd_result) = CommandProcessor::parse(&message) {
                                    writer.write_all(b"\x1b[1A\x1b[2K").await?; // Move cursor up + limpa linha

                                    let is_quit = matches!(cmd_result, CommandResult::Quit);

                                    match CommandProcessor::execute(
                                        cmd_result,
                                        ctx.addr,
                                        &ctx.client_manager,
                                        &ctx.room_manager,
                                    ).await {
                                        Ok(Some(whisper_msg)) => {
                                            let _ = ctx.message_sender.send(whisper_msg);
                                            let success = Colors::colorize("✓ Whisper sent", Colors::SUCCESS);
                                            writer.write_all(format!("{}\n", success).as_bytes()).await?;
                                        }
                                        Ok(None) => {
                                            if !is_quit {
                                                let success = Colors::colorize("✓ Command executed", Colors::SUCCESS);
                                                writer.write_all(format!("{}\n", success).as_bytes()).await?;
                                            }
                                        }
                                        Err(msg) => {
                                            // Error/info message
                                            let color = if msg.starts_with("✓") {
                                                Colors::SUCCESS
                                            } else if msg.starts_with("Available") || msg.starts_with("Users") {
                                                Colors::INFO
                                            } else if msg.contains("joined") || msg.contains("created") || msg.contains("Returned"){
                                                Colors::SUCCESS
                                            } else if msg.starts_with("═══") {
                                                Colors::BRIGHT_YELLOW
                                            } else {
                                                Colors::ERROR
                                            };
                                            let colored_msg = Colors::colorize(&msg, color);
                                            writer.write_all(format!("{}\n", colored_msg).as_bytes()).await?;
                                        }
                                    }

                                    if is_quit {
                                        break;
                                    }
                                } else {
                                    // Clean de telnet input
                                    writer.write_all(b"\x1b[1A\x1b[2K").await?;

                                    // Normal message
                                    if let Some(sender_name) = ctx.client_manager.get_clients_name(&ctx.addr).await {
                                        if let Some(room) = ctx.room_manager.get_user_room(&ctx.addr).await {
                                            let own_msg = Formatter::format_own_message(&message, TERMINAL_WIDTH);
                                            let colored_own = Colors::colorize(&own_msg, Colors::BRIGHT_CYAN);
                                            writer.write_all(format!("{}\n", colored_own).as_bytes()).await?;

                                            // Message to others
                                            let chat_msg = ChatMessage::new(
                                                message,
                                                ctx.addr,
                                                sender_name,
                                                room,
                                            );
                                            let _ = ctx.message_sender.send(chat_msg);
                                        }
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
                        MessageType::Whisper => {
                            // Verify the receiver
                            if let Some(target) = chat_msg.target {
                                if target == ctx.addr {
                                    let whisper_text = format!("[Whisper from {}] {}",
                                        chat_msg.sender_name, chat_msg.content);
                                    let colored = Colors::colorize(&whisper_text, Colors::WHISPER);
                                    writer.write_all(format!("{}\n", colored).as_bytes()).await?;
                                }
                            }
                        }
                        MessageType::System => {
                            let system_text = format!("[SYSTEM] {}", chat_msg.content);
                            let colored = Colors::colorize(&system_text, Colors::SYSTEM);
                            writer.write_all(format!("{}\n", colored).as_bytes()).await?;
                        }
                        MessageType::Chat => {
                            // Only receives from the same room
                            if let Some(my_room) = ctx.room_manager.get_user_room(&ctx.addr).await {
                                if chat_msg.room == my_room && chat_msg.sender_addr != ctx.addr {
                                    // Get users color
                                    if let Some(client_info) = ctx.client_manager.get_client_info(&chat_msg.sender_addr).await {
                                        let user_color = Colors::get_color_by_index(client_info.color_index);
                                        let colored_name = Colors::colorize(&chat_msg.sender_name, user_color);
                                        let fmt_msg = format!("{}: {}\n", colored_name, chat_msg.content);
                                        writer.write_all(fmt_msg.as_bytes()).await?;
                                    }
                                }
                            }
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
            &mut input
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
        }
        self.client_manager.remove_client(&self.addr).await;
    }
}