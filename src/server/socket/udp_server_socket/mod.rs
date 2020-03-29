use crate::Result;
use crate::server::socket::{ServerSocket};
use super::client_socket::ClientSocket;

use std::io::stdin;
use std::thread;
use std::time::Instant;

use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent, Config as LaminarConfig};
use crossbeam_channel::{self, unbounded, Receiver as ChannelReceiver, SendError, Sender as ChannelSender, TryRecvError};
use std::{time};
use std::net::IpAddr;

/////

pub struct UdpServerSocket {
    connect_function: Box<dyn Fn(&ClientSocket)>,
    receive_function: Box<dyn Fn(&ClientSocket, &str)>,
    disconnect_function: Box<dyn Fn(IpAddr)>,
}

impl ServerSocket for UdpServerSocket {
    fn new() -> UdpServerSocket {
        println!("Hello UdpServerSocket!");

        let new_server_socket = UdpServerSocket {
            connect_function: Box::new(|client_socket| { println!("default. Connected!"); }),
            receive_function: Box::new(|client_socket, msg| { println!("default. Received {:?}", msg); }),
            disconnect_function: Box::new(|ip_address| { println!("default. Disconnected {:?}", ip_address); })
        };

        new_server_socket
    }

    fn listen(&self, address: &str) {
        let mut config = LaminarConfig::default();
        config.heartbeat_interval = Option::Some(time::Duration::from_millis(500));
        let mut socket = LaminarSocket::bind_with_config(address, config).unwrap();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

        let _thread = thread::spawn(move || socket.start_polling());

        let sender_func_factory = |packet_addr, sender: ChannelSender<LaminarPacket>| {
          let output = move |msg: &str| {
              let msg_string: String = msg.to_string();
              let packet = LaminarPacket::reliable_unordered(
                  packet_addr,
                  msg_string.clone().into_bytes()
              );
              sender.send(packet);
          };

          output
        };

        loop {
            if let Ok(event) = receiver.recv() {
                match event {
                    SocketEvent::Connect(packet_addr) => {
                        println!("Client connected: {}", packet_addr);

                        sender
                            .send(LaminarPacket::reliable_unordered(
                                packet_addr,
                                "server-handshake-response".as_bytes().to_vec(),
                            ))
                            .expect("This should send");

                        let packet_ip = packet_addr.ip();
                        let client_socket = ClientSocket::new(
                            packet_ip,
                            sender_func_factory(packet_addr, sender.clone()));

                        (self.connect_function)(&client_socket);
                    }
                    SocketEvent::Packet(packet) => {
                        let packet_payload = packet.payload();
                        let packet_addr = packet.addr();
                        let packet_ip = packet_addr.ip();
                        let msg = String::from_utf8_lossy(packet_payload);

                        let client_socket = ClientSocket::new(
                            packet_ip,
                            sender_func_factory(packet_addr, sender.clone()));

                        (self.receive_function)(&client_socket, &msg);
                    }
                    SocketEvent::Timeout(address) => {
                        (self.disconnect_function)(address.ip());
                    }
                }
            }
        }
    }

    fn on_connection(&mut self, func: impl Fn(&ClientSocket) + 'static) {
        self.connect_function = Box::new(func);
    }

    fn on_disconnection(&mut self, func: impl Fn(IpAddr) + 'static) {
        self.disconnect_function = Box::new(func);
    }

    fn on_receive(&mut self, func: impl Fn(&ClientSocket, &str) + 'static) {
        self.receive_function = Box::new(func);
    }
}
