use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, Widget, ListItem},
};
use chat_core::message::chat_message::ChatMessage;

pub struct MsgList<'a> {
    messages: &'a [ChatMessage],
    current_username: &'a str,
    scroll_offset: usize,
}

impl<'a> MsgList<'a> {
    pub fn new(
        messages: &'a [ChatMessage],
        current_username: &'a str,
        scroll_offset: usize,
    ) -> Self {
        Self {
            messages,
            current_username,
            scroll_offset
        }
    }

    pub fn format_message(&self, message: &'a ChatMessage) -> ListItem<'a> {
        let is_own_message = message.sender_name == self.current_username;
        let timestamp = message.timestamp.format("%H:%M:%S").to_string();

        let line = if is_own_message {
            Line::from(vec![
                Span::styled(
                    format!("[{}] ", timestamp),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("{}: ", message.sender_name),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::raw(&message.content),
            ])
        } else {
            Line::from(vec![
                Span::styled(
                    format!("[{}] ", timestamp),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("{}: ", message.sender_name),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
                Span::raw(&message.content),
            ])
        };

        ListItem::new(line)
    }
}

impl<'a> Widget for MsgList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let visible_height = area.height.saturating_sub(2) as usize; // -2 para as bordas

        let total_messages = self.messages.len();
        let start_idx = total_messages.saturating_sub(visible_height + self.scroll_offset);
        let end_idx = total_messages.saturating_sub(self.scroll_offset);

        let visible_messages = if start_idx < end_idx {
            &self.messages[start_idx..end_idx]
        } else {
            &[]
        };

        let items: Vec<ListItem> = visible_messages
            .iter()
            .map(|msg| self.format_message(msg))
            .collect();

        // Render the list
        List::new(items).render(area, buf);
    }
}