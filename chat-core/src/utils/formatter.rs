use crate::utils::{color_manager::ColorGenerator, color::Colors};
use crate::message::chat_message::{MessageType, ChatMessage};

pub struct Formatter;

impl Formatter {
    /// Formata mensagem própria (You: message)
    pub fn format_own_message(content: &str, _terminal_width: usize) -> String {
        format!("You: {}", content)
    }
}

pub fn format_message(msg: &ChatMessage) -> String {
    let timestamp = msg.timestamp.format("%H:%M:%S");
    let user_color_ansi = ColorGenerator::hex_to_ansi(&msg.color);

    match msg.message_type {
        MessageType::Chat => {
            format!(
                "{}[{}]{} {}{}{}: {}\n",
                Colors::BOLD,
                timestamp,
                Colors::RESET,
                user_color_ansi,
                msg.sender_name,
                Colors::RESET,
                msg.content
            )
        }
        MessageType::System => {
            format!(
                "{}[SYSTEM] {}{}\n",
                Colors::SYSTEM,
                msg.content,
                Colors::RESET
            )
        }
        MessageType::Whisper => {
            format!(
                "{}[Whisper from {}] {}{}\n",
                Colors::WHISPER,
                msg.sender_name,
                msg.content,
                Colors::RESET
            )
        }
        MessageType::Command => {
            format!(
                "{}[Command] {}{}\n",
                Colors::INFO,
                msg.content,
                Colors::RESET
            )
        }
        MessageType::UserList => {
            // User list is handled separately by the client
            String::new()
        }

        MessageType::RoomList => {
            String::new()
        }
        MessageType::RoomJoin => {
            String::new()
        }
    }
}

pub fn format_user_list(users: &[String]) -> String {
    let mut output = format!("{}Users in this room:{}\n", Colors::INFO, Colors::RESET);
    for user in users {
        // Generate color for each user
        let user_color_hex = ColorGenerator::generate_user_color(user);
        let user_color_ansi = ColorGenerator::hex_to_ansi(&user_color_hex);

        output.push_str(&format!("  {}{}{}\n", user_color_ansi, user, Colors::RESET));
    }
    output
}