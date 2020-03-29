use crate::Result;
use crate::server::socket::{ServerSocket};
use super::client_socket::ClientSocket;

use std::io::stdin;
use std::thread;
use std::time::Instant;

use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent, Config as LaminarConfig};
use crossbeam_channel::{self, unbounded, Receiver as ChannelReceiver, SendError, Sender as ChannelSender, TryRecvError};
use std::{time};

/////

pub struct UdpServerSocket {
    connect_function: Box<dyn Fn(&ClientSocket)>,
    receive_function: Box<dyn Fn(&ClientSocket, &str)>,
}

impl ServerSocket for UdpServerSocket {
    fn new() -> UdpServerSocket {
        println!("Hello UdpServerSocket!");

        let new_server_socket = UdpServerSocket {
            connect_function: Box::new(|client_socket| { println!("default. Connected!"); }),
            receive_function: Box::new(|client_socket, msg| { println!("default. Received {:?}", msg); })
        };

        new_server_socket
    }

    fn listen(&self, address: &str) {
        let mut config = LaminarConfig::default();
        config.heartbeat_interval = Option::Some(time::Duration::from_millis(500));
        let mut socket = LaminarSocket::bind_with_config(address, config).unwrap();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

        let _thread = thread::spawn(move || socket.start_polling());

        loop {
            if let Ok(event) = receiver.recv() {
                match event {
                    SocketEvent::Connect(address) => {
                        println!("Client connected: {}", address);

                        sender
                            .send(LaminarPacket::reliable_unordered(
                                address,
                                "server-handshake-response".as_bytes().to_vec(),
                            ))
                            .expect("This should send");

                        let cloned_sender = sender.clone();
                        let client_socket = ClientSocket::new(address.ip(), move |msg| {
                            let msg_string: String = msg.to_string();
                            let packet = LaminarPacket::reliable_unordered(
                                address,
                                msg_string.clone().into_bytes()
                            );
                            cloned_sender.send(packet);
                        });

                        (self.connect_function)(&client_socket);
                    }
                    SocketEvent::Packet(packet) => {
                        let packet_payload = packet.payload();
                        let packet_addr = packet.addr();
                        let packet_ip = packet_addr.ip();
                        let msg = String::from_utf8_lossy(packet_payload);

                        let cloned_sender = sender.clone();
                        let client_socket = ClientSocket::new(packet_ip, move |msg| {
                            let msg_string: String = msg.to_string();
                            let packet = LaminarPacket::reliable_unordered(
                                packet_addr,
                                msg_string.clone().into_bytes()
                            );
                            cloned_sender.send(packet);
                        });

                        (self.receive_function)(&client_socket, &msg);
                    }
                    SocketEvent::Timeout(address) => {
                        println!("Client disconnected: {}", address);
                    }
                }
            }
        }
    }

    fn on_connection(&mut self, func: impl Fn(&ClientSocket) + 'static) {
        self.connect_function = Box::new(func);
    }

    fn on_disconnection(&self, func: fn()) {

    }

    fn on_receive(&mut self, func: impl Fn(&ClientSocket, &str) + 'static) {
        self.receive_function = Box::new(func);
    }

    fn on_error(&self, func: fn(&str)) {

    }
}
