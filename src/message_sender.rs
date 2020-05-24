
use std::error::Error;

use gaia_socket_shared::{MessageHeader, StringUtils};

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use web_sys::RtcDataChannel;

        pub struct MessageSender {
            data_channel: RtcDataChannel,
        }

        impl MessageSender {
            pub fn new(data_channel: RtcDataChannel) -> MessageSender {
                MessageSender {
                    data_channel
                }
            }
            pub fn send(&mut self, message: String) -> Result<(), Box<dyn Error + Send>> {
                self.data_channel.send_with_str(&message.push_front(MessageHeader::Data as u8));
                Ok(())
            }
        }
    }
    else {
        use std::{
            net::{SocketAddr, UdpSocket},
            rc::Rc,
            cell::RefCell
        };

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
            pub fn send(&mut self, message: String) -> Result<(), Box<dyn Error + Send>> {
                match self.socket
                    .borrow()
                    .send_to(message.push_front(MessageHeader::Data as u8).as_bytes(), self.address)
                {
                    Ok(_) => { Ok(()) }
                    Err(err) => { Err(Box::new(err)) }
                }
            }
        }
    }
}