
use std::error::Error;

use gaia_socket_shared::{MessageHeader, StringUtils};

pub use super::client_message::ClientMessage;

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
                let (address, msg_str) = message;
                let new_message = (address, msg_str.push_front(MessageHeader::Data as u8));
                match self.internal.send(new_message).await {
                    Ok(content) => { Ok(content) },
                    Err(error) => { return Err(Box::new(error)); }
                }
            }
        }
    }
    else if #[cfg(feature = "use-udp")] {
        /// UDP Message Sender
        use std::{
            rc::Rc,
            cell::RefCell,
            net::{SocketAddr, UdpSocket},
            collections::HashMap,
        };
        use gaia_socket_shared::ConnectionManager;

        #[derive(Clone)]
        pub struct MessageSender {
            socket: Rc<RefCell<UdpSocket>>,
            clients: Rc<RefCell<HashMap<SocketAddr, ConnectionManager>>>,
        }

        impl MessageSender {
            pub fn new(socket: Rc<RefCell<UdpSocket>>, clients: Rc<RefCell<HashMap<SocketAddr, ConnectionManager>>>) -> MessageSender {
                MessageSender {
                    socket,
                    clients,
                }
            }
            pub async fn send(&mut self, message: ClientMessage) -> Result<(), Box<dyn Error + Send>> {
                let (address, message) = message;
                match self.socket
                    .borrow()
                    .send_to(message.push_front(MessageHeader::Data as u8).as_bytes(), address)
                {
                    Ok(_) => {
                        match self.clients.borrow_mut().get_mut(&address) {
                            Some(connection) => {
                                connection.mark_sent();
                            }
                            None => {
                                //sending to an unknown address??
                            }
                        }
                        Ok(())
                    }
                    Err(err) => { Err(Box::new(err)) }
                }
            }
        }
    }
}