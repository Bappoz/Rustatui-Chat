use std::sync::Arc;
use tokio::{
    net::TcpStream,
    sync::{Mutex, mpsc},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter}
};
use chat_core::message::chat_message::ChatMessage;

pub struct TuiClient {
    writer: Arc<Mutex<BufWriter<tokio::net::tcp::OwnedWriteHalf>>>,
    pub message_rx: mpsc::UnboundedReceiver<ChatMessage>,
}

impl TuiClient {
    pub async fn connect(
        server_addr: &str,
        username: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(server_addr).await?;
        let (read_half, write_half) = stream.into_split();

        let writer = Arc::new(Mutex::new(BufWriter::new(write_half)));

        let (message_tx, message_rx) = mpsc::unbounded_channel();

        let username_clone = username.clone();
        tokio::spawn(async move {
            Self::receive_messages(read_half, message_tx, username_clone).await;
        });

        let mut w = writer.lock().await;
        w.write_all(format!("/nick {}\n", username).as_bytes()).await?;
        w.flush().await?;
        drop(w);

        Ok(Self {
            writer,
            message_rx,
        })
    }

    async fn receive_messages(
        read_half: tokio::net::tcp::OwnedReadHalf,
        tx: mpsc::UnboundedSender<ChatMessage>,
        current_room: String,
    ) {
        let mut reader = BufReader::new(read_half);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    if let Ok(message) = Self::parse_message(line.trim(), &current_room) {
                        let _ = tx.send(message);
                    }
                }
                Err(_) => break,    
            }
        }
    }

    fn parse_message(line: &str, room: &str) -> Result<ChatMessage, Box<dyn std::error::Error>> {
        // TODO: Implement real parsing
        Ok(ChatMessage {
            content: line.to_string(),
            sender_addr: "0.0.0.0:0".parse()?,
            sender_name: "Server".to_string(),
            room: room.to_string(),
            message_type: chat_core::message::chat_message::MessageType::Chat,
            target: None,
            color: "#FFFFFF".to_string(),
            timestamp: chrono::Utc::now(),
        })
    }

    pub async fn send_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut w = self.writer.lock().await;
        w.write_all(format!("{}\n", message).as_bytes()).await?;
        w.flush().await?;
        Ok(())
    }

    pub async fn change_room(&self, room: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_message(&format!("/join {}", room)).await
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_message("/quit").await
    }
}