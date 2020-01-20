use std::net::SocketAddr;

pub struct Packet {
    addr: SocketAddr,
    payload: Box<[u8]>,
}

impl Packet {
    pub(crate) fn new(
        addr: SocketAddr,
        payload: Box<[u8]>,
    ) -> Packet {
        Packet {
            addr,
            payload,
        }
    }
}