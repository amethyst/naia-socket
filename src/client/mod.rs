
use gaia_socket::client::{ClientSocket, ClientSocketImpl, SocketEvent, MessageSender};
use gaia_socket::shared::{find_my_ip_address};

use crate::internal_shared::SERVER_PORT;
use crate::internal_shared::PING_MSG;
use crate::internal_shared::PONG_MSG;

use std::{thread, time};

pub struct Client {
    socket: ClientSocketImpl,
    sender: MessageSender,
}

impl Client {
    pub fn new() -> Client {

        let current_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;

        let mut client_socket = ClientSocketImpl::bind(current_socket_address.as_str());

        let message_sender = client_socket.get_sender();

        Client {
            socket: client_socket,
            sender: message_sender,
        }
    }

    pub fn update(&mut self) {
        match self.socket.receive() {
            SocketEvent::Connection(address) => {
                println!("Client connected to: {}", address);
                self.sender.send(PING_MSG.to_string());
            }
            SocketEvent::Disconnection(address) => {
                println!("Client disconnected");
            }
            SocketEvent::Message(address, message) => {
                println!("Client recv: {:?}", message);

                if message.eq(PONG_MSG) {
                    thread::sleep(time::Duration::from_millis(1000));
                    let to_server_message: String = PING_MSG.to_string();
                    println!("Client send: {}", to_server_message);
                    self.sender.send(to_server_message)
                        .expect("send error");
                }
            }
            SocketEvent::Error(error) => {
                println!("Client error: {}", error);
                //break;
            }
            SocketEvent::None => {
                println!("Client no event");
                //break;
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
