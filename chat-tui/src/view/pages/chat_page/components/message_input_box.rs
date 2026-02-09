use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use crate::state::state::InputMode;

pub struct MessageInputBox<'a> {
    content: &'a str,
    input_mode: &'a InputMode,
    can_send: bool,
}

impl<'a> MessageInputBox<'a> {
    pub fn new(
        content: &'a str,
        input_mode: &'a InputMode,
        can_send: bool,
    ) -> Self {
        Self{
            content,
            input_mode,
            can_send,
        }
    }
}

impl<'a> Widget for MessageInputBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let is_editing = matches!(self.input_mode, InputMode::Editing);

        let (border_color, title) = if is_editing {
            (Color::Green, "Message [ESC to exit, ENTER to send]")
        } else {
            (Color::Yellow, "Message [i to edit, q to quit]")
        };

        let text = if is_editing {
            Line::from(vec![
                Span::raw(self.content),
                Span::styled("█", Style::default().fg(Color::Green)),
            ])
        } else {
            Line::from(self.content)
        };

        let title_with_status = if self.can_send && is_editing {
            format!("{} ✓  ", title)
        } else {
            title.to_string()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(border_color)
                    .add_modifier(if is_editing { Modifier::BOLD } else { Modifier::empty() })
            )
            .title(title_with_status);

        let paragraph = Paragraph::new(text).block(block);

        paragraph.render(area, buf);
    }
}