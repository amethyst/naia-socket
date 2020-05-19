
use gaia_client_socket::{ClientSocket, ClientSocketImpl, SocketEvent, MessageSender};

///TODO: example should have a method, loop(func: &Closure<FnMut()>)
/// in Linux, this will create a blocking loop that repeatedly calls func
/// in Wasm, this will call func every request_animation_frame, from code below
///                 we need to do this because we can't just block the main thread of the browser
///                 since it's gotta process messages from the data channel
///
/// do client_socket.receive() stuff in a closure passed to this method

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

    fn update(&mut self) {
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

    pub fn start_loop(&mut self) {
        loop {
            self.update();
        }
    }
}