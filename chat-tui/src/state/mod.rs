pub mod action;
pub mod state;


use std::sync::Arc;
use tokio::sync::Mutex;
use crate::state::state::AppState;

pub type SharedState = Arc<Mutex<AppState>>;

pub fn create_shared_state() -> SharedState {
    Arc::new(Mutex::new(AppState::new()))
}