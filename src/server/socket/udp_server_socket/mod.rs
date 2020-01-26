use crate::Result;
use crate::server::socket::ServerSocket;
use super::client_socket::ClientSocket;

use std::io::stdin;
use std::thread;
use std::time::Instant;

use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent};
use crossbeam_channel::{self, unbounded, Receiver, SendError, Sender, TryRecvError};

/////

pub struct UdpServerSocket {
    receive_function: fn(ClientSocket, &str)
}

impl ServerSocket for UdpServerSocket {
    fn new() -> UdpServerSocket {
        println!("Hello UdpServerSocket!");

        let new_server_socket = UdpServerSocket {
            receive_function: |client_socket, msg| { println!("default. Received {:?} from {:?}", msg, client_socket.ip); }
        };

        new_server_socket
    }

    fn listen(&self, address: &str) {
        let mut socket = LaminarSocket::bind(address).unwrap();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

        let _thread = thread::spawn(move || socket.start_polling());

        loop {
            if let Ok(event) = receiver.recv() {
                match event {
                    SocketEvent::Packet(packet) => {
                        let msg1 = packet.payload();

                        let msg = String::from_utf8_lossy(msg1);
                        let ip = packet.addr().ip();

                        println!("1. Received {:?} from {:?}", msg, ip);

                        (self.receive_function)(ClientSocket { ip }, &msg);

//                            sender
//                                .send(LaminarPacket::reliable_unordered(
//                                    packet.addr(),
//                                    "Copy that!".as_bytes().to_vec(),
//                                ))
//                                .expect("This should send");
                    }
                    SocketEvent::Timeout(address) => {
                        println!("Client timed out: {}", address);
                    }
                    _ => {}
                }
            }
        }
    }

    fn on_connection(&self, func: fn(ClientSocket)){

    }

    fn on_disconnection(&self, func: fn(ClientSocket)) {

    }

    fn on_receive(&mut self, func: fn(ClientSocket, &str)) {
        self.receive_function = func;
    }

    fn on_error(&self, func: fn(&str)) {

    }
}
