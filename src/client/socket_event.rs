use std::net::SocketAddr;
use std::error::Error;

pub enum SocketEvent {
    Connection(SocketAddr),
    Disconnection(SocketAddr),
    Message(SocketAddr, String),
    Error(Box<dyn Error + Send>),
    None,
}