use crate::server::{ServerSocket};
use super::client_message::ClientMessage;

use std::io::stdin;
use std::thread;
use std::time::Instant;

use laminar::{ErrorKind, Packet as LaminarPacket, Socket as LaminarSocket, SocketEvent, Config as LaminarConfig};
use crossbeam_channel::{Sender as ChannelSender};
use std::{time};
use std::net::IpAddr;



/////

pub struct UdpServerSocket {
    connect_function: Option<Box<dyn Fn(&ClientMessage)>>,
    receive_function: Option<Box<dyn Fn(&ClientMessage)>>,
    disconnect_function: Option<Box<dyn Fn(&ClientMessage)>>,
}

impl ServerSocket for UdpServerSocket {
    fn new() -> UdpServerSocket {
        println!("Hello UdpServerSocket!");

        let new_server_socket = UdpServerSocket {
            connect_function: None,
            receive_function: None,
            disconnect_function: None
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

                        (self.connect_function.as_ref().unwrap())(&client_socket);
                    }
                    SocketEvent::Packet(packet) => {
                        let packet_payload = packet.payload();
                        let packet_addr = packet.addr();
                        let packet_ip = packet_addr.ip();
                        let msg = String::from_utf8_lossy(packet_payload);

                        let client_socket = ClientSocket::new(
                            packet_ip,
                            sender_func_factory(packet_addr, sender.clone()));

                        (self.receive_function.as_ref().unwrap())(&client_socket, &msg);
                    }
                    SocketEvent::Timeout(address) => {
                        let client_socket = ClientSocket::new(
                            address.ip(),
                            move |msg: &str| {});
                        (self.disconnect_function.as_ref().unwrap())(&client_socket);
                    }
                }
            }
        }
    }

    fn on_connection(&mut self, func: impl Fn(&ClientMessage) + 'static) {
        self.connect_function = Some(Box::new(func));
    }

    fn on_receive(&mut self, func: impl Fn(&ClientMessage, &str) + 'static) {
        self.receive_function = Some(Box::new(func));
    }

    fn on_error(&mut self, func: impl Fn(&ClientMessage, &str) + 'static) {
        unimplemented!()
    }

    fn on_disconnection(&mut self, func: impl Fn(&ClientMessage) + 'static) {
        self.disconnect_function = Some(Box::new(func));
    }

    fn get_sender(&mut self) -> ChannelSender<ClientMessage> {
        unimplemented!()
    }
}
