
pub use super::client_message::ClientMessage;
use std::error::Error;

#[cfg(feature = "use-udp")]
use crossbeam_channel;

#[cfg(feature = "use-udp")]
use laminar::Packet as LaminarPacket;

#[cfg(feature = "use-webrtc")]
use futures_channel;

#[cfg(feature = "use-webrtc")]
use futures_util::{SinkExt};

pub struct MessageSender {
    #[cfg(feature = "use-udp")]
    internal: crossbeam_channel::Sender<LaminarPacket>,

    #[cfg(feature = "use-webrtc")]
    internal: futures_channel::mpsc::Sender<ClientMessage>,
}

impl MessageSender {
    #[cfg(feature = "use-udp")]
    pub fn new(sender: crossbeam_channel::Sender<LaminarPacket>) -> MessageSender {
        MessageSender {
            internal: sender
        }
    }

    #[cfg(feature = "use-webrtc")]
    pub fn new(sender: futures_channel::mpsc::Sender<ClientMessage>) -> MessageSender {
        MessageSender {
            internal: sender
        }
    }

    #[cfg(feature = "use-udp")]
    pub async fn send(&mut self, message: ClientMessage) -> Result<(), Box<dyn Error + Send>> {
        let (address, message) = message;
        match self.internal.send(LaminarPacket::unreliable(address,message.into_bytes())) {
            Ok(content) => { Ok(content) },
            Err(error) => { return Err(Box::new(error)); }
        }
    }

    #[cfg(feature = "use-webrtc")]
    pub async fn send(&mut self, message: ClientMessage) -> Result<(), Box<dyn Error + Send>> {
        match self.internal.send(message).await {
            Ok(content) => { Ok(content) },
            Err(error) => { return Err(Box::new(error)); }
        }
    }
}