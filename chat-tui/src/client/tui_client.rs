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
    pub action_tx: Option<mpsc::UnboundedSender<crate::state::action::Action>>,
}

impl TuiClient {
    pub async fn connect(
        server_addr: &str,
        username: String,
        action_tx: mpsc::UnboundedSender<crate::state::action::Action>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(server_addr).await?;
        let (read_half, write_half) = stream.into_split();

        let writer = Arc::new(Mutex::new(BufWriter::new(write_half)));
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        let mut reader = BufReader::new(read_half);
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Envia username
        let mut w = writer.lock().await;
        w.write_all(format!("{}\n", username).as_bytes()).await?;
        w.flush().await?;
        drop(w);

        // Read validation lines
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        
        let mut validation_lines = Vec::new();
        
        for _ in 0..5 {
            let mut line = String::new();
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(100),
                reader.read_line(&mut line)
            ).await {
                Ok(Ok(n)) if n > 0 => {
                    let clean = Self::strip_ansi_codes(&line);
                    validation_lines.push(clean);
                }
                _ => break,
            }
        }

        // Check if the name was accepted
        for line in &validation_lines {
            if line.contains("The name is taken") {
                return Err("Username already taken. Please choose another name.".into());
            }
            if line.contains("must be between") {
                return Err("Username must be between 2-20 characters.".into());
            }
        }

        // Accepted name! Spawn receiver
        let username_clone = username.clone();
        let action_tx_clone = action_tx.clone();
        tokio::spawn(async move {
            Self::receive_messages_with_reader(reader, message_tx, username_clone, action_tx_clone).await;
        });

        Ok(Self {
            writer,
            message_rx,
            action_tx: Some(action_tx),
        })
    }

    async fn receive_messages_with_reader(
        mut reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
        tx: mpsc::UnboundedSender<ChatMessage>,
        current_room: String,
        action_tx: mpsc::UnboundedSender<crate::state::action::Action>,
    ) {
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    let trimmed = line.trim();
                    
                    // Remove ANSI color codes
                    let clean_line = Self::strip_ansi_codes(trimmed);

                    // Ignore prompts and empty lines
                    if clean_line.is_empty()
                        || clean_line.starts_with('>')
                        || clean_line.contains("Enter your nickname")
                        || clean_line.contains("Welcome")
                        || clean_line.contains("Type /help")
                        || clean_line.contains("Joined room:")
                        || clean_line.contains("Available commands") {
                        continue;
                    }

                    // Parse structured messages from server
                    if let Ok(message) = Self::parse_structured_message(&clean_line, &current_room, &action_tx) {
                        let _ = tx.send(message);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading: {}", e);
                    break
                },
            }
        }
    }
    
    fn strip_ansi_codes(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Skip ANSI escape sequence
                if chars.peek() == Some(&'[') {
                    chars.next(); // skip '['
                    // Skip until we hit a letter (end of ANSI sequence)
                    while let Some(&next_ch) = chars.peek() {
                        chars.next();
                        if next_ch.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
            } else {
                result.push(ch);
            }
        }
        result
    }

    pub async fn send_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut w = self.writer.lock().await;
        w.write_all(format!("{}\n", message).as_bytes()).await?;
        w.flush().await?;
        Ok(())
    }


    fn parse_structured_message(
        line: &str, 
        room: &str,
        action_tx: &mpsc::UnboundedSender<crate::state::action::Action>,
    ) -> Result<ChatMessage, Box<dyn std::error::Error>> {
        let line = line.trim();

        // Ignore empty lines
        if line.is_empty() {
            return Err("Empty line".into());
        }

        // USER_LIST|user1,user2,user3
        if line.starts_with("USER_LIST|") {
            let users_str = line.strip_prefix("USER_LIST|").unwrap_or("");
            let users: Vec<String> = users_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            // Send action to update user list
            let _ = action_tx.send(crate::state::action::Action::UpdateUserList(users));
            
            // Return a dummy message that will be ignored
            return Err("User list update".into());
        }

        // SYSTEM|content
        if line.starts_with("SYSTEM|") {
            let content = line.strip_prefix("SYSTEM|").unwrap_or(line).trim().to_string();
            return Ok(ChatMessage {
                content,
                sender_addr: "0.0.0.0:0".parse()?,
                sender_name: "System".to_string(),
                room: room.to_string(),
                message_type: chat_core::message::chat_message::MessageType::System,
                target: None,
                color: "#808080".to_string(),
                timestamp: chrono::Utc::now(),
            });
        }

        // CHAT|timestamp|sender|color|content
        if line.starts_with("CHAT|") {
            let parts: Vec<&str> = line.splitn(5, '|').collect();
            if parts.len() == 5 {
                let timestamp_str = parts[1];
                let sender = parts[2].to_string();
                let color = parts[3].to_string();
                let content = parts[4].to_string();

                let timestamp = chrono::DateTime::parse_from_str(
                    timestamp_str, 
                    "%Y-%m-%d %H:%M:%S"
                )
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

                return Ok(ChatMessage {
                    content,
                    sender_addr: "0.0.0.0:0".parse()?,
                    sender_name: sender,
                    room: room.to_string(),
                    message_type: chat_core::message::chat_message::MessageType::Chat,
                    target: None,
                    color,
                    timestamp,
                });
            }
        }

        Err("Unparseable line".into())
    }


    pub async fn change_room(&self, room: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_message(&format!("/join {}", room)).await
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_message("/quit").await
    }
}