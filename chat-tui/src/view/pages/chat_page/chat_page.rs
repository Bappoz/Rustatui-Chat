use ratatui::{
    layout::{Constraint, Direction,Layout, Rect},
    buffer::Buffer,
    widgets::Widget,
};
use crate::state::state::AppState;
use super::components::{Header, MessageInputBox, MessageList, RoomList, UserList, HelpBar};
pub struct ChatPage<'a> {
    state: &'a AppState
}

impl<'a> ChatPage<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    fn render_helper(&self, area: Rect, buf: &mut Buffer) {
        let help_bar = HelpBar::new(&self.state.input_mode);
        help_bar.render(area, buf)
    }

    pub fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let current_room = self.state.current_room.as_deref();
        let header = Header::new(
            &self.state.username,
            &self.state.server_address,
            current_room,
            &self.state.connection_status
        );

        header.render(area, buf);
    }

    fn render_rooms(&self, area: Rect, buf: &mut Buffer) {
        let current_room = self.state.current_room.as_deref();
        let room_list = RoomList::new(&self.state.available_rooms, current_room).focused(false);
        room_list.render(area, buf);
    }

    fn render_users(&self, area: Rect, buf: &mut Buffer) {
        let mut all_users = vec![self.state.username.clone()];
        for user in &self.state.users_in_room {
            if user != &self.state.username {
                all_users.push(user.clone());
            }
        }
        let user_list = UserList::new(&all_users, &self.state.username);
        user_list.render(area, buf);
    }

    pub fn render_sidebar(&self, area: Rect, buf: &mut Buffer) {
        let sidebar_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60),     // Rooms
                Constraint::Percentage(40),     // Users
            ]).split(area);

        self.render_rooms(sidebar_chunks[0], buf);
        self.render_users(sidebar_chunks[1], buf);
    }

    fn render_messages(&self, area: Rect, buf: &mut Buffer) {
        let message_list = MessageList::new(
            &self.state.messages,
            &self.state.username,
            self.state.scroll_offset,
        );
        message_list.render(area, buf)
    }

    fn render_message_input(&self, area: Rect, buf: &mut Buffer) {
        let can_send = self.state.can_send_message();
        let message_input = MessageInputBox::new(
            &self.state.message_input,
            &self.state.input_mode,
            can_send,
        );
        message_input.render(area, buf);
    }

    pub fn render_chat_area(&self, area: Rect, buf: &mut Buffer) {
        let chat_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(3),          // Input
            ]).split(area);

        self.render_messages(chat_chunks[0], buf);
        self.render_message_input(chat_chunks[1], buf);
        self.render_helper(chat_chunks[2], buf);
    }

}

impl<'a> Widget for ChatPage<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Header + Content
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ]).split(area);

        self.render_header(main_chunks[0], buf);

        // Layout do conteúdo: Sidebar + Chat
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20), // Sidebar (Rooms + Users)
                Constraint::Percentage(80), // Área do chat
            ])
            .split(main_chunks[1]);

        // Renderiza sidebar
        self.render_sidebar(content_chunks[0], buf);

        // Renderiza área do chat
        self.render_chat_area(content_chunks[1], buf);
    }
}