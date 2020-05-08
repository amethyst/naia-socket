
use std::error::Error;
use std::net::SocketAddr;
use crossbeam_channel;
use laminar::Packet as LaminarPacket;


pub struct MessageSender {
    internal: crossbeam_channel::Sender<LaminarPacket>,
    address: SocketAddr
}

impl MessageSender {

    pub fn new(address: SocketAddr, sender: crossbeam_channel::Sender<LaminarPacket>) -> MessageSender {
        MessageSender {
            internal: sender,
            address
        }
    }

    pub fn send(&mut self, message: String) -> Result<(), Box<dyn Error + Send>> {
        match self.internal.send(LaminarPacket::unreliable(self.address,message.into_bytes())) {
            Ok(content) => { Ok(content) },
            Err(error) => { return Err(Box::new(error)); }
        }
    }
}

//#[cfg(feature = "UdpClient")]
//#[cfg(feature = "WebrtcClient")]