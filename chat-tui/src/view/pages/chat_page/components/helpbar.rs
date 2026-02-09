use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::state::state::InputMode;

pub struct HelpBar<'a>{
    input_mode: &'a InputMode,
}

impl<'a> HelpBar<'a> {
    pub fn new(input_mode: &'a InputMode) -> Self {
        Self { input_mode }
    }
}

impl<'a> Widget for HelpBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let help_text = match self.input_mode {
            InputMode::Normal => Line::from(vec![
                Span::styled("i", Style::default().fg(Color::Yellow)),
                Span::raw(":edit "),
                Span::styled("n", Style::default().fg(Color::Yellow)),
                Span::raw(":next room "),
                Span::styled("p", Style::default().fg(Color::Yellow)),
                Span::raw(":prev room "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(":scroll "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(":quit"),
            ]),
            InputMode::Editing => Line::from(vec![
                Span::styled("ESC", Style::default().fg(Color::Green)),
                Span::raw(":exit edit "),
                Span::styled("ENTER", Style::default().fg(Color::Green)),
                Span::raw(":send message"),
            ]),
        };

        let help = Paragraph::new(help_text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Commands"));

        help.render(area, buf);

    }
}