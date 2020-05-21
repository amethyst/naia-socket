use std::net::SocketAddr;

pub enum SocketEvent {
    Connection(SocketAddr),
    Disconnection(SocketAddr),
    Message(SocketAddr, String),
    Tick,
}