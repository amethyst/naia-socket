
extern crate log;

use std::{
    net::{SocketAddr, UdpSocket},
    cell::RefCell,
    rc::Rc
};

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use crate::error::GaiaClientSocketError;
use gaia_socket_shared::{find_my_ip_address, find_available_port, SERVER_HANDSHAKE_MESSAGE, CLIENT_HANDSHAKE_MESSAGE, Config};

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
            receive_buffer: vec![0; 1400], //should be input from config
        }
    }

    pub fn receive(&mut self) -> Result<SocketEvent, GaiaClientSocketError> {

        if !self.connected {
            if self.timeout > 0 {
                self.timeout -= 1;
            } else {

                match self.socket
                    .borrow()
                    .send_to(CLIENT_HANDSHAKE_MESSAGE.to_string().as_bytes(), self.address)
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
                    let msg = String::from_utf8_lossy(payload).to_string();

                    if msg.eq(SERVER_HANDSHAKE_MESSAGE) {
                        if !self.connected {
                            self.connected = true;
                            return Ok(SocketEvent::Connection);
                        }
                    }
                    else {
                        return Ok(SocketEvent::Message(msg));
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