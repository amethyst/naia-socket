use crate::Result;
use crate::server::socket::ServerSocket;
use super::client_socket::ClientSocket;

use std::io::stdin;
use std::thread;
use std::time::Instant;

use laminar::{ErrorKind, Packet, Socket, SocketEvent};

const SERVER: &str = "127.0.0.1:12351";

/////


pub struct UdpServerSocket {

}

impl ServerSocket for UdpServerSocket {
    fn new() -> Result<UdpServerSocket> {
        println!("Hello UdpServerSocket!");

        let mut socket = Socket::bind(SERVER).unwrap();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
        let _thread = thread::spawn(move || socket.start_polling());

        loop {
            if let Ok(event) = receiver.recv() {
                match event {
                    SocketEvent::Packet(packet) => {
                        let msg = packet.payload();

                        if msg == b"Bye!" {
                            break;
                        }

                        let msg = String::from_utf8_lossy(msg);
                        let ip = packet.addr().ip();

                        println!("Received {:?} from {:?}", msg, ip);

                        sender
                            .send(Packet::reliable_unordered(
                                packet.addr(),
                                "Copy that!".as_bytes().to_vec(),
                            ))
                            .expect("This should send");
                    }
                    SocketEvent::Timeout(address) => {
                        println!("Client timed out: {}", address);
                    }
                    _ => {}
                }
            }
        }

        Ok(UdpServerSocket {})
    }

    fn on_connection(&self, func: fn(ClientSocket)){

    }

    fn on_disconnection(&self, func: fn(ClientSocket)) {

    }

    fn on_receive(&self, func: fn(ClientSocket, &str)) {

    }

    fn on_error(&self, func: fn(&str)) {

    }

    fn listen<S>(&self, address: &str) {

    }
}
