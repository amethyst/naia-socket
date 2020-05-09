
use gaia_socket::{ClientSocket, ClientSocketImpl, SocketEvent};
use crate::internal_shared::find_my_ip_address;

use crate::internal_shared::SERVER_PORT;

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
                    //sender.send("just one extra post-connect message...".to_string());
                }
                SocketEvent::Disconnection(address) => {
                    println!("Client disconnected");
                }
                SocketEvent::Message(address, message) => {
                    println!("Client received: {:?}", message);
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

    pub fn update(&mut self) {

    }

    pub fn on_connect(&mut self, func: fn()) {

    }

    pub fn on_disconnect(&mut self, func: fn()) {

    }

    pub fn connect(&mut self) {

    }

    pub fn queue_message(&mut self) {

    }

    pub fn receive(&mut self) {

    }
}
