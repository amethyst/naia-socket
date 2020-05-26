use std::net::SocketAddr;
use crate::Packet;

pub enum SocketEvent {
    Connection(SocketAddr),
    Disconnection(SocketAddr),
    Packet(Packet),
    Tick,
}