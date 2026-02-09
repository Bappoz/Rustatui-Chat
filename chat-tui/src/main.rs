use std::io;
use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use chat_tui::state::state::AppState;
use chat_tui::view::view::View;
use chat_tui::event::event_handler::{EventHandler, Event};
use chat_tui::input::input_handler::InputHandler;
use chat_tui::state::state::{AppPage, InputMode, FocusedField};
use chat_tui::state::action::Action;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App state
    let mut app_state = AppState::new();

    // create the event handle (captures the binds)
    let event_handler = EventHandler::new(Duration::from_millis(250));

    // main loop
    let result = run_app(&mut terminal, &mut app_state, event_handler);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    state: &mut AppState,
    event_handler: EventHandler,
) -> Result<(), Box<dyn std::error::Error>>
    where
        B::Error: 'static,
{
    loop {
        // Render the UI
        terminal.draw(|frame| {
            View::render(state, frame);
        })?;

        // Capture the events
        match event_handler.next()? {
            Event::Key(key) => {
                // Translate keybind action
                if let Some(action) = InputHandler::handle_key(
                    key,
                    &state.current_page,
                    &state.input_mode,
                    &state.focused_field,
                ) {
                    handle_action(state, action);

                    if state.should_quit {
                        break;
                    }
                }
            }
            Event::Tick => {}
            Event::Render => {
                // forces render
            }
        }
    }

    Ok(())
}

fn handle_action(state: &mut AppState, action: Action) {
    match action {
        Action::ScrollUp => {
            state.scroll_offset = state.scroll_offset.saturating_add(1);
        },
        Action::ScrollDown => {
            state.scroll_offset = state.scroll_offset.saturating_sub(1);
        },
        Action::NextRoom => {
            state.next_room();
        },
        Action::PreviousRoom => {
            state.previous_room();
        },
        Action::ChangeRoom(room) => {
            state.change_room(room);
        },
        Action::Quit => {
            state.should_quit = true;
        }
        Action::SwitchToConnectionPage => {
            state.current_page = AppPage::Connection;
        }
        Action::SwitchToChatPage => {
            state.current_page = AppPage::Chat;
        }
        Action::UpdateServerAddress(input) => {
            handle_text_input(&mut state.server_address, &input);
        }
        Action::UpdateUsername(input) => {
            handle_text_input(&mut state.username, &input);
        }
        Action::UpdateMessageInput(input) => {
            handle_text_input(&mut state.message_input, &input);
        }
        Action::Connect => {
            // TODO: implementar conexão real
            if !state.username.is_empty() && !state.server_address.is_empty() {
                // Adiciona o usuário na lista
                state.users_in_room.insert(0, state.username.clone());
                state.current_page = AppPage::Chat;
            }
        }
        Action::SendMessage => {
            if state.can_send_message() {
                // TODO: send message
                state.clear_input();
            }
        }
        Action::ToggleInputMode => {
            state.input_mode = match state.input_mode {
                InputMode::Normal => InputMode::Editing,
                InputMode::Editing => InputMode::Normal,
            };
        }
        Action::FocusNext => {
            state.focused_field = match state.current_page {
                AppPage::Connection => match state.focused_field {
                    FocusedField::ServerAddress => FocusedField::Username,
                    FocusedField::Username => FocusedField::ConnectButton,
                    FocusedField::ConnectButton => FocusedField::ServerAddress,
                    _ => FocusedField::ServerAddress,
                },
                AppPage::Chat => FocusedField::MessageInput,
            };
        }
        Action::FocusPrevious => {
            state.focused_field = match state.current_page {
                AppPage::Connection => match state.focused_field {
                    FocusedField::ServerAddress => FocusedField::ConnectButton,
                    FocusedField::Username => FocusedField::ServerAddress,
                    FocusedField::ConnectButton => FocusedField::Username,
                    _ => FocusedField::ConnectButton,
                },
                AppPage::Chat => FocusedField::MessageInput,
            };
        }
        _ => {}
    }
}

fn handle_text_input(field: &mut String, input: &str) {
    if input == "\x08" {
        // Backspace
        field.pop();
    } else {
        // Adiciona caractere
        field.push_str(input);
    }
}