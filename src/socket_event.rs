
pub enum SocketEvent {
    Connection,
    Disconnection,
    Message(String),
    None,
}