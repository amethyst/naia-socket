
pub use super::client_message::ClientMessage;
use std::error::Error;

cfg_if! {
    if #[cfg(feature = "use-webrtc")] {
        /// WebRTC Message Sender
        use futures_channel;
        use futures_util::SinkExt;

        pub struct MessageSender {
            internal: futures_channel::mpsc::Sender<ClientMessage>,
        }

        impl MessageSender {
            pub fn new(sender: futures_channel::mpsc::Sender<ClientMessage>) -> MessageSender {
                MessageSender {
                    internal: sender
                }
            }
            pub async fn send(&mut self, message: ClientMessage) -> Result<(), Box<dyn Error + Send>> {
                match self.internal.send(message).await {
                    Ok(content) => { Ok(content) },
                    Err(error) => { return Err(Box::new(error)); }
                }
            }
        }
    }
    else if #[cfg(feature = "use-udp")] {
        /// UDP Message Sender
        use crossbeam_channel;
        use laminar::Packet as LaminarPacket;

        pub struct MessageSender {
            internal: crossbeam_channel::Sender<LaminarPacket>,
        }

        impl MessageSender {
            pub fn new(sender: crossbeam_channel::Sender<LaminarPacket>) -> MessageSender {
                MessageSender {
                    internal: sender
                }
            }
            pub async fn send(&mut self, message: ClientMessage) -> Result<(), Box<dyn Error + Send>> {
                let (address, message) = message;
                match self.internal.send(LaminarPacket::unreliable(address,message.into_bytes())) {
                    Ok(content) => { Ok(content) },
                    Err(error) => { return Err(Box::new(error)); }
                }
            }
        }
    }
}