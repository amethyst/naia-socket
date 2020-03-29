use crate::Result;
use crate::client::socket::{ClientSocket};
use super::server_socket::ServerSocket;

use std::io::stdin;
use std::thread;
use std::time::Instant;
use std::net::SocketAddr;
use log::error;

use crossbeam_channel::{Sender as ChannelSender, Receiver as ChannelReceiver};
use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent, Config as LaminarConfig};
use std::{time};

pub struct UdpClientSocket {
    connect_function: Box<dyn Fn(&ServerSocket)>,
    receive_function: Box<dyn Fn(&ServerSocket, &str)>,
    disconnect_function: Box<dyn Fn()>,
}

impl ClientSocket for UdpClientSocket {
    fn new() -> UdpClientSocket {
        println!("Hello UdpClientSocket!");

        let new_client_socket = UdpClientSocket {
            connect_function: Box::new(|server_socket| { println!("default. Connected!"); }),
            receive_function: Box::new(|server_socket, msg| { println!("default. Received {:?}", msg); }),
            disconnect_function: Box::new(|| { println!("default. Disconnected :("); })
        };

        new_client_socket
    }

    fn connect(&self, address: &str) {
        let mut config = LaminarConfig::default();
        config.heartbeat_interval = Option::Some(time::Duration::from_millis(500));
        let mut socket = LaminarSocket::bind_with_config("127.0.0.1:12352", config).unwrap();
        println!("ClientSocket.connect() {}", "127.0.0.1:12352");

        let server: SocketAddr = address.parse().unwrap();

        let sender: ChannelSender<LaminarPacket> = socket.get_packet_sender();

        let receiver: ChannelReceiver<SocketEvent> = socket.get_event_receiver();
        let poll_thread = thread::spawn(move || socket.start_polling());

        //Trying to wrap the crossbeam ChannelSender struct into my own ClientSender trait...
        let cloned_server: SocketAddr = server.clone();
        let cloned_sender = sender.clone();
        let server_socket: ServerSocket = ServerSocket::new(move |msg| {
            let msg_string: String = msg.to_string();
            let packet = LaminarPacket::reliable_unordered(
                cloned_server,
                msg_string.clone().into_bytes()
            );
            cloned_sender.send(packet);
        });
        ///////

        //Send initial server handshake
        let line: String = "client-handshake-request".to_string();
        println!("Client sending 'client-handshake-request'");
        sender.send(LaminarPacket::reliable_unordered(
            server,
            line.clone().into_bytes(),
        ));
        ///////

        loop {
            if let Ok(event) = receiver.recv() {
                match event {
                    SocketEvent::Connect(address) => {
                        // SHOULD NOT EVER GET HERE!
                        error!("Client Socket has received a packet from an unknown host!");
                    }
                    SocketEvent::Packet(packet) => {
                        if packet.addr() == server {
                            let msg1 = packet.payload();

                            let msg = String::from_utf8_lossy(msg1);
                            let ip = packet.addr().ip();

                            let server_handshake_str = "server-handshake-response".to_string();
                            if msg.eq(server_handshake_str.as_str()) {
                                (self.connect_function)(&server_socket);
                            }
                            else {
                                (self.receive_function)(&server_socket, &msg);
                            }
                        } else {
                            println!("Unknown sender.");
                        }
                    }
                    SocketEvent::Timeout(address) => {
                        (self.disconnect_function)();
                    }
                }
            }
        }
    }

    fn send(&self, msg: &str) {

    }

    fn on_connection(&mut self, func: impl Fn(&ServerSocket) + 'static) {
        self.connect_function = Box::new(func);
    }

    fn on_receive(&mut self, func: impl Fn(&ServerSocket, &str) + 'static) {
        self.receive_function = Box::new(func);
    }

    fn on_disconnection(&mut self, func: impl Fn() + 'static) {
        self.disconnect_function = Box::new(func);
    }
}
