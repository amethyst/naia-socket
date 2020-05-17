
use std::{thread, time};

use gaia_client_socket::{ClientSocket, ClientSocketImpl, SocketEvent, MessageSender};

#[cfg(not(target_arch = "wasm32"))]
use gaia_socket_shared::{find_my_ip_address};

pub struct Client {
    socket: ClientSocketImpl,
    sender: MessageSender,
}

const SERVER_PORT: &str = "3179";
const PING_MSG: &str = "ping";
const PONG_MSG: &str = "pong";

pub fn main_common() {

    // Uncomment the line below to enable logging. You don't need it if something else (e.g. quicksilver) is logging for you
    set_logger(log::Level::Info);

    info!("Client Example Started");

    #[cfg(not(target_arch = "wasm32"))]
    let current_socket_string = find_my_ip_address::get() + ":" + SERVER_PORT;

    #[cfg(not(target_arch = "wasm32"))]
    let current_socket_address = current_socket_string.as_str();

    #[cfg(target_arch = "wasm32")]
    let current_socket_address = "192.168.1.5/3179";

    let mut client_socket = ClientSocketImpl::bind(current_socket_address);

    let mut message_sender = client_socket.get_sender();

    loop {
        match client_socket.receive() {
            SocketEvent::Connection() => {
                info!("Client connected to: {}", client_socket.server_address());
                message_sender.send(PING_MSG.to_string())
                    .expect("send error");
            }
            SocketEvent::Disconnection() => {
                info!("Client disconnected from: {}", client_socket.server_address());
            }
            SocketEvent::Message(message) => {
                info!("Client recv: {}", message);

                if message.eq(&PONG_MSG.to_string()) {
//                    thread::sleep(time::Duration::from_millis(1000));
                    let to_server_message: String = PING_MSG.to_string();
                    info!("Client send: {}", to_server_message);
                    message_sender.send(to_server_message)
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

fn set_logger(level: log::Level) {
    #[cfg(target_arch = "wasm32")]
    web_logger::custom_init(web_logger::Config { level });

    #[cfg(not(target_arch = "wasm32"))]
    simple_logger::init_with_level(level).expect("A logger was already initialized");
}