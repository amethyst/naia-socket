use std::{cell::RefCell, error::Error, rc::Rc};

use crate::Packet;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use std::collections::VecDeque;
        use web_sys::RtcDataChannel;

        #[derive(Clone)]
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

        #[derive(Clone)]
        pub struct MessageSender {
            address: SocketAddr,
            socket: Rc<RefCell<UdpSocket>>,
        }

        impl MessageSender {
            pub fn new(address: SocketAddr, socket: Rc<RefCell<UdpSocket>>) -> MessageSender {
                MessageSender {
                    address,
                    socket,
                }
            }
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
