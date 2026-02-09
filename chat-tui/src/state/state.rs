use chat_core::message::chat_message::ChatMessage;


#[derive(Debug, Clone, PartialEq)]
pub enum AppPage {
    Connection,
    Chat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FocusedField {
    ServerAddress,
    Username,
    ConnectButton,
    MessageInput,
    MessageList,
}

pub struct AppState {
    pub current_page: AppPage,
    pub should_quit: bool,

    pub server_address: String,
    pub username: String,
    pub connection_status: ConnectionStatus,

    pub messages: Vec<ChatMessage>,
    pub message_input: String,
    pub input_mode: InputMode,
    pub scroll_offset: usize,

    pub available_rooms: Vec<String>,
    pub current_room: Option<String>,
    pub users_in_room: Vec<String>,

    pub focused_field: FocusedField,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_page: AppPage::Connection,
            should_quit: false,
            server_address: String::from("127.0.0.1:4556"),
            username: String::new(),
            connection_status: ConnectionStatus::Disconnected,
            messages: Vec::new(),
            message_input: String::new(),
            input_mode: InputMode::Normal,
            scroll_offset: 0,
            available_rooms: vec![
                "general".to_string(),
                "random".to_string(),
                "tech".to_string(),
            ],
            current_room: Some("general".to_string()),
            users_in_room: vec!["alice".to_string(), "bob".to_string()],
            focused_field: FocusedField::ServerAddress,
        }
    }
}


impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connected)
    }

    pub fn can_send_message(&self) -> bool {
        self.is_connected() && !self.message_input.trim().is_empty()
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        self.messages.push(msg)
    }

    pub fn clear_input(&mut self) {
        self.message_input.clear()
    }

    pub fn change_room(&mut self, room: String) {
        if self.available_rooms.contains(&room) {
            self.current_room = Some(room);
        }
    }

    pub fn next_room(&mut self) {
        if let Some(curr) = &self.current_room {
            if let Some(idx) = self.available_rooms.iter().position(|r| r == curr) {
                let next_idx = (idx + 1) % self.available_rooms.len();
                self.current_room = Some(self.available_rooms[next_idx].clone());
            }
        } else if !self.available_rooms.is_empty() {
            self.current_room = Some(self.available_rooms[0].clone());
        }
    }

    pub fn previous_room(&mut self) {
        if let Some(curr) = &self.current_room {
            if let Some(idx) = self.available_rooms.iter().position(|r| r == curr) {
                let prev_index = if idx == 0 {
                    self.available_rooms.len() - 1
                } else {
                    idx - 1
                };
                self.current_room = Some(self.available_rooms[prev_index].clone())
            }
        }
    }


}