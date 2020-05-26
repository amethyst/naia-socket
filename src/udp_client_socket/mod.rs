
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
use crate::Packet;
use gaia_socket_shared::{find_my_ip_address, find_available_port, MessageHeader, Config, ConnectionManager, DEFAULT_MTU, Timer};

pub struct UdpClientSocket {
    address: SocketAddr,
    connected: bool,
    handshake_timer: Timer,
    socket: Rc<RefCell<UdpSocket>>,
    receive_buffer: Vec<u8>,
    connection_manager: Rc<RefCell<ConnectionManager>>,
    message_sender: MessageSender,
    config: Config,
}

impl UdpClientSocket {
    pub fn connect(server_address: &str, config: Option<Config>) -> UdpClientSocket {

        let client_ip_address = find_my_ip_address::get();
        let free_socket = find_available_port::get(&client_ip_address).expect("no available ports");
        let client_socket_address = client_ip_address + ":" + free_socket.to_string().as_str();

        let server_socket_address: SocketAddr = server_address.parse().unwrap();

        let socket = Rc::new(RefCell::new(UdpSocket::bind(client_socket_address).unwrap()));
        socket.borrow().set_nonblocking(true).expect("can't set socket to non-blocking!");

        let some_config = match config {
            Some(config) => config,
            None => Config::default(),
        };

        let connection_manager = Rc::new(RefCell::new(ConnectionManager::new(some_config.heartbeat_interval, some_config.disconnection_timeout_duration)));
        let message_sender = MessageSender::new(server_socket_address, socket.clone(), connection_manager.clone());
        let mut handshake_timer = Timer::new(some_config.send_handshake_interval);
        handshake_timer.ring_manual();

        UdpClientSocket {
            address: server_socket_address,
            connected: false,
            handshake_timer,
            socket,
            receive_buffer: vec![0; DEFAULT_MTU as usize],
            connection_manager,
            message_sender,
            config: some_config,
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
            if self.handshake_timer.ringing() {
                match self.socket
                    .borrow()
                    .send_to(&[MessageHeader::ClientHandshake as u8], self.address)
                    {
                        Ok(_) => { }
                        Err(err) => { return Err(GaiaClientSocketError::Wrapped(Box::new(err))); }
                    }
                self.handshake_timer.reset();
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
                            let boxed = payload[1..].to_vec().into_boxed_slice();
                            let packet = Packet::new_raw(boxed);
                            return Ok(SocketEvent::Packet(packet));
                        }
                        MessageHeader::Heartbeat => {
                            // Already registered heartbeat, no need for more
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