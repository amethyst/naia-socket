use crate::Result;
use crate::server::socket::ServerSocket;
use super::client_socket::ClientSocket;

use std::io::stdin;
use std::thread;
use std::time::Instant;

use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent};
use std::cell::RefCell;

const SERVER: &str = "127.0.0.1:12351";

/////


pub struct UdpServerSocket {
    laminar_socket: RefCellLaminarSocket,
    pub receive_function: fn(ClientSocket, &str)
}

impl ServerSocket for UdpServerSocket {
    fn new() -> Result<UdpServerSocket> {
        println!("Hello UdpServerSocket!");

        let mut socket = LaminarSocket::bind(SERVER).unwrap();

        let new_server_socket = UdpServerSocket {
            laminar_socket: socket,
            receive_function: |client_socket, msg| { println!("2. Received {:?} from {:?}", msg, client_socket.ip); }
        };

        let sender = socket.get_packet_sender();
        let receiver = socket.get_event_receiver();

        thread::spawn(move || {
            loop {
                if let Ok(event) = receiver.recv() {
                    match event {
                        SocketEvent::Packet(packet) => {
                            let msg1 = packet.payload();

                            let msg = String::from_utf8_lossy(msg1);
                            let ip = packet.addr().ip();

                            println!("1. Received {:?} from {:?}", msg, ip);

                            (new_server_socket.receive_function)(ClientSocket { ip }, &msg);

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
        });

        Ok(new_server_socket)
    }

    fn on_connection(&self, func: fn(ClientSocket)){

    }

    fn on_disconnection(&self, func: fn(ClientSocket)) {

    }

    fn on_receive(&self, func: fn(ClientSocket, &str)) {

    }

    fn on_error(&self, func: fn(&str)) {

    }

    fn listen(&self, address: &str) {

    }

    fn update(&mut self) {
        self.laminar_socket.manual_poll(Instant::now());
    }
}
