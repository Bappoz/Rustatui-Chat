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
        let help_message = match self.input_mode {
            InputMode::Normal => {
                "i:edit | n:next room | p:prev room | ↑↓:scroll | q:quit"
            }
            InputMode::Editing => {
                "Esc:normal | Enter:send"
            }
        };

        let help = Paragraph::new(help_message)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Commands"));

        help.render(area, buf);

    }
}