use crate::Result;
use crate::client::socket::{Sender, ClientSocket};

use std::io::stdin;
use std::thread;
use std::time::Instant;
use std::net::SocketAddr;
use log::error;

use crossbeam_channel::{Sender as ChannelSender, Receiver as ChannelReceiver};
use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent, Config as LaminarConfig};
use std::{time};

struct ClientSender {
    send_function: Box<dyn Fn(&str)>
}

impl ClientSender {
    fn new(func: impl Fn(&str) + 'static) -> ClientSender {
        ClientSender {
            send_function: Box::new(func)
        }
    }
}

impl Sender for ClientSender {
    fn send(&self, msg: &str) {
        (self.send_function)(msg);
    }
}

pub struct UdpClientSocket {
    connect_function: Box<dyn Fn(&Sender)>,
    receive_function: Box<dyn Fn(&Sender, &str)>,
}

impl ClientSocket for UdpClientSocket {
    fn new() -> UdpClientSocket {
        println!("Hello UdpClientSocket!");

        let new_client_socket = UdpClientSocket {
            connect_function: Box::new(|sender| { println!("default. Connected!"); }),
            receive_function: Box::new(|sender, msg| { println!("default. Received {:?}", msg); })
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
        let _thread = thread::spawn(move || socket.start_polling());

        //Trying to wrap the crossbeam ChannelSender struct into my own ClientSender trait...
        let cloned_server: SocketAddr = server.clone();
        let cloned_sender = sender.clone();
        let new_sender: ClientSender = ClientSender::new(move |msg| {
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
                                (self.connect_function)(&new_sender);
                            }
                            else {
                                (self.receive_function)(&new_sender, &msg);
                            }
                        } else {
                            println!("Unknown sender.");
                        }
                    }
                    SocketEvent::Timeout(address) => {
                        println!("Server disconnected..");
                    }
                }
            }
        }
    }

    fn on_connection(&mut self, func: impl Fn(&Sender) + 'static) {
        self.connect_function = Box::new(func);
    }

    fn on_disconnection(&self, func: fn()) {

    }

    fn on_receive(&mut self, func: impl Fn(&Sender, &str) + 'static) {
        self.receive_function = Box::new(func);
    }

    fn disconnect(&self) {

    }

    fn send(&self, msg: &str) {

    }
}
