
use std::{
    collections::{VecDeque, HashMap},
    net::{SocketAddr, UdpSocket},
    cell::RefCell,
    rc::Rc,
    io::ErrorKind,
    time::Duration,
};
use log::info;

mod timer;
use timer::Timer;

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use gaia_socket_shared::{MessageHeader, Config, StringUtils, DEFAULT_MTU, ConnectionManager};
use crate::error::GaiaServerSocketError;

pub struct UdpServerSocket {
    socket: Rc<RefCell<UdpSocket>>,
    receive_buffer: Vec<u8>,
    message_sender: MessageSender,
    heartbeat_timer: Timer,
    tick_timer: Timer,
    clients: Rc<RefCell<HashMap<SocketAddr, ConnectionManager>>>,
    heartbeat_interval: Duration,
    timeout_duration: Duration,
    outstanding_disconnects: VecDeque<SocketAddr>,
}

impl UdpServerSocket {
    pub async fn listen(address: &str, config: Option<Config>) -> UdpServerSocket {
        info!("UDP Server listening on: {}", address);

        let socket = Rc::new(RefCell::new(UdpSocket::bind(address).unwrap()));
        socket.borrow().set_nonblocking(true).expect("can't set socket to non-blocking!");

        let some_config = config.unwrap();
        let timeout_duration = some_config.idle_connection_timeout;
        let heartbeat_interval = some_config.heartbeat_interval / 2;
        let heartbeat_timer = Timer::new(heartbeat_interval);
        let tick_timer = Timer::new(Duration::from_secs(1));
        let clients_map = Rc::new(RefCell::new(HashMap::new()));
        let message_sender = MessageSender::new(socket.clone(), clients_map.clone());

        UdpServerSocket {
            socket,
            receive_buffer: vec![0; DEFAULT_MTU as usize], //should be input from config
            message_sender,
            heartbeat_timer,
            tick_timer,
            clients: clients_map,
            heartbeat_interval,
            timeout_duration,
            outstanding_disconnects: VecDeque::new()
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
                                self.clients.borrow_mut().insert(address, ConnectionManager::new(self.heartbeat_interval, self.timeout_duration));
                                output = Some(Ok(SocketEvent::Connection(address)));
                            }
                        }
                        MessageHeader::Data => {
                            let message = String::from_utf8_lossy(payload).to_string();
                            output = Some(Ok(SocketEvent::Message(address, message.trim_front(1))));
                        }
                        MessageHeader::Heartbeat => {
                            // Already registered heartbeat, no need for more
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
