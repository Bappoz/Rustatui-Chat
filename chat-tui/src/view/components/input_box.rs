use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct InputBox<'a> {
    title: &'a str,
    content: &'a str,
    is_focused: bool,
    is_editing: bool,
}


impl<'a> InputBox<'a> {
    pub fn new(title: &'a str, content: &'a str) -> Self {
        Self {
            title,
            content,
            is_focused: false,
            is_editing: false,
        }
    }

    pub fn focused(mut self, is_focused: bool) -> Self {
        self.is_focused = is_focused;
        self
    }

    pub fn editing(mut self, is_editing: bool) -> Self {
        self.is_editing = is_editing;
        self
    }
}

impl<'a> Widget for InputBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Define the color by the state

        let (border_color, border_styles) = if self.is_editing {
            (Color::Green, Modifier::BOLD)
        } else if self.is_focused {
            (Color::Yellow, Modifier::empty())
        } else {
            (Color::White, Modifier::empty())
        };

        // Create the block with a border
        let block = Block::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color).add_modifier(border_styles))
            .title(self.title);

        let text = if self.is_editing {
            Line::from(vec![
                Span::raw(self.content),
                Span::styled("█", Style::default().fg(Color::Green))
            ])
        } else {
            Line::from(self.content)
        };

        let paragraph = Paragraph::new(text).block(block);

        paragraph.render(area, buf)
    }
}