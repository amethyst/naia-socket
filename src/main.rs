
use std::net::{SocketAddr};

use gaia_socket::server::{ServerSocket, ServerSocketImpl, SocketEvent};
use gaia_socket::shared::{find_my_ip_address};

use crate::internal_shared;

fn main() {
    internal_shared::init();

    let current_socket_address = find_my_ip_address::get() + ":" + internal_shared::SERVER_PORT;

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

                if message.eq(internal_shared::PING_MSG) {
                    let to_client_message: String = internal_shared::PONG_MSG.to_string();
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