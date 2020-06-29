use std::{
    cell::RefCell,
    collections::HashSet,
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    rc::Rc,
};

use super::message_sender::MessageSender;
use super::socket_event::SocketEvent;
use crate::error::NaiaServerSocketError;
use crate::Packet;
use naia_socket_shared::{Config, Timer};

#[derive(Debug)]
pub struct UdpServerSocket {
    socket: Rc<RefCell<UdpSocket>>,
    receive_buffer: Vec<u8>,
    message_sender: MessageSender,
    tick_timer: Timer,
    clients: Rc<RefCell<HashSet<SocketAddr>>>,
}

impl UdpServerSocket {
    pub async fn listen(address: &str, config: Option<Config>) -> UdpServerSocket {
        let socket = Rc::new(RefCell::new(UdpSocket::bind(address).unwrap()));
        socket
            .borrow()
            .set_nonblocking(true)
            .expect("can't set socket to non-blocking!");

        let tick_interval = match config {
            Some(config) => config.tick_interval,
            None => Config::default().tick_interval,
        };

        let clients = Rc::new(RefCell::new(HashSet::new()));
        let message_sender = MessageSender::new(socket.clone(), clients.clone());

        UdpServerSocket {
            socket,
            receive_buffer: vec![0; 1472], //should be input from config
            message_sender,
            tick_timer: Timer::new(tick_interval),
            clients: clients,
        }
    }

    pub async fn receive(&mut self) -> Result<SocketEvent, NaiaServerSocketError> {
        let mut output: Option<Result<SocketEvent, NaiaServerSocketError>> = None;
        while output.is_none() {
            if self.tick_timer.ringing() {
                self.tick_timer.reset();
                output = Some(Ok(SocketEvent::Tick));
                continue;
            }

            let buffer: &mut [u8] = self.receive_buffer.as_mut();
            match self
                .socket
                .borrow()
                .recv_from(buffer)
                .map(move |(recv_len, address)| (&buffer[..recv_len], address))
            {
                Ok((payload, address)) => {
                    if !self.clients.borrow().contains(&address) {
                        self.clients.borrow_mut().insert(address);
                    }
                    let packet = Packet::new(address, payload.to_vec());
                    output = Some(Ok(SocketEvent::Packet(packet)));
                    continue;
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    //just didn't receive anything this time
                }
                Err(err) => {
                    output = Some(Err(NaiaServerSocketError::Wrapped(Box::new(err))));
                    continue;
                }
            }
        }
        return output.unwrap();
    }

    pub fn get_sender(&mut self) -> MessageSender {
        return self.message_sender.clone();
    }

    pub fn get_clients(&mut self) -> Vec<SocketAddr> {
        self.clients.borrow().iter().cloned().collect()
    }
}
