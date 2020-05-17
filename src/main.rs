
#[macro_use]
extern crate log;

use std::net::{SocketAddr};
use simple_logger;

use gaia_server_socket::{ServerSocket, ServerSocketImpl, SocketEvent};
use gaia_socket_shared::{find_my_ip_address};

const SERVER_PORT: &str = "3179";
const PING_MSG: &str = "ping";
const PONG_MSG: &str = "pong";

#[tokio::main]
async fn main() {

    simple_logger::init_with_level(log::Level::Info).expect("A logger was already initialized");

    let current_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;

    let mut server_socket = ServerSocketImpl::bind(current_socket_address.as_str()).await;

    let mut sender = server_socket.get_sender();

    loop {
        match server_socket.receive().await {
            SocketEvent::Connection(address) => {
                info!("Server connected to: {}", address);
            }
            SocketEvent::Disconnection(address) => {
                info!("Server disconnected from: {:?}", address);
            }
            SocketEvent::Message(address, message) => {
                info!("Server recv <- {}: {}", address, message);

                if message.eq(PING_MSG) {
                    let to_client_message: String = PONG_MSG.to_string();
                    info!("Server send -> {}: {}", address, to_client_message);
                    sender.send((address, to_client_message))
                        .await.expect("send error");
                }
            }
            SocketEvent::Tick => {

            }
            SocketEvent::Error(error) => {
                info!("Server Error: {}", error);
            }
        }
    }
}