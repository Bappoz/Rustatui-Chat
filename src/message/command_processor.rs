use crate::server::room_manager::RoomManager;
use crate::client::client_manager::ClientManager;
use std::net::SocketAddr;

pub enum CommandResult {
    ChangeNick(String),
    JoinRoom(String, Option<String>),
    ListUsers,
    ListRooms,
    Whisper(String, String),            // Target name e message
    Quit,
    Help,
    InvalidCommand(String),
}

pub struct CommandProcessor;

impl CommandProcessor {
    pub fn parse(input: &str) -> Option<CommandResult> {
        let input = input.trim();

        if !input.starts_with('/') {
            return None;
        }

        let parts: Vec<&str> = input[1..].split_whitespace().collect();
        if parts.is_empty() {
            return Some(CommandResult::InvalidCommand("Empty command".to_string()));
        }

        match parts[0] {
            "nick" => {
                if parts.len() < 2 {
                    return Some(CommandResult::InvalidCommand(
                        "Usage: /nick <new_name>".to_string()
                    ));
                }
                Some(CommandResult::ChangeNick(parts[1].to_string()))
            },
            "join" => {
                if parts.len() < 2 {
                    return Some(CommandResult::InvalidCommand(
                        "Usage: /join <room> [password]".to_string()
                    ));
                }
                let room = parts[1].to_string();
                let password = parts.get(2).map(|s| s.to_string());
                Some(CommandResult::JoinRoom(room, password))
            },

            "list" => Some(CommandResult::ListUsers),

            "rooms" => Some(CommandResult::ListRooms),

            "whisper" | "w" => {
                if parts.len() < 3 {
                    return Some(CommandResult::InvalidCommand(
                        "Usage: /w <user> <message>".to_string()
                    ));
                }
                let target = parts[1].to_string();
                let msg = parts[2..].join(" ");
                Some(CommandResult::Whisper(target, msg))
            },

            "quit" | "exit" => Some(CommandResult::Quit),

            "help" | "?" => Some(CommandResult::Help),

            _ => Some(CommandResult::InvalidCommand(
                format!("Unknown command: {}", parts[0])
            )),
        }

    }


    pub async fn execute(
        result: CommandResult,
        addr: SocketAddr,
        client_manager: &ClientManager,
        room_manager: &RoomManager
    ) -> String {
        match result {
            CommandResult::ChangeNick(new_name) => {
                if client_manager.is_name_available(&new_name).await {
                    client_manager.update_client_name(addr, new_name.clone()).await;
                    format!("✓ Your name is now: {}", new_name)
                } else {
                    "✗ Name already taken".to_string()

                }
            },

            CommandResult::JoinRoom(room, password) => {
                let current_room = room_manager.get_user_room(&addr).await;
                if let Some(curr) = &current_room {
                    room_manager.leave_room(curr, &addr).await;
                }
                match room_manager.join_room(&room, addr, password.as_deref()).await {
                    Ok(_) => format!("✓ Joined room: {}", room),
                    Err(e) => format!("✗ {}", e),
                }
            }

            CommandResult::ListUsers => {
                if let Some(room_name) = room_manager.get_user_room(&addr).await {
                    let members = room_manager.get_room_members(&room_name).await;
                    let mut users = Vec::new();

                    for member_addr in members {
                        if let Some(name) = client_manager.get_clients_name(&member_addr).await {
                            users.push(name);
                        }
                    }
                    format!("Users in {}: {}", room_name, users.join(", "))
                } else {
                    "✗ You are not in a room".to_string()
                }
            },

            CommandResult::ListRooms => {
                let rooms = room_manager.list_rooms().await;
                let mut output = String::from("Available rooms:\n");

                for (name, count, protected) in rooms {
                    let lock = if protected {"🔒"} else { "" };
                    output.push_str(&format!("  {} {} ({} users)\n", lock, name, count));
                }
                output
            },

            CommandResult::Whisper(_, _) => {
                "Whisper feature coming soon!".to_string()
            },

            CommandResult::Quit => "Goodbye!".to_string(),

            CommandResult::Help => {
                String::from(
                    "Available commands:\n\
                    /nick <name>        - Change your nickname\n\
                    /join <room> [pwd]  - Join a room\n\
                    /list               - List users in current room\n\
                    /rooms              - List all rooms\n\
                    /whisper <user> <msg> - Send private message\n\
                    /help               - Show this help\n\
                    /quit               - Exit chat"
                )
            },

            CommandResult::InvalidCommand(msg) => {
                format!("✗ {}", msg)
            },

        }
    }
}