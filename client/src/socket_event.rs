use crate::Packet;

/// An Event that can be emitted by the Client Socket
#[derive(Debug)]
pub enum SocketEvent {
    /// An event containing a new packet received from the Server
    Packet(Packet),
    /// An event indicating nothing was received
    None,
}
