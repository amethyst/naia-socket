
use std::{
    collections::{VecDeque, HashMap},
    net::{SocketAddr, UdpSocket},
    cell::RefCell,
    rc::Rc,
    io::ErrorKind,
};
use log::info;

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use gaia_socket_shared::{MessageHeader, Config, DEFAULT_MTU, ConnectionManager, Timer};
use crate::error::GaiaServerSocketError;
use crate::Packet;

pub struct UdpServerSocket {
    socket: Rc<RefCell<UdpSocket>>,
    receive_buffer: Vec<u8>,
    message_sender: MessageSender,
    heartbeat_timer: Timer,
    tick_timer: Timer,
    clients: Rc<RefCell<HashMap<SocketAddr, ConnectionManager>>>,
    outstanding_disconnects: VecDeque<SocketAddr>,
    config: Config,
}

impl UdpServerSocket {
    pub async fn listen(address: &str, config: Option<Config>) -> UdpServerSocket {
        info!("UDP Server listening on: {}", address);

        let socket = Rc::new(RefCell::new(UdpSocket::bind(address).unwrap()));
        socket.borrow().set_nonblocking(true).expect("can't set socket to non-blocking!");

        let mut some_config = match config {
            Some(config) => config,
            None => Config::default(),
        };
        some_config.heartbeat_interval /= 2;

        let clients_map = Rc::new(RefCell::new(HashMap::new()));
        let message_sender = MessageSender::new(socket.clone(), clients_map.clone());

        UdpServerSocket {
            socket,
            receive_buffer: vec![0; DEFAULT_MTU as usize], //should be input from config
            message_sender,
            heartbeat_timer: Timer::new(some_config.heartbeat_interval),
            tick_timer: Timer::new(some_config.tick_interval),
            clients: clients_map,
            outstanding_disconnects: VecDeque::new(),
            config: some_config,
        }
    }

    pub async fn receive(&mut self) -> Result<SocketEvent, GaiaServerSocketError> {
        let mut output: Option<Result<SocketEvent, GaiaServerSocketError>> = None;
        while output.is_none() {

            // heartbeats
            if self.heartbeat_timer.ringing() {
                self.heartbeat_timer.reset();

                for (address, connection) in self.clients.borrow_mut().iter_mut() {
                    if connection.should_drop() {
                        self.outstanding_disconnects.push_back(*address);
                    }
                    else if connection.should_send_heartbeat() {
                        match self.socket
                            .borrow()
                            .send_to(&[MessageHeader::Heartbeat as u8], address)
                            {
                                Ok(_) => {
                                    connection.mark_sent();
                                },
                                Err(error) => { output = Some(Err(GaiaServerSocketError::Wrapped(Box::new(error)))); }
                            }
                    }
                }
            }

            if let Some(addr) = self.outstanding_disconnects.pop_front() {
                self.clients.borrow_mut().remove(&addr);
                output = Some(Ok(SocketEvent::Disconnection(addr)));
            }

            if self.tick_timer.ringing() {
                self.tick_timer.reset();
                output = Some(Ok(SocketEvent::Tick));
            }

            let buffer: &mut [u8] = self.receive_buffer.as_mut();
            match self.socket
                .borrow()
                .recv_from(buffer)
                .map(move |(recv_len, address)| (&buffer[..recv_len], address))
            {
                Ok((payload, address)) => {

                    match self.clients.borrow_mut().get_mut(&address) {
                        Some(connection) => {
                            connection.mark_heard();
                        }
                        None => {
                            //not yet established connection
                        }
                    }

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

                            if !self.clients.borrow().contains_key(&address) {
                                self.clients.borrow_mut().insert(address, ConnectionManager::new(self.config.heartbeat_interval, self.config.disconnection_timeout_duration));
                                output = Some(Ok(SocketEvent::Connection(address)));
                            }
                        }
                        MessageHeader::Data => {
                            let boxed = payload[1..].to_vec().into_boxed_slice();
                            let packet = Packet::new_raw(address, boxed);
                            output = Some(Ok(SocketEvent::Packet(packet)));
                        }
                        MessageHeader::Heartbeat => {
                            // Already registered heartbeat, no need for more
                            info!("Heartbeat");
                        }
                        _ => {}
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    //just didn't receive anything this time
                }
                Err(err) => {
                    output = Some(Err(GaiaServerSocketError::Wrapped(Box::new(err))));
                }
            }
        }
        return output.unwrap();
    }

    pub fn get_sender(&mut self) -> MessageSender {
        return self.message_sender.clone();
    }

    pub fn get_clients(&mut self) -> Vec<SocketAddr> {
        self.clients.borrow().keys().cloned().collect()
    }
}
