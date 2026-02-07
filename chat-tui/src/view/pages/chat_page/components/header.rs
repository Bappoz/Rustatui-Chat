use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use crate::state::state::ConnectionStatus;

pub struct Header<'a> {
    username: &'a str,
    server_address: &'a str,
    current_room: Option<&'a str>,
    connection_status: &'a ConnectionStatus
}

impl<'a> Header<'a> {
    pub fn new(
        username: &'a str,
        server_address: &'a str,
        current_room: Option<&'a str>,
        connection_status: &'a ConnectionStatus
    ) -> Self {
        Self {
            username,
            server_address,
            current_room,
            connection_status,
        }
    }
}

impl<'a> Widget for Header<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (status_icon, status_color) = match self.connection_status {
            ConnectionStatus::Connected => ("●", Color::Green),
            ConnectionStatus::Connecting => ("◐", Color::Yellow),
            ConnectionStatus::Disconnected => ("○", Color::Red),
            ConnectionStatus::Error(_) => ("✖", Color::Red),
        };
        let room_text = if let Some(room) = self.current_room {
            format!(" | Room {}", room)
        } else {
            String::from(" | No Room")
        };

        let text = Line::from(vec![
            Span::styled(status_icon, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(self.username, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" @ "),
            Span::styled(self.server_address, Style::default().fg(Color::Gray)),
            Span::styled(&room_text, Style::default().fg(Color::Yellow)),
        ]);

        let header = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Chat Application"));

        header.render(area, buf)
    }
}