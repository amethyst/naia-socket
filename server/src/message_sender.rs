use std::error::Error;

use crate::Packet;

cfg_if! {
    if #[cfg(feature = "use-webrtc")] {
        // WebRTC Message Sender
        use futures_channel;
        use futures_util::SinkExt;

        /// Handles sending messages to a Client that has established a connection with the Server socket
        #[derive(Debug)]
        pub struct MessageSender {
            internal: futures_channel::mpsc::Sender<Packet>,
        }

        impl MessageSender {
            /// Create a new MessageSender, given a reference to a async channel connected to the RtcServer
            pub fn new(sender: futures_channel::mpsc::Sender<Packet>) -> MessageSender {
                MessageSender {
                    internal: sender,
                }
            }
            /// Send a Packet to a client
            pub async fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {
                match self.internal.send(packet).await {
                    Ok(content) => { Ok(content) },
                    Err(error) => { return Err(Box::new(error)); }
                }
            }
        }
    }
    else if #[cfg(feature = "use-udp")] {
        // UDP Message Sender
        use std::{
            rc::Rc,
            cell::RefCell,
            net::{SocketAddr, UdpSocket},
            collections::HashSet,
        };

        /// Handles sending messages to a Client that has established a connection with the Server socket
        #[derive(Clone, Debug)]
        pub struct MessageSender {
            socket: Rc<RefCell<UdpSocket>>,
            clients: Rc<RefCell<HashSet<SocketAddr>>>,
        }

        impl MessageSender {
            /// Create a new MessageSender, if supplied with the UdpSocket and a reference to a list
            /// of established clients
            pub fn new(socket: Rc<RefCell<UdpSocket>>, clients: Rc<RefCell<HashSet<SocketAddr>>>) -> MessageSender {
                MessageSender {
                    socket,
                    clients,
                }
            }

            /// Send a Packet to a client
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
