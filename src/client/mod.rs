
use std::{thread, time};

use gaia_socket::client::{ClientSocket, ClientSocketImpl, SocketEvent, MessageSender};

#[cfg(feature = "UdpClient")]
use gaia_socket::shared::{find_my_ip_address};

use crate::internal_shared;

pub struct Client {
    socket: ClientSocketImpl,
    sender: MessageSender,
}

impl Client {
    pub fn new() -> Client {

        // Uncomment the line below to enable logging. You don't need it if something else (e.g. quicksilver) is logging for you
        //internal_shared::init();

        #[cfg(feature = "UdpClient")]
        let current_socket_string = find_my_ip_address::get() + ":" + internal_shared::SERVER_PORT;

        #[cfg(feature = "UdpClient")]
        let current_socket_address = current_socket_string.as_str();

        #[cfg(feature = "WebrtcClient")]
        let current_socket_address = "192.168.1.5/3179";

        let mut client_socket = ClientSocketImpl::bind(current_socket_address);

        let message_sender = client_socket.get_sender();

        Client {
            socket: client_socket,
            sender: message_sender,
        }
    }

    pub fn update(&mut self) {
        match self.socket.receive() {
            SocketEvent::Connection() => {
                info!("Client connected to: {}", self.socket.server_address());
                self.sender.send(internal_shared::PING_MSG.to_string())
                    .expect("send error");
            }
            SocketEvent::Disconnection() => {
                info!("Client disconnected from: {}", self.socket.server_address());
            }
            SocketEvent::Message(message) => {
                info!("Client recv: {}", message);

                if message.eq(internal_shared::PONG_MSG) {
//                    thread::sleep(time::Duration::from_millis(1000));
                    let to_server_message: String = internal_shared::PING_MSG.to_string();
                    info!("Client send: {}", to_server_message);
                    self.sender.send(to_server_message)
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
//
//    pub fn on_connect(&mut self, func: fn()) {
//
//    }
//
//    pub fn on_disconnect(&mut self, func: fn()) {
//
//    }
//
//    pub fn connect(&mut self) {
//
//    }
//
//    pub fn queue_message(&mut self) {
//
//    }
//
//    pub fn receive(&mut self) {
//
//    }
}
