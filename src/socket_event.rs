use std::error::Error;

pub enum SocketEvent {
    Connection,
    Disconnection,
    Message(String),
    Error(Box<dyn Error + Send>),
    None,
}