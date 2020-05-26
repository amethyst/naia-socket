
use crate::Packet;

pub enum SocketEvent {
    Connection,
    Disconnection,
    Packet(Packet),
    None,
}