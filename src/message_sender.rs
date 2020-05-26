
use std::{
    error::Error,
    rc::Rc,
    cell::RefCell
};

use gaia_socket_shared::{MessageHeader, ConnectionManager};

use crate::Packet;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use web_sys::RtcDataChannel;

        #[derive(Clone)]
        pub struct MessageSender {
            data_channel: RtcDataChannel,
            connection_manager: Rc<RefCell<ConnectionManager>>,
        }

        impl MessageSender {
            pub fn new(data_channel: RtcDataChannel, connection_manager: Rc<RefCell<ConnectionManager>>) -> MessageSender {
                MessageSender {
                    data_channel,
                    connection_manager,
                }
            }
            pub fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {
                //add header to packet
                let mut header: Vec<u8> = Vec::new();
                header.push(MessageHeader::Data as u8);
                let outgoing_packet = [header.as_slice(), &packet.payload()]
                    .concat()
                    .into_boxed_slice();

                //send it
                self.data_channel.send_with_u8_array(&outgoing_packet);
                self.connection_manager.borrow_mut().mark_sent();
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
            connection_manager: Rc<RefCell<ConnectionManager>>,
        }

        impl MessageSender {
            pub fn new(address: SocketAddr, socket: Rc<RefCell<UdpSocket>>, connection_manager: Rc<RefCell<ConnectionManager>>) -> MessageSender {
                MessageSender {
                    address,
                    socket,
                    connection_manager,
                }
            }
            pub fn send(&mut self, packet: Packet) -> Result<(), Box<dyn Error + Send>> {

                //add header to packet
                let mut header: Vec<u8> = Vec::new();
                header.push(MessageHeader::Data as u8);
                let outgoing_packet = [header.as_slice(), &packet.payload()]
                    .concat()
                    .into_boxed_slice();

                //send it
                match self.socket
                    .borrow()
                    .send_to(&outgoing_packet, self.address)
                {
                    Ok(_) => {
                        self.connection_manager.borrow_mut().mark_sent();
                        Ok(())
                    }
                    Err(err) => { Err(Box::new(err)) }
                }
            }
        }
    }
}