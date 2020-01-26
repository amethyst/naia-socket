use crate::Result;
use crate::client::socket::ClientSocket;

use std::io::stdin;
use std::thread;
use std::time::Instant;

use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent, Config as LaminarConfig};
use std::{time};

pub struct UdpClientSocket {
    receive_function: Box<dyn Fn(&str)>
}

impl ClientSocket for UdpClientSocket {
    fn new() -> UdpClientSocket {
        println!("Hello UdpClientSocket!");

        let new_client_socket = UdpClientSocket {
            receive_function: Box::new(|msg| { println!("default. Received {:?}", msg); })
        };

        new_client_socket
    }

    fn connect(&self, address: &str) {
        let mut config = LaminarConfig::default();
        config.heartbeat_interval = Option::Some(time::Duration::from_millis(500));
        let mut socket = LaminarSocket::bind_with_config("127.0.0.1:12352", config).unwrap();
        println!("Connected on {}", "127.0.0.1:12352");

        let server = address.parse().unwrap();
        let line: String = "yo".to_string();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
        let _thread = thread::spawn(move || socket.start_polling());

        println!("Client sending 'yo'");
        sender.send(LaminarPacket::reliable_unordered(
            server,
            line.clone().into_bytes(),
        ));

        loop {
            if let Ok(event) = receiver.recv() {
                match event {
                    SocketEvent::Packet(packet) => {
                        if packet.addr() == server {
                            let msg1 = packet.payload();

                            let msg = String::from_utf8_lossy(msg1);
                            let ip = packet.addr().ip();

                            (self.receive_function)(&msg);
                        } else {
                            println!("Unknown sender.");
                        }
                    }
                    SocketEvent::Timeout(_) => {}
                        _ => println!("Server disconnected.."),
                }
            }
        }
    }

    fn on_connection(&self, func: fn()) {

    }

    fn on_disconnection(&self, func: fn()) {

    }

    fn on_receive(&mut self, func: impl Fn(&str) + 'static) {
        self.receive_function = Box::new(func);
    }

    fn on_error(&self, func: fn(&str)) {

    }

    fn disconnect(&self) {

    }

    fn send(&self, msg: &str) {

    }
}
