use crate::Result;
use crate::client::socket::ClientSocket;

use std::io::stdin;
use std::thread;
use std::time::Instant;

use laminar::{ErrorKind, Packet, Socket, SocketEvent};

pub struct UdpClientSocket {
    receive_function: fn(&str)
}

impl ClientSocket for UdpClientSocket {
    fn new() -> UdpClientSocket {
        println!("Hello UdpClientSocket!");

        let new_client_socket = UdpClientSocket {
            receive_function: |msg| { println!("default. Received {:?}", msg); }
        };

        new_client_socket
    }

    fn connect(&self, address: &str) {
        let mut socket = Socket::bind("127.0.0.1:12352").unwrap();
        println!("Connected on {}", "127.0.0.1:12352");

        let server = address.parse().unwrap();

        println!("Type a message and press Enter to send. Send `Bye!` to quit.");

        let stdin = stdin();
        let mut s_buffer = String::new();

        loop {
            s_buffer.clear();
            stdin.read_line(&mut s_buffer);
            let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");

            socket.send(Packet::reliable_unordered(
                server,
                line.clone().into_bytes(),
            ));

            socket.manual_poll(Instant::now());

            match socket.recv() {
                Some(SocketEvent::Packet(packet)) => {
                    if packet.addr() == server {
                        println!("Server sent: {}", String::from_utf8_lossy(packet.payload()));
                    } else {
                        println!("Unknown sender.");
                    }
                }
                Some(SocketEvent::Timeout(_)) => {}
                _ => println!("Silence.."),
            }
        }
    }

    fn on_connection(&self, func: fn()) {

    }

    fn on_disconnection(&self, func: fn()) {

    }

    fn on_receive(&mut self, func: fn(&str)) {
        self.receive_function = func;
    }

    fn on_error(&self, func: fn(&str)) {

    }

    fn disconnect(&self) {

    }

    fn send<S>(&self, msg: &str) {

    }
}
