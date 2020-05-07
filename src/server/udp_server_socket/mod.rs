
use std::io::stdin;
use std::thread;
use std::time::Instant;

use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent, Config as LaminarConfig};
use crossbeam_channel::{self, Receiver, Sender};
use std::{time};
use std::net::IpAddr;

use async_trait::async_trait;

use crate::server::{ServerSocket};
use super::client_message::ClientMessage;
use super::client_event::ClientEvent;
use super::message_sender::MessageSender;

/////

pub struct UdpServerSocket {
    sender: Sender<LaminarPacket>,
    receiver: Receiver<SocketEvent>
}

#[async_trait]
impl ServerSocket for UdpServerSocket {
    async fn bind(address: &str) -> UdpServerSocket {
        println!("Hello UdpServerSocket!");

        let mut config = LaminarConfig::default();
        config.heartbeat_interval = Option::Some(time::Duration::from_millis(500));
        let mut socket = LaminarSocket::bind_with_config(address, config).unwrap();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

        let _thread = thread::spawn(move || socket.start_polling());

        UdpServerSocket {
            sender,
            receiver,
        }
    }

    async fn receive(&mut self) -> ClientEvent {
        match self.receiver.recv() {
            Ok(event) => {
                match event {
                    SocketEvent::Connect(packet_addr) => {
                        println!("Client connected: {}", packet_addr);
                        return ClientEvent::Connection(packet_addr);
                    }
                    SocketEvent::Packet(packet) => {
                        let msg = String::from_utf8_lossy(packet.payload());
                        return ClientEvent::Message(packet.addr(), msg.to_string());
                    }
                    SocketEvent::Timeout(address) => {
                        return ClientEvent::Disconnection(address);
                    }
                }
            }
            Err(err) => {
                // ?
                return ClientEvent::Error(Box::new(err));
            }
        }
    }

    fn get_sender(&mut self) -> MessageSender {
        return MessageSender::new(self.sender.clone());
    }
}
