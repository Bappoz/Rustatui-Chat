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
        let items: Vec<ListItem> = self
            .rooms
            .iter()
            .map(|room| {
                let room_name_clean = room.trim_end_matches("🔒");
                let has_lock = room.ends_with("🔒");

                let is_current = Some(room_name_clean) == self.current_room;

                let style = if is_current {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let display_text = if has_lock {
                    format!("🔒 {}", room_name_clean)
                } else {
                    format!("  {}", room_name_clean)
                };

                ListItem::new(Line::from(Span::styled(display_text, style)))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Rooms ({})", self.rooms.len())),
        );

        list.render(area, buf);
    }
}