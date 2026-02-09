use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    // Navigation
    Quit,
    SwitchToConnectionPage,
    SwitchToChatPage,

    // Connection
    UpdateServerAddress(String),
    UpdateUsername(String),
    Connect,
    Disconnect,

    //Chat actions
    UpdateMessageInput(String),
    SendMessage,
    ScrollUp,
    ScrollDown,

    //Room actions
    ChangeRoom(String),
    NextRoom,
    PreviousRoom,

    // Ui Actions
    FocusNext,
    FocusPrevious,
    ToggleInputMode,


    // System actions
    Tick,
    Render,
    Error(String),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Action::Quit => write!(f, "Quit"),
            Action::Connect => write!(f, "Connect"),
            Action::Disconnect => write!(f, "Disconnect"),
            Action::SendMessage => write!(f, "SendMessage"),
            _ => write!(f, "{:?}", self)
        }
    }
}