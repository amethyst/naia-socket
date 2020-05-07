
pub use super::ClientMessage;
use std::error::Error;

#[cfg(feature = "UdpServer")]
use crossbeam_channel;

#[cfg(feature = "UdpServer")]
use laminar::Packet as LaminarPacket;

#[cfg(feature = "WebrtcServer")]
use futures_channel;

#[cfg(feature = "WebrtcServer")]
use futures_util::{SinkExt};

pub struct MessageSender {
    #[cfg(feature = "UdpServer")]
    internal: crossbeam_channel::Sender<LaminarPacket>,

    #[cfg(feature = "WebrtcServer")]
    internal: futures_channel::mpsc::Sender<ClientMessage>,
}

impl MessageSender {
    #[cfg(feature = "UdpServer")]
    pub fn new(sender: crossbeam_channel::Sender<LaminarPacket>) -> MessageSender {
        MessageSender {
            internal: sender
        }
    }

    #[cfg(feature = "WebrtcServer")]
    pub fn new(sender: futures_channel::mpsc::Sender<ClientMessage>) -> MessageSender {
        MessageSender {
            internal: sender
        }
    }

    #[cfg(feature = "UdpServer")]
    pub async fn send(&mut self, message: ClientMessage) -> Result<(), Box<dyn Error + Send>> {
        let (address, message) = message;
        match self.internal.send(LaminarPacket::unreliable(address,message.into_bytes())) {
            Ok(content) => { Ok(content) },
            Err(error) => { return Err(Box::new(error)); }
        }
    }

    #[cfg(feature = "WebrtcServer")]
    pub async fn send(&mut self, message: ClientMessage) -> Result<(), Box<dyn Error + Send>> {
        match self.internal.send(message).await {
            Ok(content) => { Ok(content) },
            Err(error) => { return Err(Box::new(error)); }
        }
    }
}

//#[cfg(feature = "UdpServer")]

//#[cfg(feature = "WebrtcServer")]