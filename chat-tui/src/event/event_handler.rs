use std::time::Duration;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyEventKind};

#[derive(Clone, Debug)]
pub enum Event {
    Key(KeyEvent),
    Tick,
    Render,
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Return only events of key pressed 
    pub fn next(&self) -> std::io::Result<Event> {
        if event::poll(self.tick_rate)? {
            if let CrosstermEvent::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    return Ok(Event::Key(key))
                }
            }
        }
        Ok(Event::Tick)
    }
}