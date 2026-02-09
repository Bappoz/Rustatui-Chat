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

        let mut reader = BufReader::new(read_half);
        
        // Aguarda um pouco para servidor enviar prompt
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Envia username
        let mut w = writer.lock().await;
        w.write_all(format!("{}\n", username).as_bytes()).await?;
        w.flush().await?;
        drop(w);

        // Lê linhas de validação do servidor
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        
        let mut validation_lines = Vec::new();
        
        // Tenta ler até 5 linhas ou até timeout
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

        // Verifica se nome foi rejeitado
        for line in &validation_lines {
            if line.contains("The name is taken") {
                return Err("Username already taken. Please choose another name.".into());
            }
            if line.contains("must be between") {
                return Err("Username must be between 2-20 characters.".into());
            }
        }

        // Nome aceito! Spawn receiver
        let username_clone = username.clone();
        tokio::spawn(async move {
            Self::receive_messages_with_reader(reader, message_tx, username_clone).await;
        });

        Ok(Self {
            writer,
            message_rx,
        })
    }

    async fn receive_messages_with_reader(
        mut reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
        tx: mpsc::UnboundedSender<ChatMessage>,
        current_room: String,
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

                    // Ignore prompts
                    if clean_line.is_empty()
                        || clean_line.starts_with('>')
                        || clean_line.contains("Enter your nickname")
                        || clean_line.contains("Welcome")
                        || clean_line.contains("Type /help")
                        || clean_line.contains("Available commands") {
                        continue;
                    }

                    // Parse and send message
                    if let Ok(message) = Self::parse_message(&clean_line, &current_room) {
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


    fn parse_message(line: &str, room: &str) -> Result<ChatMessage, Box<dyn std::error::Error>> {
        let line = line.trim();

        // Ignora linhas vazias
        if line.is_empty() {
            return Err("Empty line".into());
        }

        // Ignora apenas prompts específicos de conexão
        if line.contains("Name (press Enter for") 
            || line.contains("Welcome to Rusty Chat")
            || line.contains("Type /help for commands")
            || line.starts_with("Name must be")
            || line.starts_with("The name is taken")
            || line.starts_with("Command executed")
            || line.starts_with("Whisper sent")
            || line.starts_with("Joined room:")
            || line.starts_with("Welcome,")
            || line.contains("joined")
            || line.contains("Returned to")
        {
            return Err("System prompt".into());
        }

        // Mensagens do sistema [SYSTEM]
        if line.starts_with("[SYSTEM]") {
            return Ok(ChatMessage {
                content: line.strip_prefix("[SYSTEM]").unwrap_or(line).trim().to_string(),
                sender_addr: "0.0.0.0:0".parse()?,
                sender_name: "System".to_string(),
                room: room.to_string(),
                message_type: chat_core::message::chat_message::MessageType::System,
                target: None,
                color: "#FFA500".to_string(),
                timestamp: chrono::Utc::now(),
            });
        }

        // Whispers [Whisper from ...]
        if line.starts_with("[Whisper from") {
            if let Some(close_bracket) = line.find(']') {
                let sender = line[14..close_bracket].to_string();
                let content = line[close_bracket + 1..].trim().to_string();
                return Ok(ChatMessage {
                    content,
                    sender_addr: "0.0.0.0:0".parse()?,
                    sender_name: sender,
                    room: room.to_string(),
                    message_type: chat_core::message::chat_message::MessageType::Whisper,
                    target: None,
                    color: "#FF00FF".to_string(),
                    timestamp: chrono::Utc::now(),
                });
            }
        }

        // Mensagens de chat normais (formato: "username: message")
        if let Some(colon_pos) = line.find(':') {
            if colon_pos > 0 && colon_pos < line.len() - 1 {
                let sender = line[..colon_pos].trim().to_string();
                let content = line[colon_pos + 1..].trim().to_string();
                
                // Aceita qualquer mensagem que tenha o formato correto
                if !sender.is_empty() && !content.is_empty() {
                    return Ok(ChatMessage {
                        content,
                        sender_addr: "0.0.0.0:0".parse()?,
                        sender_name: sender,
                        room: room.to_string(),
                        message_type: chat_core::message::chat_message::MessageType::Chat,
                        target: None,
                        color: "#00FFFF".to_string(),
                        timestamp: chrono::Utc::now(),
                    });
                }
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