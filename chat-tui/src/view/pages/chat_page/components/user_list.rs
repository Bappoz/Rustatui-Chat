use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};

pub struct UserList<'a> {
    users: &'a [String],
    current_username: &'a str,
}

impl<'a> UserList<'a> {
    pub fn new(users: &'a [String], current_username: &'a str) -> Self {
        Self {
            users,
            current_username,
        }
    }
}

impl<'a> Widget for UserList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let items: Vec<ListItem> = self.users
            .iter()
            .map(|user| {
                let is_self = user == self.current_username;
                let line = if is_self {
                    Line::from(vec![
                        Span::styled("● ", Style::default().fg(Color::Green)),
                        Span::styled(
                            format!("{} (you)", user),
                            Style::default().fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ])
                } else {
                    Line::from(vec![
                        Span::styled("● ", Style::default().fg(Color::Cyan)),
                        Span::styled(
                            user,
                            Style::default().fg(Color::Cyan),
                        ),
                    ])
                };
                ListItem::new(line)
            }).collect();


        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("User Online ({})", self.users.len()))
            );

        list.render(area, buf);
    }
}