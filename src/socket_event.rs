use crate::Packet;

pub enum SocketEvent {
    Packet(Packet),
    Tick,
}
