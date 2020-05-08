
use std::time::Instant;
use std::net::SocketAddr;
use std::fmt;
use std::{time};
use std::borrow::Borrow;
use std::error::Error;

use crossbeam_channel::{Sender as ChannelSender, Receiver as ChannelReceiver};
use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent, Config as LaminarConfig};
use log::error;

use crate::client::{ClientSocket};
use super::server_event::ServerEvent;
use super::message_sender::MessageSender;

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
    receiver: ChannelReceiver<SocketEvent>
}

impl ClientSocket for UdpClientSocket {
    fn bind(address: &str) -> UdpClientSocket {
        println!("Hello UdpClientSocket!");

        let mut config = LaminarConfig::default();
        config.heartbeat_interval = Option::Some(time::Duration::from_millis(500));
        let mut client_socket = LaminarSocket::bind_with_config("127.0.0.1:12352", config).unwrap();
        println!("ClientSocket.bind() {}", "127.0.0.1:12352");

        let sender: ChannelSender<LaminarPacket> = client_socket.get_packet_sender();

        let server_address: SocketAddr = address.parse().unwrap();

        //Send initial server handshake
        let line: String = "client-handshake-request".to_string();
        println!("Client sending 'client-handshake-request'");
        sender.send(LaminarPacket::reliable_unordered(
            server_address,
            line.clone().into_bytes(),
        ));

        client_socket.manual_poll(Instant::now());

        UdpClientSocket {
            address: server_address,
            sender,
            receiver: client_socket.get_event_receiver(),
        }
    }

    fn receive(&mut self) -> ServerEvent {
        match self.receiver.recv() {
            Ok(event) => {
                match event {
                    SocketEvent::Connect(address) => {
                        // SHOULD NOT EVER GET HERE!, get a server-handshake-response instead!
                        error!("Client Socket has received a packet from an unknown host!");
                        return ServerEvent::Error(Box::new(StringError { msg: "Client Socket has received a packet from an unknown host!".to_string() }));
                    }
                    SocketEvent::Packet(packet) => {
                        if packet.addr() == self.address {
                            let msg = String::from_utf8_lossy(packet.payload());
                            return ServerEvent::Message(packet.addr(), msg.to_string());
                        } else {
                            println!("Unknown sender.");
                            return ServerEvent::Error(Box::new(StringError { msg: "Unknown sender.".to_string() }));
                        }
                    }
                    SocketEvent::Timeout(address) => {

                        return ServerEvent::Disconnection(address);
                    }
                }
            }
            Err(error) => {
                return ServerEvent::Error(Box::new(error));
            }
        }
    }

    fn get_sender(&mut self) -> MessageSender {
        return MessageSender::new(self.address, self.sender.clone());
    }
}

// In Receive
//                        let server_handshake_str = "server-handshake-response".to_string();
//                        if msg.eq(server_handshake_str.as_str()) {
//                            if !self.server_socket.as_ref().unwrap().connected {
//                                self.server_socket.as_mut().unwrap().connected = true;
//                                (self.connect_function.as_ref().unwrap())(self.server_socket.as_ref().unwrap());
//                            }
//                        }
//                        else {
//                            (self.receive_function.as_ref().unwrap())(self.server_socket.as_ref().unwrap(), &msg);
//                        }


// In Disconnect
//                        if self.server_socket.as_ref().unwrap().connected {
//                            self.server_socket.as_mut().unwrap().connected = false;
//                            (self.disconnect_function.as_ref().unwrap())();
//                        }