use std::io;
use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
};

use chat_tui::{
    app::App,
    state::action::Action,
    view::view::View,
    event::event_handler::{Event, EventHandler},
    input::input_handler::InputHandler
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create App
    let mut app = App::new();

    // Create event handler
    let event_handler = EventHandler::new(Duration::from_millis(100));

    // Get action sender
    let action_tx = app.action_tx();

    // main loop
    let result = run_app(&mut terminal, &mut app, event_handler, action_tx).await;

    // Restores terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: EventHandler,
    action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
) -> io::Result<()>
    where io::Error: From<<B as Backend>::Error>
{
    loop {
        // Process messages and actions
        app.tick().await;

        // Render UI
        terminal.draw(|frame| {
            View::render(&app.state, frame);
        })?;

        // quit
        if app.state.should_quit {
            break;
        }

        // Capture events with timeout
        match event_handler.next()? {
            Event::Key(key) => {
                // Translate keyboard actions
                if let Some(action) = InputHandler::handle_key(
                    key,
                    &app.state.current_page,
                    &app.state.input_mode,
                    &app.state.focused_field,
                ) {
                    // Send action to process
                    let _ = action_tx.send(action);
                }
            }
            Event::Tick => {}
            Event::Render => {}
        }
    }

    Ok(())
}