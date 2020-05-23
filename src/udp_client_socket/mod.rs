
extern crate log;

use std::{
    net::{SocketAddr, UdpSocket},
    cell::RefCell,
    rc::Rc
};

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use crate::error::GaiaClientSocketError;
use gaia_socket_shared::{find_my_ip_address, find_available_port, MessageHeader, Config, StringUtils, DEFAULT_MTU};

pub struct UdpClientSocket {
    address: SocketAddr,
    connected: bool,
    timeout: u16,
    socket: Rc<RefCell<UdpSocket>>,
    receive_buffer: Vec<u8>,
}

impl UdpClientSocket {
    pub fn connect(server_address: &str, config: Option<Config>) -> UdpClientSocket {

        let client_ip_address = find_my_ip_address::get();
        let free_socket = find_available_port::get(&client_ip_address).expect("no available ports");
        let client_socket_address = client_ip_address + ":" + free_socket.to_string().as_str();

        let server_socket_address: SocketAddr = server_address.parse().unwrap();

        let socket = Rc::new(RefCell::new(UdpSocket::bind(client_socket_address).unwrap()));

        UdpClientSocket {
            address: server_socket_address,
            connected: false,
            timeout: 0,
            socket,
            receive_buffer: vec![0; DEFAULT_MTU as usize],
        }
    }

    pub fn receive(&mut self) -> Result<SocketEvent, GaiaClientSocketError> {

        if !self.connected {
            if self.timeout > 0 {
                self.timeout -= 1;
            } else {

                match self.socket
                    .borrow()
                    .send_to(&[MessageHeader::ClientHandshake as u8], self.address)
                {
                    Ok(_) => { }
                    Err(err) => { return Err(GaiaClientSocketError::Wrapped(Box::new(err))); }
                }

                self.timeout = 100;
                return Ok(SocketEvent::None);
            }
        }

        let buffer: &mut [u8] = self.receive_buffer.as_mut();
        match self.socket
            .borrow()
            .recv_from(buffer)
            .map(move |(recv_len, address)| (&buffer[..recv_len], address))
        {
            Ok((payload, address)) => {
                if address == self.address {
                    let header: MessageHeader = payload[0].into();
                    match header {
                        MessageHeader::ServerHandshake => {
                            if !self.connected {
                                self.connected = true;
                                return Ok(SocketEvent::Connection);
                            }
                        }
                        MessageHeader::Data => {
                            let msg = String::from_utf8_lossy(payload).to_string().trim_front(1);
                            return Ok(SocketEvent::Message(msg));
                        }
                        _ => {}
                    }
                } else {
                    return Err(GaiaClientSocketError::Message("Unknown sender.".to_string()));
                }
            }
            Err(e) => {
                return Err(GaiaClientSocketError::Wrapped(Box::new(e)));
            }
        }

        return Ok(SocketEvent::None);
    }

    pub fn get_sender(&mut self) -> MessageSender {
        return MessageSender::new(self.address, self.socket.clone());
    }

    pub fn server_address(&self) -> SocketAddr {
        return self.address;
    }
}