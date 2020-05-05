use std::net::SocketAddr;

pub enum ClientEvent {
    Connection(SocketAddr),
    Disconnection(SocketAddr),
    Message(SocketAddr, String),
    Tick
}