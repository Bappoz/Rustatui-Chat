use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use crate::state::state::{AppState, InputMode, ConnectionStatus, FocusedField};
use crate::view::components::input_box::InputBox;

pub struct ConnectionPage<'a> {
    state: &'a AppState,
}

impl<'a> ConnectionPage<'a> {
    pub fn new( state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for ConnectionPage<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),      // Title
                Constraint::Min(0),         // Content
                Constraint::Length(3)       // Status
            ]).split(area);

        self.render_title(chunks[0], buf);
        self.render_inputs(chunks[1], buf);
        self.render_status(chunks[2], buf);
    }
}

impl<'a> ConnectionPage<'a> {
    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let title = Paragraph::new("Chat App - Connection")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::default()));

        title.render(area, buf);
    }

    fn render_inputs(&self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),      // Server Address
                Constraint::Length(3),      // Username
                Constraint::Length(3),      // Connect button
                Constraint::Min(0),
            ]).split(area);

        let server_input = InputBox::new("Server Address", &self.state.server_address)
            .focused(matches!(self.state.focused_field, FocusedField::ServerAddress))
            .editing(
        matches!(self.state.focused_field, FocusedField::ServerAddress)
                 && matches!(self.state.input_mode, InputMode::Editing)
            );
        server_input.render(chunks[0], buf);

        let username = InputBox::new("Username", &self.state.username)
            .focused(matches!(self.state.focused_field, FocusedField::Username))
            .editing(
                matches!(self.state.focused_field, FocusedField::Username)
                && matches!(self.state.input_mode, InputMode::Editing)
            );
        username.render(chunks[1], buf);

        self.render_connect_button(chunks[2], buf);
    }

    pub fn render_connect_button(&self, area: Rect, buf: &mut Buffer) {
        let is_focused = matches!(self.state.focused_field, FocusedField::ConnectButton);
        let can_connect = !self.state.server_address.is_empty()
            && !self.state.username.is_empty()
            && matches!(self.state.connection_status, 
                ConnectionStatus::Disconnected | ConnectionStatus::Error(_));

        let (text, style) = if can_connect {
            ("[ Connect ]", Style::default().fg(if is_focused {Color::Yellow} else {Color::Green}))
        } else {
            ("[ Connect ]", Style::default().fg(Color::DarkGray))
        };

        let button = Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(style)
            .block(Block::default().borders(Borders::ALL));

        button.render(area, buf)
    }

    pub fn render_status(&self, area: Rect, buf: &mut Buffer) {
        let (status_text, status_color) = match &self.state.connection_status {
            ConnectionStatus::Disconnected => ("Disconnected", Color::Gray),
            ConnectionStatus::Connecting => ("Connecting...", Color::Yellow),
            ConnectionStatus::Connected => ("Connected", Color::Green),
            ConnectionStatus::Error(err) => (err.as_str(), Color::Red),
        };

        let status = Paragraph::new(Line::from(vec![
            Span::raw("Status: "),
            Span::styled(status_text, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
        ]))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        status.render(area, buf)
    }


}