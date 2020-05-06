use std::net::SocketAddr;
use std::error::Error;

pub enum ClientEvent {
    Connection(SocketAddr),
    Disconnection(SocketAddr),
    Message(SocketAddr, String),
    Tick,
    Error(Box<dyn Error + Send>)
}