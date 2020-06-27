
use log::{info};

use naia_client_socket::{ClientSocket, SocketEvent, MessageSender, Config, Packet};

const PING_MSG: &str = "ping";
const PONG_MSG: &str = "pong";

const SERVER_PORT: &str = "14191";

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        const SERVER_IP_ADDRESS: &str = "192.168.1.9"; // Put your Server's IP Address here!, can't easily find this automatically from the browser
    } else {
        const SERVER_IP_ADDRESS: &str = find_my_ip_address::get();
    }
}

pub struct App {
    client_socket: ClientSocket,
    message_sender: MessageSender,
    message_count: u8,
}

impl App {
    pub fn new() -> App {

        info!("Naia Client Socket Example Started");

        let server_socket_address = format!("{}:{}", SERVER_IP_ADDRESS, SERVER_PORT);

        let mut client_socket = ClientSocket::connect(&server_socket_address, Some(Config::default()));
        let mut message_sender = client_socket.get_sender();

        message_sender.send(Packet::new(
            PING_MSG.to_string().into_bytes(),
        )).unwrap();

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
                    SocketEvent::Packet(packet) => {

                        let message = String::from_utf8_lossy(packet.payload());
                        info!("Client recv: {}", message);

                        if message.eq(PONG_MSG) && self.message_count < 10 {
                            self.message_count += 1;
                            let to_server_message: String = PING_MSG.to_string();
                            info!("Client send: {}", to_server_message);
                            self.message_sender.send(Packet::new(
                                to_server_message.into_bytes(),
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