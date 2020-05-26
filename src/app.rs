
use log::{info};

use gaia_client_socket::{ClientSocket, SocketEvent, MessageSender, Config, Packet};

const PING_MSG: &str = "ping";
const PONG_MSG: &str = "pong";

pub struct App {
    client_socket: ClientSocket,
    message_sender: MessageSender,
    message_count: u8,
}

impl App {
    pub fn new(server_socket_address: &str, config: Option<Config>) -> App {

        let mut client_socket = ClientSocket::connect(&server_socket_address, config);
        let message_sender = client_socket.get_sender();

        App {
            client_socket,
            message_sender,
            message_count: 0,
        }
    }

    pub fn update(&mut self) {
        match self.client_socket.receive() {
            Ok(event) => {
                match event {
                    SocketEvent::Connection => {
                        info!("Client connected to: {}", self.client_socket.server_address());
                        self.message_sender.send(Packet::new(
                            PING_MSG.to_string().into_bytes(),
                        ))
                            .expect("send error");
                    }
                    SocketEvent::Disconnection => {
                        info!("Client disconnected from: {}", self.client_socket.server_address());
                    }
                    SocketEvent::Packet(packet) => {

                        let message = String::from_utf8_lossy(packet.payload());
                        info!("Client recv: {}", message);

                        if message.eq(&PONG_MSG.to_string()) && self.message_count < 10 {
                            self.message_count += 1;
                            let to_server_message: String = PING_MSG.to_string();
                            info!("Client send: {}", to_server_message);
                            self.message_sender.send(Packet::new(
                                to_server_message.clone().into_bytes(),
                            ))
                                .expect("send error");
                        }
                    }
                    SocketEvent::None => {
                        //info!("Client non-event");
                    }
                }
            }
            Err(err) => {
                info!("Client Error: {}", err);
            }
        }
    }
}