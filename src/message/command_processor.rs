use crate::server::room_manager::RoomManager;
use crate::client::client_manager::ClientManager;
use std::net::SocketAddr;
use crate::message::chat_message::ChatMessage;

pub enum CommandResult {
    ChangeNick(String),
    JoinRoom(String, Option<String>),
    CreateRoom(String, Option<String>),
    InviteUser(String, String),
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

            "create" => {
                if parts.len() < 2 {
                    return Some(CommandResult::InvalidCommand(
                        "Usage: /create <room_name> [password]".to_string()
                    ));
                }
                let room_name = parts[1].to_string();
                let password = parts.get(2).map(|s| s.to_string());
                Some(CommandResult::CreateRoom(room_name, password))
            },

            "invite" => {
                if parts.len() < 2 {
                    return Some(CommandResult::InvalidCommand(
                        "Usage: /invite <username> <room_name>".to_string()
                    ));
                }
                let username = parts[1].to_string();
                let room_name = parts[2].to_string();
                Some(CommandResult::InviteUser(username, room_name))
            }

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
    ) -> Result<Option<ChatMessage>, String> {
        match result {
            CommandResult::ChangeNick(new_name) => {
                if client_manager.is_name_available(&new_name).await {
                    client_manager.update_client_name(addr, new_name.clone()).await;
                    Ok(None)
                } else {
                    Err("✗ Name already taken".to_string())
                }
            },

            CommandResult::JoinRoom(room, password) => {
                let current_room = room_manager.get_user_room(&addr).await;
                if let Some(curr) = &current_room {
                    room_manager.leave_room(curr, &addr).await;
                }
                match room_manager.join_room(&room, addr, password.as_deref()).await {
                    Ok(_) => Ok(None),
                    Err(e) => Err(e),
                }
            }

            CommandResult::CreateRoom(room_name, password) => {
                match room_manager.create_room(room_name.clone(), password.clone(), addr).await {
                    Ok(_) => {
                        // Automatically enters created room
                        let _ = room_manager.join_room(&room_name, addr, password.as_deref()).await;
                        let msg = if password.is_some() {
                            format!("✓ Room '{}' created (password protected) and joined", room_name)
                        } else {
                            format!("✓ Room '{}' created and joined", room_name)
                        };
                        Err(msg)
                    }
                    Err(e) => Err(format!("x {}", e))
                }
            },

            CommandResult::InviteUser(username, room_name) => {
                let room_info = room_manager.get_room_info(&room_name).await;
                if room_info.is_none() {
                    return Err(format!("✗ Room '{}' does not exist", room_name));
                }

                let (owner_addr, password) = room_info.unwrap();
                // Verify if is the owner who is inviting
                if owner_addr != addr {
                    return Err("✗ Only the owner of this room can invite users".to_string());
                }

                // Search the user
                if let Some(target_addr) = client_manager.get_client_by_name(&username).await {
                    if let Some(sender_name) = client_manager.get_clients_name(&addr).await {
                        let invite_msg = if let Some(pwd) = password {
                            format!(
                                "\n📨 {} invited you to join room '{}'. Password: {}\nUse: /join {} {}\n",
                                sender_name, room_name, pwd, room_name, pwd
                            )
                        } else {
                            format!(
                                "📨 {} invited you to join room '{}'\nUse: /join {}",
                                sender_name, room_name, room_name
                            )
                        };

                        let whisper_msg = ChatMessage::whisper(
                            invite_msg,
                            addr,
                            sender_name,
                            target_addr,
                        );
                        return Ok(Some(whisper_msg));
                    }
                }
                Err(format!("✗ User '{}' not found", username))
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
                    Err(format!("Users in {}: {}", room_name, users.join(", ")))
                } else {
                    Err("You are not in a room".to_string())
                }
            },

            CommandResult::ListRooms => {
                let rooms = room_manager.list_rooms().await;
                let mut output = String::from("Available rooms:\n");

                for (name, count, protected) in rooms {
                    let lock = if protected {"🔒"} else { "" };
                    output.push_str(&format!("  {} {} ({} users)\n", lock, name, count));
                }
                Err(output)
            },

            CommandResult::Whisper(target_name, message) => {
                // Search user by name
                if let Some(target_addr) = client_manager.get_client_by_name(&target_name).await {
                    if let Some(sender_name) = client_manager.get_clients_name(&addr).await {
                        let whisper_msg = ChatMessage::whisper(
                            message,
                            addr,
                            sender_name,
                            target_addr,
                        );
                        return Ok(Some(whisper_msg));
                    }
                }
                return Err(format!("User '{}' not found", target_name));
            },

            CommandResult::Quit => Ok(None),

            CommandResult::Help => Err(
                String::from(
                    "\n\n═══════════════ Available Commands ═══════════════\n\
                    /nick <name>            - Change your nickname\n\
                    /create <room> [pwd]    - Create a new room\n\
                    /join <room> [pwd]      - Join a room\n\
                    /invite <user> <room>   - Invite user to your room\n\
                    /list                   - List users in current room\n\
                    /rooms                  - List all rooms\n\
                    /w <user> <msg>         - Send private message\n\
                    /help                   - Show this help\n\
                    /quit                   - Exit chat\n\
                    ═══════════════════════════════════════════════════\n\n"
                )),

            CommandResult::InvalidCommand(msg) => Err(format!("✗ {}",msg)),
        }
    }
}