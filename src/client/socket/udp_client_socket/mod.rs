
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
use std::borrow::Borrow;

pub struct UdpClientSocket {
    client_socket: Option<LaminarSocket>,
    server_socket: Option<Box<ServerSocket>>,
    connect_function: Option<Box<dyn Fn(&ServerSocket)>>,
    receive_function: Option<Box<dyn Fn(&ServerSocket, &str)>>,
    disconnect_function: Option<Box<dyn Fn()>>,
}

impl ClientSocket for UdpClientSocket {
    fn new() -> UdpClientSocket {
        println!("Hello UdpClientSocket!");

        let new_client_socket = UdpClientSocket {
            client_socket: None,
            server_socket: None,
            connect_function: None,
            receive_function: None,
            disconnect_function: None
        };

        new_client_socket
    }

    fn connect(&mut self, address: &str) {
        let mut config = LaminarConfig::default();
        config.heartbeat_interval = Option::Some(time::Duration::from_millis(500));
        self.client_socket = Some(LaminarSocket::bind_with_config("127.0.0.1:12352", config).unwrap());
        println!("ClientSocket.connect() {}", "127.0.0.1:12352");

        let server_address: SocketAddr = address.parse().unwrap();
        let sender: ChannelSender<LaminarPacket> = self.client_socket.as_ref().unwrap().get_packet_sender();

        //Trying to wrap the crossbeam ChannelSender struct into my own ServerSocket impl...
        let cloned_address: SocketAddr = server_address.clone();
        let cloned_sender = sender.clone();
        let server_socket: ServerSocket = ServerSocket::new(cloned_address, move |msg| {
            let msg_string: String = msg.to_string();
            let packet = LaminarPacket::reliable_unordered(
                cloned_address,
                msg_string.clone().into_bytes()
            );
            cloned_sender.send(packet);
        });
        self.server_socket = Some(Box::new(server_socket));
        ///////

        //Send initial server handshake
        let line: String = "client-handshake-request".to_string();
        println!("Client sending 'client-handshake-request'");
        sender.send(LaminarPacket::reliable_unordered(
            server_address,
            line.clone().into_bytes(),
        ));
        ///////

        self.client_socket.as_mut().unwrap().manual_poll(Instant::now());
    }

    fn update(&mut self) {
        self.server_socket.as_mut().unwrap().process_send_queue();

        self.client_socket.as_mut().unwrap().manual_poll(Instant::now());

        while let Some(event) = self.client_socket.as_mut().unwrap().recv() {
            match event {
                SocketEvent::Connect(address) => {
                    // SHOULD NOT EVER GET HERE!
                    error!("Client Socket has received a packet from an unknown host!");
                }
                SocketEvent::Packet(packet) => {
                    if packet.addr() == self.server_socket.as_ref().unwrap().address {
                        let msg = String::from_utf8_lossy(packet.payload());

                        let server_handshake_str = "server-handshake-response".to_string();
                        if msg.eq(server_handshake_str.as_str()) {
                            if !self.server_socket.as_ref().unwrap().connected {
                                self.server_socket.as_mut().unwrap().connected = true;
                                (self.connect_function.as_ref().unwrap())(self.server_socket.as_ref().unwrap());
                            }
                        }
                        else {
                            (self.receive_function.as_ref().unwrap())(self.server_socket.as_ref().unwrap(), &msg);
                        }
                    } else {
                        println!("Unknown sender.");
                    }
                }
                SocketEvent::Timeout(address) => {
                    if self.server_socket.as_ref().unwrap().connected {
                        self.server_socket.as_mut().unwrap().connected = false;
                        (self.disconnect_function.as_ref().unwrap())();
                    }
                }
            }
        }
    }

    fn send(&mut self, msg: &str) {
        if self.server_socket.as_ref().unwrap().connected {
            self.server_socket.as_ref().unwrap().send(msg);
        }
        else {
            self.server_socket.as_mut().unwrap().add_to_send_queue(msg);
        }
    }

    fn on_connection(&mut self, func: impl Fn(&ServerSocket) + 'static) {
        self.connect_function = Some(Box::new(func));
    }

    fn on_receive(&mut self, func: impl Fn(&ServerSocket, &str) + 'static) {
        self.receive_function = Some(Box::new(func));
    }

    fn on_disconnection(&mut self, func: impl Fn() + 'static) {
        self.disconnect_function = Some(Box::new(func));
    }
}
