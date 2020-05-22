
use std::{
    thread,
    collections::HashSet,
    time,
    net::SocketAddr};
use log::info;

use laminar::{Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent as LaminarEvent, Config as LaminarConfig};
use crossbeam_channel::{self, Receiver, Sender};

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use gaia_socket_shared::{SERVER_HANDSHAKE_MESSAGE, CLIENT_HANDSHAKE_MESSAGE, Config};
use crate::error::GaiaServerSocketError;

pub struct UdpServerSocket {
    sender: Sender<LaminarPacket>,
    receiver: Receiver<LaminarEvent>,
    connected_clients: HashSet<SocketAddr>,
}

impl UdpServerSocket {
    pub async fn listen(address: &str, config: Option<Config>) -> UdpServerSocket {
        info!("UDP Server listening on: {}", address);

        let mut config = LaminarConfig::default();
        config.heartbeat_interval = Option::Some(time::Duration::from_millis(500));
        let mut socket = LaminarSocket::bind_with_config(address, config).unwrap();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

        let _thread = thread::spawn(move || socket.start_polling());

        UdpServerSocket {
            sender,
            receiver,
            connected_clients: HashSet::new()
        }
    }

    pub async fn receive(&mut self) -> Result<SocketEvent, GaiaServerSocketError> {
        let mut output: Option<Result<SocketEvent, GaiaServerSocketError>> = None;
        while output.is_none() {
            match self.receiver.recv() {
                Ok(event) => {
                    match event {
                        LaminarEvent::Connect(_) => { }
                        LaminarEvent::Packet(packet) => {
                            let message = String::from_utf8_lossy(packet.payload()).to_string();
                            let address = packet.addr();

                            if message.eq(CLIENT_HANDSHAKE_MESSAGE) {

                                // Server Handshake
                                match self.sender.send(LaminarPacket::unreliable(address, SERVER_HANDSHAKE_MESSAGE.to_string().into_bytes())) {
                                    Ok(_) => {},
                                    Err(error) => { output = Some(Err(GaiaServerSocketError::Wrapped(Box::new(error)))); }
                                }

                                if !self.connected_clients.contains(&address) {
                                    self.connected_clients.insert(address);
                                    output = Some(Ok(SocketEvent::Connection(address)));
                                }
                            } else {
                                output = Some(Ok(SocketEvent::Message(address, message)));
                            }
                        }
                        LaminarEvent::Timeout(_) => { }
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
        return MessageSender::new(self.sender.clone());
    }
}
