
use gaia_client_socket::{ClientSocket, ClientSocketImpl, SocketEvent, MessageSender};

const PING_MSG: &str = "ping";
const PONG_MSG: &str = "pong";

pub struct App {
    client_socket: ClientSocketImpl,
    message_sender: Option<MessageSender>,
}

impl App {
    pub fn new(server_socket_address: &str) -> App {
        let mut app = App {
            client_socket: ClientSocketImpl::bind(&server_socket_address),
            message_sender: None,
        };

        app.message_sender = Some(app.client_socket.get_sender());

        app
    }

    pub fn update(&mut self) {
        match self.client_socket.receive() {
            SocketEvent::Connection() => {
                info!("Client connected to: {}", self.client_socket.server_address());
                self.message_sender.as_mut().unwrap().send(PING_MSG.to_string())
                    .expect("send error");
            }
            SocketEvent::Disconnection() => {
                info!("Client disconnected from: {}", self.client_socket.server_address());
            }
            SocketEvent::Message(message) => {
                info!("Client recv: {}", message);

                if message.eq(&PONG_MSG.to_string()) {
                    let to_server_message: String = PING_MSG.to_string();
                    info!("Client send: {}", to_server_message);
                    self.message_sender.as_mut().unwrap().send(to_server_message)
                        .expect("send error");
                }
            }
            SocketEvent::Error(error) => {
                info!("Client error: {}", error);
            }
            SocketEvent::None => {
                //info!("Client no event");
            }
        }
    }
}