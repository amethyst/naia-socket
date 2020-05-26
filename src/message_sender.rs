
use std::error::Error;

use gaia_socket_shared::{MessageHeader};

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
                    internal: sender
                }
            }
            pub async fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {

                let address = packet.address();

                //add header to packet
                let mut header: Vec<u8> = Vec::new();
                header.push(MessageHeader::Data as u8);
                let new_payload = [header.as_slice(), &packet.payload()]
                    .concat()
                    .into_boxed_slice();

                /////////
                match self.internal.send(Packet::new_raw(address, new_payload)).await {
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
            pub async fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {
                let address = packet.address();

                //add header to packet
                let mut header: Vec<u8> = Vec::new();
                header.push(MessageHeader::Data as u8);
                let outgoing_packet = [header.as_slice(), &packet.payload()]
                    .concat()
                    .into_boxed_slice();

                //send it
                match self.socket
                    .borrow()
                    .send_to(&outgoing_packet, address)
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