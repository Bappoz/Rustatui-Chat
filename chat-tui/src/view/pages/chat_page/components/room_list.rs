use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};

pub struct RoomList<'a> {
    rooms: &'a [String],
    current_room: Option<&'a str>,
    is_focused: bool,
}

impl<'a> RoomList<'a> {
    pub fn new(rooms: &'a [String], current_room: Option<&'a str>) -> Self {
        Self{
            rooms,
            current_room,
            is_focused: false,
        }
    }

    pub fn focused(mut self, is_focused: bool) -> Self {
        self.is_focused = is_focused;
        self
    }
}

impl<'a> Widget for RoomList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let border_color = if self.is_focused {
            Color::Yellow
        } else {
            Color::White
        };

        let items: Vec<ListItem> = self.rooms
            .iter()
            .map(|room| {
                let is_current = self.current_room == Some(room.as_str());
                let line = if is_current {
                    Line::from(vec![
                        Span::styled("► ", Style::default().fg(Color::Green)),
                        Span::styled(room, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    ])
                } else {
                    Line::from(vec![
                        Span::raw("  "),
                        Span::raw(room),
                    ])
                };
                ListItem::new(line)
            }).collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
                    .title(format!("Rooms ({})", self.rooms.len()))
            );
        list.render(area, buf)
    }
}