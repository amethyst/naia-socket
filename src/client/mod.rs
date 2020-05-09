
use gaia_socket::client::{ClientSocket, ClientSocketImpl, SocketEvent};
use gaia_socket::shared::{find_my_ip_address};

use crate::internal_shared::SERVER_PORT;
use crate::internal_shared::PING_MSG;
use crate::internal_shared::PONG_MSG;

use std::{thread, time};

pub struct Client {
    //socket: ClientSocketImpl
}

impl Client {
    pub fn new() -> Client {

        let current_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;
        let mut client_socket = ClientSocketImpl::bind(current_socket_address.as_str());

        let mut sender = client_socket.get_sender();

        loop {
            match client_socket.receive() {
                SocketEvent::Connection(address) => {
                    println!("Client connected to: {}", address);
                    sender.send(PING_MSG.to_string());
                }
                SocketEvent::Disconnection(address) => {
                    println!("Client disconnected");
                }
                SocketEvent::Message(address, message) => {
                    println!("Client received: {:?}", message);

                    if message.eq(PONG_MSG) {
                        thread::sleep(time::Duration::from_millis(1000));
                        let to_server_message: String = PING_MSG.to_string();
                        println!("Client send: {}", to_server_message);
                        sender.send(to_server_message)
                            .expect("send error");
                    }
                }
                SocketEvent::Error(error) => {
                    println!("Client error: {}", error);
                }
                SocketEvent::None => {
                    println!("Client no event");
                    //break;
                }
            }
        }

        Client {
            //socket: client_socket
        }
    }

//    pub fn update(&mut self) {
//
//    }
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
