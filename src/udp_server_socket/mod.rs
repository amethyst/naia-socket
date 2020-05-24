
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    cell::RefCell,
    rc::Rc,
    io::ErrorKind,
    time::Duration,
};
use log::info;

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use gaia_socket_shared::{MessageHeader, Config, StringUtils, DEFAULT_MTU, ConnectionManager};
use crate::error::GaiaServerSocketError;

pub struct UdpServerSocket {
    socket: Rc<RefCell<UdpSocket>>,
    receive_buffer: Vec<u8>,
    message_sender: MessageSender,
    heartbeat_timer: ConnectionManager,
    clients: Rc<RefCell<HashMap<SocketAddr, ConnectionManager>>>,
    heartbeat_interval: Duration,
}

impl UdpServerSocket {
    pub async fn listen(address: &str, config: Option<Config>) -> UdpServerSocket {
        info!("UDP Server listening on: {}", address);

        let socket = Rc::new(RefCell::new(UdpSocket::bind(address).unwrap()));
        socket.borrow().set_nonblocking(true).expect("can't set socket to non-blocking!");

        let heartbeat_interval = config.unwrap().heartbeat_interval / 2;
        let heartbeat_timer = ConnectionManager::new(heartbeat_interval);
        let clients_map = Rc::new(RefCell::new(HashMap::new()));
        let message_sender = MessageSender::new(socket.clone(), clients_map.clone());

        UdpServerSocket {
            socket,
            receive_buffer: vec![0; DEFAULT_MTU as usize], //should be input from config
            message_sender,
            heartbeat_timer,
            clients: clients_map,
            heartbeat_interval
        }
    }

    pub async fn receive(&mut self) -> Result<SocketEvent, GaiaServerSocketError> {
        let mut output: Option<Result<SocketEvent, GaiaServerSocketError>> = None;
        while output.is_none() {

            // heartbeats
            if self.heartbeat_timer.should_send_heartbeat() {
                self.heartbeat_timer.mark_sent();

                for (address, connection) in self.clients.borrow_mut().iter_mut() {
                    if connection.should_send_heartbeat() {
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

                            if !self.clients.borrow().contains_key(&address) {
                                self.clients.borrow_mut().insert(address, ConnectionManager::new(self.heartbeat_interval));
                                output = Some(Ok(SocketEvent::Connection(address)));
                            }
                        }
                        MessageHeader::Data => {
                            let message = String::from_utf8_lossy(payload).to_string();
                            output = Some(Ok(SocketEvent::Message(address, message.trim_front(1))));
                        }
                        MessageHeader::Heartbeat => {
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
}
