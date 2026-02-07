use ratatui::{
    layout::{Constraint, Direction,Layout, Rect},
    buffer::Buffer,
    widgets::Widget,
};
use crate::view::pages::chat_page::components::{
    header::Header,
    message_input_box::MessageInputBox,
    message_list::MsgList,
    room_list::RoomList,
    user_list::UserList,
};
use crate::state::state::AppState;

pub struct ChatPage<'a> {
    state: &'a AppState
}

impl<'a> ChatPage<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let current_room = Some("general");
        let header = Header::new(
            &self.state.username,
            &self.state.server_address,
            current_room,
            &self.state.connection_status
        );

        header.render(area, buf);
    }

    fn render_rooms(&self, area: Rect, buf: &mut Buffer) {
        let rooms = vec![
            "general".to_string(),
            "random".to_string(),
            "tech".to_string(),
        ];
        let current_room = Some("general");
        let room_list = RoomList::new(&rooms, current_room).focused(false);
        room_list.render(area, buf);
    }

    fn render_users(&self, area: Rect, buf: &mut Buffer) {
        let users = vec![
            self.state.username.clone(),
            "bob".to_string(),
            "Alice".to_string(),
        ];
        let user_list = UserList::new(&users, &self.state.username);
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
        let message_list = MsgList::new(
            &self.state.messages,
            &self.state.username,
            self.state.scroll_offset,
        );
        message_list.render(area, buf)
    }

    fn render_message_input(&self, area: Rect, buf: &mut Buffer) {
        let can_send = self.state.can_send_message();
        let messsage_input = MessageInputBox::new(
            &self.state.message_input,
            &self.state.input_mode,
            can_send,
        );
        messsage_input.render(area, buf);
    }

    pub fn render_chat_area(&self, area: Rect, buf: &mut Buffer) {
        let chat_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(90),     // Message
                Constraint::Length(3),          // Input
            ]).split(area);

        self.render_messages(chat_chunks[0], buf);
        self.render_message_input(chat_chunks[1], buf)
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
    }
}