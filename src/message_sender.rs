
use std::error::Error;

#[cfg(not(target_arch = "wasm32"))]
use std::net::SocketAddr;

#[cfg(not(target_arch = "wasm32"))]
use crossbeam_channel;

#[cfg(not(target_arch = "wasm32"))]
use laminar::Packet as LaminarPacket;

#[cfg(not(target_arch = "wasm32"))]
pub struct MessageSender {
    internal: crossbeam_channel::Sender<LaminarPacket>,
    address: SocketAddr
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
pub struct MessageSender {
}

#[cfg(target_arch = "wasm32")]
impl MessageSender {

    pub fn new() -> MessageSender {
        MessageSender {
        }
    }

    pub fn send(&mut self, message: String) -> Result<(), Box<dyn Error + Send>> {
        Ok(())
    }
}