use crate::Packet;

pub enum SocketEvent {
    Packet(Packet),
    None,
}
