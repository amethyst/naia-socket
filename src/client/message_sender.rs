
use std::error::Error;

#[cfg(feature = "UdpClient")]
use std::net::SocketAddr;

#[cfg(feature = "UdpClient")]
use crossbeam_channel;

#[cfg(feature = "UdpClient")]
use laminar::Packet as LaminarPacket;

#[cfg(feature = "UdpClient")]
pub struct MessageSender {
    internal: crossbeam_channel::Sender<LaminarPacket>,
    address: SocketAddr
}

#[cfg(feature = "UdpClient")]
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

#[cfg(feature = "WebrtcClient")]
pub struct MessageSender {
}

#[cfg(feature = "WebrtcClient")]
impl MessageSender {

    pub fn new() -> MessageSender {
        MessageSender {
        }
    }

    pub fn send(&mut self, message: String) -> Result<(), Box<dyn Error + Send>> {
        Ok(())
    }
}

//#[cfg(feature = "UdpClient")]
//#[cfg(feature = "WebrtcClient")]