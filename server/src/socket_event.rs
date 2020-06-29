use crate::Packet;

/// An Event that can be emitted by the Server Socket
#[derive(Debug)]
pub enum SocketEvent {
    /// An event containing a new packet received from a Client
    Packet(Packet),
    /// A Tick event
    /// The duration between Ticks is defined in the Config given to the Server Socket
    Tick,
}
