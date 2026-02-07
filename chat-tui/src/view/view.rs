use ratatui::{
    backend::Backend,
    Frame,
};
use crate::state::state::{AppState, AppPage};
use crate::view::pages::{
    chat_page::chat_page::ChatPage,
    connection_page::connection_page::ConnectionPage,
};

pub struct View;

impl View {
    pub fn render(state: &AppState, frame: &mut Frame) {
        match state.current_page {
            AppPage::Connection => {
                let connection_page = ConnectionPage::new(state);
                frame.render_widget(connection_page, frame.area());
            },
            AppPage::Chat => {
                let chat_page = ChatPage::new(state);
                frame.render_widget(chat_page, frame.area());
            }
        }
    }
}
