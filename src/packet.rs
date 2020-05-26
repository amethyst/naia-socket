use std::net::SocketAddr;

pub struct Packet {
    /// The address from where it came
    address: SocketAddr,
    /// The raw payload of the packet
    payload: Box<[u8]>,
}

impl Packet {
    pub fn new(address: SocketAddr, payload: Vec<u8>) -> Packet {
        Packet {
            address,
            payload: payload.into_boxed_slice(),
        }
    }

    pub fn new_raw(address: SocketAddr, payload: Box<[u8]>) -> Packet {
        Packet {
            address,
            payload
        }
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }
}