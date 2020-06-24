
use std::error::Error;

use crate::Packet;

cfg_if! {
    if #[cfg(feature = "use-webrtc")] {
        /// WebRTC Message Sender
        use futures_channel;
        use futures_util::SinkExt;

        pub struct MessageSender {
            internal: futures_channel::mpsc::Sender<Packet>,
        }

        impl MessageSender {
            pub fn new(sender: futures_channel::mpsc::Sender<Packet>) -> MessageSender {
                MessageSender {
                    internal: sender,
                }
            }
            pub async fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {
                match self.internal.send(packet).await {
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
            collections::HashSet,
        };

        #[derive(Clone)]
        pub struct MessageSender {
            socket: Rc<RefCell<UdpSocket>>,
            clients: Rc<RefCell<HashSet<SocketAddr>>>,
        }

        impl MessageSender {
            pub fn new(socket: Rc<RefCell<UdpSocket>>, clients: Rc<RefCell<HashSet<SocketAddr>>>) -> MessageSender {
                MessageSender {
                    socket,
                    clients,
                }
            }
            pub async fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {
                let address = packet.address();
                if !self.clients.borrow_mut().contains(&address) {
                    panic!("sending to an unknown client?");
                }

                //send it
                if let Err(err) = self.socket.borrow().send_to(&packet.payload(), address) {
                    return Err(Box::new(err));
                }
                else {
                    return Ok(());
                }
            }
        }
    }
}