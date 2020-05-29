
pub struct Packet {
    /// The raw payload of the packet
    payload: Box<[u8]>,

}

impl Packet {
    pub fn new(payload: Vec<u8>) -> Packet {
        Packet {
            payload: payload.into_boxed_slice(),
        }
    }

    pub fn new_raw(payload: Box<[u8]>) -> Packet {
        Packet {
            payload,
        }
    }

    pub fn empty() -> Packet {
        Packet {
            payload: Box::new([]),
        }
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}