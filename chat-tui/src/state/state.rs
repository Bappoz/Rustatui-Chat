use chat_core::message::chat_message::ChatMessage;
use crate::state::state::ConnectionStatus::Disconnected;

const MAX_MESSAGES_IN_A_ROOM: usize = 100;

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
}