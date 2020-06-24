use std::net::SocketAddr;
use crate::Packet;

pub enum SocketEvent {
    Packet(Packet),
    Tick,
}