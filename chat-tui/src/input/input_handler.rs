use crate::state::{action::Action, state::{AppPage, InputMode, FocusedField}};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

pub struct InputHandler;

impl InputHandler {
    pub fn handle_key(
        key: KeyEvent,
        current_page: &AppPage,
        input_mode: &InputMode,
        focused_field: &FocusedField,
    ) -> Option<Action> {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Some(Action::Quit);
        }

        match input_mode {
            InputMode::Normal => Self::handle_normal_mode(key, current_page, focused_field),
            InputMode::Editing => Self::handle_editing_mode(key, current_page, focused_field),

        }
    }

    pub fn handle_normal_mode(
        key: KeyEvent,
        current_page: &AppPage,
        focused_field: &FocusedField,
    ) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Tab => Some(Action::FocusNext),
            KeyCode::BackTab => Some(Action::FocusPrevious),
            KeyCode::Enter => match current_page {
                AppPage::Connection => {
                    if matches!(focused_field, FocusedField::ConnectButton) {
                        Some(Action::Connect)
                    } else {
                        Some(Action::ToggleInputMode)
                    }
                }
                AppPage::Chat => Some(Action::SendMessage)
            },
            KeyCode::Char('i') => Some(Action::ToggleInputMode),
            KeyCode::Up => Some(Action::ScrollUp),
            KeyCode::Down => Some(Action::ScrollDown),
            KeyCode::Char('n') => Some(Action::NextRoom),
            KeyCode::Char('p') => Some(Action::PreviousRoom),
            _ => None,
        }
    }

    pub fn handle_editing_mode(
        key: KeyEvent,
        current_page: &AppPage,
        focused_field: &FocusedField,
    ) -> Option<Action> {
        match key.code {
            KeyCode::Esc => Some(Action::ToggleInputMode),
            KeyCode::Enter => match current_page {
                AppPage::Connection => Some(Action::ToggleInputMode),
                AppPage::Chat => Some(Action::SendMessage),
            },
            KeyCode::Char(c) => match (current_page, focused_field) {
                (AppPage::Connection, FocusedField::ServerAddress) => {
                    Some(Action::UpdateServerAddress(c.to_string()))
                },
                (AppPage::Connection, FocusedField::Username) => {
                    Some(Action::UpdateUsername(c.to_string()))
                },
                (AppPage::Chat, FocusedField::MessageList) => {
                    Some(Action::UpdateMessageInput(c.to_string()))
                },
                _ => None
            },
            KeyCode::Backspace => match (current_page, focused_field) {
                (AppPage::Connection, FocusedField::ServerAddress) => {
                    Some(Action::UpdateServerAddress(String::from("\x08")))
                }
                (AppPage::Connection, FocusedField::Username) => {
                    Some(Action::UpdateUsername(String::from("\x08")))
                },
                (AppPage::Chat, FocusedField::MessageList) => {
                    Some(Action::UpdateMessageInput(String::from("\x08")))
                },
                _ => None
            },
            _ => None,
        }
    }
}