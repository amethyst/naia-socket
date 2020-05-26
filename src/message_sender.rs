
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
            connectionless: bool,
        }

        impl MessageSender {
            pub fn new(sender: futures_channel::mpsc::Sender<Packet>, connectionless: bool) -> MessageSender {
                MessageSender {
                    internal: sender,
                    connectionless
                }
            }
            pub async fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {

                let outpacket = {
                    if self.connectionless {
                        packet
                    }
                    else {
                        //add header to packet
                        let mut header: Vec<u8> = Vec::new();
                        header.push(MessageHeader::Data as u8);
                        let new_payload = [header.as_slice(), &packet.payload()]
                            .concat()
                            .into_boxed_slice();

                        Packet::new_raw(packet.address(), new_payload)
                    }
                };

                match self.internal.send(outpacket).await {
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
                let mut connection_wrapped = self.clients.borrow_mut();
                let connection = connection_wrapped.get_mut(&address).expect("sending to an unknown client?");

                if connection.is_connectionless() {
                    //send it
                    if let Err(err) = self.socket.borrow().send_to(&packet.payload(), address) {
                        return Err(Box::new(err));
                    }
                    else {
                        return Ok(());
                    }
                }
                else {
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
                            connection.mark_sent();
                            Ok(())
                        }
                        Err(err) => { Err(Box::new(err)) }
                    }
                }
            }
        }
    }
}