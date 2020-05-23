
use std::{
    collections::HashSet,
    net::{SocketAddr, UdpSocket},
    cell::RefCell,
    rc::Rc,
};
use log::info;

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use gaia_socket_shared::{MessageHeader, Config, StringUtils, DEFAULT_MTU};
use crate::error::GaiaServerSocketError;

pub struct UdpServerSocket {
    connected_clients: HashSet<SocketAddr>,
    socket: Rc<RefCell<UdpSocket>>,
    receive_buffer: Vec<u8>,
}

impl UdpServerSocket {
    pub async fn listen(address: &str, config: Option<Config>) -> UdpServerSocket {
        info!("UDP Server listening on: {}", address);

        let socket = Rc::new(RefCell::new(UdpSocket::bind(address).unwrap()));

        UdpServerSocket {
            connected_clients: HashSet::new(),
            socket,
            receive_buffer: vec![0; DEFAULT_MTU as usize], //should be input from config
        }
    }

    pub async fn receive(&mut self) -> Result<SocketEvent, GaiaServerSocketError> {
        let mut output: Option<Result<SocketEvent, GaiaServerSocketError>> = None;
        while output.is_none() {

            let buffer: &mut [u8] = self.receive_buffer.as_mut();
            match self.socket
                .borrow()
                .recv_from(buffer)
                .map(move |(recv_len, address)| (&buffer[..recv_len], address))
            {
                Ok((payload, address)) => {
                    let header: MessageHeader = payload[0].into();
                    match header {
                        MessageHeader::ClientHandshake => {
                            // Server Handshake
                            match self.socket
                                .borrow()
                                .send_to(&[MessageHeader::ServerHandshake as u8], address)
                                {
                                    Ok(_) => {},
                                    Err(error) => { output = Some(Err(GaiaServerSocketError::Wrapped(Box::new(error)))); }
                                }

                            if !self.connected_clients.contains(&address) {
                                self.connected_clients.insert(address);
                                output = Some(Ok(SocketEvent::Connection(address)));
                            }
                        }
                        MessageHeader::Data => {
                            let message = String::from_utf8_lossy(payload).to_string();
                            output = Some(Ok(SocketEvent::Message(address, message.trim_front(1))));
                        }
                        _ => {}
                    }
                }
                Err(err) => {
                    output = Some(Err(GaiaServerSocketError::Wrapped(Box::new(err))));
                }
            }

        }
        return output.unwrap();
    }

    pub fn get_sender(&mut self) -> MessageSender {
        return MessageSender::new(self.socket.clone());
    }
}
