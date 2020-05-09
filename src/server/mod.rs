
use std::net::{SocketAddr};

use gaia_socket::server::{ServerSocket, ServerSocketImpl, SocketEvent};
use gaia_socket::shared::{find_my_ip_address};

use crate::internal_shared::SERVER_PORT;

pub struct Server {
    //socket: ServerSocketImpl
}

impl Server {
    pub async fn new() -> Server {

        let current_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;

        let mut server_socket = ServerSocketImpl::bind(current_socket_address.as_str()).await;

        let mut sender = server_socket.get_sender();

        loop {
            match server_socket.receive().await {
                SocketEvent::Connection(address) => {
                    println!("Server connected to: {}", address);
                }
                SocketEvent::Disconnection(address) => {
                    println!("Server disconnected from: {:?}", address);
                }
                SocketEvent::Message(address, message) => {
                    println!("Server received: {}", message);

                    println!("Server send(): {}", message);
                    sender.send((address, message))
                        .await.expect("send error");
                }
                SocketEvent::Tick => {

                }
                SocketEvent::Error(error) => {
                    println!("Server Error: {}", error);
                }
            }
        }

        Server {
            //socket: server_socket
        }
    }

//    pub fn update(&mut self) {
//
//    }
//
//    pub fn connect(&self, listen_addr: SocketAddr) { //put a port in here..
//
//    }
//
//    pub fn on_connect(&self, func: fn()) { //function should have client, clientData, and callback?
//
//    }
//
//    pub fn on_disconnect(&self, func: fn()) { //function should have client
//
//    }
//
//    pub fn add_object(&self) {
//
//    }
//
//    pub fn remove_object(&self) {
//
//    }
//
//    pub fn send_message(&self) {
//
//    }
//
//    pub fn receive_message(&self) {
//    }
}
