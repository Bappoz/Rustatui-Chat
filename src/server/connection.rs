use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::client::client_manager::ClientManager;
use crate::message::chat_message::ChatMessage;

struct MessageLoopContext {
    client_manager: ClientManager,
    addr: SocketAddr,
    message_sender: Sender<ChatMessage>,
}

pub struct ClientConnection {
    stream: TcpStream,
    addr: SocketAddr,
    client_manager: ClientManager,
    message_sender: Sender<ChatMessage>,
    message_receiver: Receiver<ChatMessage>,
}

impl ClientConnection {
    pub fn new(
        stream: TcpStream,
        addr: SocketAddr,
        client_manager: ClientManager,
        message_sender: Sender<ChatMessage>,
        message_receiver: Receiver<ChatMessage>,
    ) -> Self {
        Self {
            stream, addr, client_manager, message_sender, message_receiver,
        }
    }

    async fn register_client_name(
        client_manager: &ClientManager,
        addr: SocketAddr,
        buf_reader: &mut BufReader<tokio::net::tcp::ReadHalf<'_>>,
        writer: &mut tokio::net::tcp::WriteHalf<'_>,
        input: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>>{
        loop {
            writer.write_all(b"\nName: ").await?;
            input.clear();
            buf_reader.read_line(input).await?;

            let name = input.trim().to_string();
            if name.is_empty() {
                writer.write_all(b"Name cannot be empty. Try again!").await?;
                continue;
            }

            if client_manager.is_name_available(&name).await {
                client_manager.register_client(addr, name).await;
                break;
            }
            writer.write_all(b"The name is taken. Choose another name.").await?;
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
        loop {
            select! {
                result = buf_reader.read_line(input) => {
                    match result {
                        Ok(0) => break,
                        Ok(_) => {
                            let message = input.trim().to_string();
                            if !message.is_empty() {
                                let chat_msg = ChatMessage::new(message, ctx.addr);
                                let _ = ctx.message_sender.send(chat_msg);
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
                    if chat_msg.sender_addr != ctx.addr {
                        if let Some(sender_name) = ctx.client_manager.get_clients_name(&chat_msg.sender_addr).await {
                            let fmt_msg = format!("{}: {}\n", sender_name, chat_msg.content);
                            writer.write_all(fmt_msg.as_bytes()).await?;
                        }
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

        // Registra nome do cliente
        if let Err(e) = Self::register_client_name(&self.client_manager, self.addr, &mut buf_reader, &mut writer, &mut input).await {
            eprintln!("Error trying to register client: {}", e);
            return;
        }
        input.clear();
        let _ = writer.write_all(b"\n").await;

        // Cria contexto com os campos necessários
        let ctx = MessageLoopContext {
            client_manager: self.client_manager.clone(),
            addr: self.addr,
            message_sender: self.message_sender.clone(),
        };
        let mut message_receiver = self.message_receiver;

        // Loop de mensagens
        if let Err(e) = Self::message_loop_static(ctx, &mut message_receiver, &mut buf_reader, &mut writer, &mut input).await {
            eprintln!("Error in the message loop: {}", e);
        }

        // Cleanup ao desconectar
        self.client_manager.remove_client(&self.addr).await;
    }
}