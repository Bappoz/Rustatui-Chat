use crate::state::state::{AppPage, ConnectionStatus,AppState, InputMode, FocusedField};
use crate::state::action::Action;
use crate::client::tui_client::TuiClient;
use tokio::sync::mpsc;

pub struct App {
    pub state: AppState,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

impl App {
    pub fn new() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        Self {
            state: AppState::default(),
            action_tx,
            action_rx,
        }
    }

    pub fn action_tx(&self) -> mpsc::UnboundedSender<Action> {
        self.action_tx.clone()
    }

    pub async fn tick(&mut self) {
        // Collect messages
        let messages: Vec<_> = if let Some(client) = &mut self.state.client {
            let mut msgs = Vec::new();
            while let Ok(message) = client.message_rx.try_recv() {
                msgs.push(message);
            }
            msgs
        } else {
            Vec::new()
        };

        // Add message after borrow is done
        for message in messages {
            self.state.add_message(message);
        }

        // Process missing actions
        while let Ok(action) = self.action_rx.try_recv() {
            self.handle_action(action).await;
        }
    }

    async fn connect(&mut self) {
        if self.state.server_address.is_empty() || self.state.username.is_empty() {
            self.state.connection_status = ConnectionStatus::Error("Missing address or username".to_string());
            return;
        }
        self.state.connection_status = ConnectionStatus::Connecting;

        match TuiClient::connect(&self.state.server_address, self.state.username.clone(), self.action_tx.clone()).await {
            Ok(client) => {
                self.state.client = Some(client);
                self.state.connection_status = ConnectionStatus::Connected;
                self.state.current_page = AppPage::Chat;
            },
            Err(e) => {
                self.state.connection_status = ConnectionStatus::Error(e.to_string());
            },
        }
    }

    async fn send_message(&mut self) {
        if !self.state.can_send_message(){
            return;
        }

        let message = self.state.message_input.clone();

        if let Some(client) = &self.state.client {
            // Send to the server - server will echo it back
            if let Err(e) = client.send_message(&message).await {
                self.state.connection_status = ConnectionStatus::Error(e.to_string());
            }
        }

        self.state.clear_input();
    }

    fn handle_text_input(field: &mut String, input: &str) {
        if input == "\x08" {
            field.pop();
        } else {
            field.push_str(input);
        }
    }

    fn focus_next(&mut self) {
        self.state.focused_field = match self.state.current_page {
            AppPage::Connection => match self.state.focused_field {
                FocusedField::ServerAddress => FocusedField::Username,
                FocusedField::Username => FocusedField::ConnectButton,
                FocusedField::ConnectButton => FocusedField::ServerAddress,
                _ => FocusedField::ServerAddress,
            },
            AppPage::Chat => FocusedField::MessageInput,
        };
    }

    fn focus_previous(&mut self) {
        self.state.focused_field = match self.state.current_page {
            AppPage::Connection => match self.state.focused_field {
                FocusedField::ServerAddress => FocusedField::ConnectButton,
                FocusedField::Username => FocusedField::ServerAddress,
                FocusedField::ConnectButton => FocusedField::Username,
                _ => FocusedField::ConnectButton,
            },
            AppPage::Chat => FocusedField::MessageInput,
        };
    }
    async fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                if let Some(client) = &self.state.client {
                    let _ = client.disconnect().await;
                }
                self.state.should_quit = true;
            },
            Action::Connect => self.connect().await,
            Action::Disconnect => {
                if let Some(client) = &self.state.client {
                    let _ = client.disconnect().await;
                }
                self.state.client = None;
                self.state.connection_status = ConnectionStatus::Disconnected;
                self.state.current_page = AppPage::Connection;
            },
            Action::SendMessage => {
                self.send_message().await;
            },
            Action::UpdateServerAddress(input) => {
                Self::handle_text_input(&mut self.state.server_address, &input);
                // Reset error status when user edits
                if matches!(self.state.connection_status, ConnectionStatus::Error(_)) {
                    self.state.connection_status = ConnectionStatus::Disconnected;
                }
            },
            Action::UpdateUsername(input) => {
                Self::handle_text_input(&mut self.state.username, &input);
                // Reset error status when user edits
                if matches!(self.state.connection_status, ConnectionStatus::Error(_)) {
                    self.state.connection_status = ConnectionStatus::Disconnected;
                }
            },
            Action::UpdateMessageInput(input) => {
                Self::handle_text_input(&mut self.state.message_input, &input);
            }
            Action::ToggleInputMode => {
                self.state.input_mode = match self.state.input_mode {
                    InputMode::Normal => {
                        match self.state.current_page{
                            AppPage::Chat => {
                                self.state.focused_field = FocusedField::MessageInput;
                            },
                            _ => {}
                        }
                        InputMode::Editing
                    },
                    InputMode::Editing => InputMode::Normal,
                };
            }
            Action::NextRoom => {
                self.state.next_room();
                if let Some(client) = &self.state.client {
                    if let Some(room) = &self.state.current_room {
                        let _ = client.change_room(room).await;
                    }
                }
            },
            Action::PreviousRoom => {
                self.state.previous_room();
                if let Some(client) = &self.state.client {
                    if let Some(room) = &self.state.current_room {
                        let _ = client.change_room(room).await;
                    }
                }
            },
            Action::ScrollUp => {
                self.state.scroll_up()
            },
            Action::ScrollDown => {
                self.state.scroll_down()
            },
            Action::FocusNext => {
                self.focus_next()
            },
            Action::FocusPrevious => {
                self.focus_previous()
            },
            Action::UpdateUserList(users) => {
                // Filter out current user to avoid duplication
                self.state.users_in_room = users.into_iter()
                    .filter(|u| u != &self.state.username)
                    .collect();
            },
            _ => {}
        }
    }
}