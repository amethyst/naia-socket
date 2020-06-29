use std::net::SocketAddr;

/// A Packet that can be sent to a Client
#[derive(Debug)]
pub struct Packet {
    /// The address from which it came, or to which it will go
    address: SocketAddr,
    /// The raw payload of the packet
    payload: Box<[u8]>,
}

impl Packet {
    /// Create a packet from a Vec payload, converts data to a slice on the heap
    pub fn new(address: SocketAddr, payload: Vec<u8>) -> Packet {
        Packet {
            address,
            payload: payload.into_boxed_slice(),
        }
    }

    /// Create a packet from an existing boxed slice of bytes
    pub fn new_raw(address: SocketAddr, payload: Box<[u8]>) -> Packet {
        Packet { address, payload }
    }

    /// Get at the underlying byte payload of the packet
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Get the address the Packet is assigned to
    pub fn address(&self) -> SocketAddr {
        self.address
    }
}
