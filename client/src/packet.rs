
/// A Packet that can be sent to the Server
#[derive(Debug)]
pub struct Packet {
    /// The raw payload of the packet
    payload: Box<[u8]>,
}

impl Packet {
    /// Create a packet from a Vec payload, converts data to a slice on the heap
    pub fn new(payload: Vec<u8>) -> Packet {
        Packet {
            payload: payload.into_boxed_slice(),
        }
    }

    /// Create a packet from an existing boxed slice of bytes
    pub fn new_raw(payload: Box<[u8]>) -> Packet {
        Packet { payload }
    }

    /// Create an empty packet
    pub fn empty() -> Packet {
        Packet {
            payload: Box::new([]),
        }
    }

    /// Get at the underlying byte payload of the packet
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}
