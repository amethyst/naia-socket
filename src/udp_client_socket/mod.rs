
extern crate log;
use log::info;

use std::{
    net::{SocketAddr, UdpSocket},
    cell::RefCell,
    rc::Rc,
    io::ErrorKind,
};

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use crate::error::GaiaClientSocketError;
use gaia_socket_shared::{find_my_ip_address, find_available_port, MessageHeader, Config, StringUtils, ConnectionManager, DEFAULT_MTU};

pub struct UdpClientSocket {
    address: SocketAddr,
    connected: bool,
    timeout: u16,
    socket: Rc<RefCell<UdpSocket>>,
    receive_buffer: Vec<u8>,
    connection_manager: Rc<RefCell<ConnectionManager>>,
    message_sender: MessageSender
}

impl UdpClientSocket {
    pub fn connect(server_address: &str, mut config: Option<Config>) -> UdpClientSocket {

        if config.is_none() {
            config = Some(Config::default());
        }

        let client_ip_address = find_my_ip_address::get();
        let free_socket = find_available_port::get(&client_ip_address).expect("no available ports");
        let client_socket_address = client_ip_address + ":" + free_socket.to_string().as_str();

        let server_socket_address: SocketAddr = server_address.parse().unwrap();

        let socket = Rc::new(RefCell::new(UdpSocket::bind(client_socket_address).unwrap()));
        socket.borrow().set_nonblocking(true).expect("can't set socket to non-blocking!");

        let some_config = config.unwrap();
        let heartbeat_interval = some_config.heartbeat_interval;
        let timeout_duration = some_config.idle_connection_timeout;
        let connection_manager = Rc::new(RefCell::new(ConnectionManager::new(heartbeat_interval, timeout_duration)));
        let message_sender = MessageSender::new(server_socket_address, socket.clone(), connection_manager.clone());

        UdpClientSocket {
            address: server_socket_address,
            connected: false,
            timeout: 0,
            socket,
            receive_buffer: vec![0; DEFAULT_MTU as usize],
            connection_manager,
            message_sender
        }
    }

    pub fn receive(&mut self) -> Result<SocketEvent, GaiaClientSocketError> {

        if self.connected {
            if self.connection_manager.borrow().should_drop() {
                self.connected = false;
                return Ok(SocketEvent::Disconnection);
            }
            if self.connection_manager.borrow().should_send_heartbeat() {
                match self.socket
                    .borrow()
                    .send_to(&[MessageHeader::Heartbeat as u8], self.address)
                    {
                        Ok(_) => { self.connection_manager.borrow_mut().mark_sent(); }
                        Err(err) => { return Err(GaiaClientSocketError::Wrapped(Box::new(err))); }
                    }
            }
        } else {
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

                    self.connection_manager.borrow_mut().mark_heard();

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
                        MessageHeader::Heartbeat => {
                            info!("Heartbeat");
                        }
                        _ => {}
                    }
                } else {
                    return Err(GaiaClientSocketError::Message("Unknown sender.".to_string()));
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                //just didn't receive anything this time
                return Ok(SocketEvent::None);
            }
            Err(e) => {
                return Err(GaiaClientSocketError::Wrapped(Box::new(e)));
            }
        }

        return Ok(SocketEvent::None);
    }

    pub fn get_sender(&mut self) -> MessageSender {
        return self.message_sender.clone();
    }

    pub fn server_address(&self) -> SocketAddr {
        return self.address;
    }
}