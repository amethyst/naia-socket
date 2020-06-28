extern crate log;

use std::{
    cell::RefCell,
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    rc::Rc,
};

use super::message_sender::MessageSender;
use super::socket_event::SocketEvent;
use crate::error::NaiaClientSocketError;
use crate::Packet;
use naia_socket_shared::{find_available_port, find_my_ip_address, Config};

#[derive(Debug)]
pub struct UdpClientSocket {
    address: SocketAddr,
    socket: Rc<RefCell<UdpSocket>>,
    receive_buffer: Vec<u8>,
    message_sender: MessageSender,
}

impl UdpClientSocket {
    pub fn connect(server_address: &str, _: Option<Config>) -> UdpClientSocket {
        let client_ip_address = find_my_ip_address().expect("cannot find current ip address");
        let free_socket = find_available_port(&client_ip_address).expect("no available ports");
        let client_socket_address = format!("{}:{}", client_ip_address, free_socket);

        let server_socket_address: SocketAddr = server_address.parse().unwrap();

        let socket = Rc::new(RefCell::new(
            UdpSocket::bind(client_socket_address).unwrap(),
        ));
        socket
            .borrow()
            .set_nonblocking(true)
            .expect("can't set socket to non-blocking!");

        let message_sender = MessageSender::new(server_socket_address, socket.clone());

        UdpClientSocket {
            address: server_socket_address,
            socket,
            receive_buffer: vec![0; 1472],
            message_sender,
        }
    }

    pub fn receive(&mut self) -> Result<SocketEvent, NaiaClientSocketError> {
        let buffer: &mut [u8] = self.receive_buffer.as_mut();
        match self
            .socket
            .borrow()
            .recv_from(buffer)
            .map(move |(recv_len, address)| (&buffer[..recv_len], address))
        {
            Ok((payload, address)) => {
                if address == self.address {
                    return Ok(SocketEvent::Packet(Packet::new(payload.to_vec())));
                } else {
                    return Err(NaiaClientSocketError::Message(
                        "Unknown sender.".to_string(),
                    ));
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                //just didn't receive anything this time
                return Ok(SocketEvent::None);
            }
            Err(e) => {
                return Err(NaiaClientSocketError::Wrapped(Box::new(e)));
            }
        }
    }

    pub fn get_sender(&mut self) -> MessageSender {
        return self.message_sender.clone();
    }

    pub fn server_address(&self) -> SocketAddr {
        return self.address;
    }
}
