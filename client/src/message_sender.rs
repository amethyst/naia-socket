use std::{cell::RefCell, error::Error, rc::Rc};

use crate::Packet;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use std::collections::VecDeque;
        use web_sys::RtcDataChannel;

        #[derive(Clone, Debug)]
        pub struct MessageSender {
            data_channel: RtcDataChannel,
            dropped_outgoing_messages: Rc<RefCell<VecDeque<Packet>>>
        }

        impl MessageSender {
            pub fn new(data_channel: RtcDataChannel,
                       dropped_outgoing_messages: Rc<RefCell<VecDeque<Packet>>>) -> MessageSender {
                MessageSender {
                    data_channel,
                    dropped_outgoing_messages
                }
            }
            pub fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {
                if let Err(_) = self.data_channel.send_with_u8_array(&packet.payload()) {
                    self.dropped_outgoing_messages.as_ref().borrow_mut().push_back(packet);
                }
                Ok(())
            }
        }
    }
    else {
        use std::{
            net::{SocketAddr, UdpSocket},
        };

        /// Handles sending messages to the Server for a given Client Socket
        #[derive(Clone, Debug)]
        pub struct MessageSender {
            address: SocketAddr,
            socket: Rc<RefCell<UdpSocket>>,
        }

        impl MessageSender {
            /// Create a new MessageSender, if supplied with the Server's address & a reference back to
            /// the parent Socket
            pub fn new(address: SocketAddr, socket: Rc<RefCell<UdpSocket>>) -> MessageSender {
                MessageSender {
                    address,
                    socket,
                }
            }

            /// Send a Packet to the Server
            pub fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {

                //send it
                if let Err(err) = self.socket.borrow().send_to(&packet.payload(), self.address) {
                    return Err(Box::new(err));
                }
                else {
                    return Ok(());
                }
            }
        }
    }
}
