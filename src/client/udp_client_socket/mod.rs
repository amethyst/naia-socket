
use std::thread;
use std::net::SocketAddr;
use std::fmt;
use std::{time};
use std::error::Error;

use crossbeam_channel::{Sender as ChannelSender, Receiver as ChannelReceiver};
use laminar::{ Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent as LaminarEvent, Config as LaminarConfig };
use log::error;

use crate::client::{ClientSocket};
use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use crate::internal_shared::{CLIENT_HANDSHAKE_MESSAGE, SERVER_HANDSHAKE_MESSAGE};
use crate::shared::{find_my_ip_address, find_available_port};

#[derive(Debug)]
pub struct StringError {
    msg: String,
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

impl Error for StringError {}

pub struct UdpClientSocket {
    address: SocketAddr,
    sender: ChannelSender<LaminarPacket>,
    receiver: ChannelReceiver<LaminarEvent>
}

impl ClientSocket for UdpClientSocket {
    fn bind(address: &str) -> UdpClientSocket {

        let mut config = LaminarConfig::default();
        config.heartbeat_interval = Option::Some(time::Duration::from_millis(500));

        let client_ip_address = find_my_ip_address::get();
        let free_socket = find_available_port::get(&client_ip_address).expect("no available ports");
        let client_socket_address = client_ip_address + ":" + free_socket.to_string().as_str();
        println!("UDP Client bound to: {}", client_socket_address);

        let mut client_socket = LaminarSocket::bind_with_config(client_socket_address, config).unwrap();

        let (sender, receiver): (ChannelSender<LaminarPacket>, ChannelReceiver<LaminarEvent>) = (client_socket.get_packet_sender(), client_socket.get_event_receiver());

        let server_address: SocketAddr = address.parse().unwrap();

        //Send initial server handshake
        let line: String = CLIENT_HANDSHAKE_MESSAGE.to_string();
        sender.send(LaminarPacket::reliable_unordered(
            server_address,
            line.clone().into_bytes(),
        ))
            .expect("failure sending client handshake");

        let _thread = thread::spawn(move || client_socket.start_polling());

        UdpClientSocket {
            address: server_address,
            sender,
            receiver,
        }
    }

    fn receive(&mut self) -> SocketEvent {
        match self.receiver.recv() {
            Ok(event) => {
                match event {
                    LaminarEvent::Connect(address) => {
                        // SHOULD NOT EVER GET HERE!, get a SERVER_HANDSHAKE_MESSAGE instead!
                        error!("Client Socket has received a packet from an unknown host!");
                        return SocketEvent::Error(Box::new(StringError { msg: "Client Socket has received a packet from an unknown host!".to_string() }));
                    }
                    LaminarEvent::Packet(packet) => {
                        if packet.addr() == self.address {
                            let msg = String::from_utf8_lossy(packet.payload());

                            if msg.eq(SERVER_HANDSHAKE_MESSAGE) {
                                return SocketEvent::Connection(packet.addr());
                            }
                            else {
                                return SocketEvent::Message(packet.addr(), msg.to_string());
                            }
                        } else {
                            println!("Unknown sender.");
                            return SocketEvent::Error(Box::new(StringError { msg: "Unknown sender.".to_string() }));
                        }
                    }
                    LaminarEvent::Timeout(address) => {

                        return SocketEvent::Disconnection(address);
                    }
                }
            }
            Err(error) => {
                return SocketEvent::Error(Box::new(error));
            }
        }
    }

    fn get_sender(&mut self) -> MessageSender {
        return MessageSender::new(self.address, self.sender.clone());
    }
}