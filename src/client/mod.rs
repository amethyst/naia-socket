
use gaia_socket::{ClientSocket, ClientSocketImpl, ServerEvent};
use crate::internal_shared::find_my_ip_address;

use crate::internal_shared::SERVER_PORT;

pub struct Client {
    //socket: ClientSocketImpl
}

impl Client {
    pub fn new() -> Client {

        let current_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;
        let mut client_socket = ClientSocketImpl::bind(current_socket_address.as_str());

        println!("Connecting to server at: {}", current_socket_address);

        let mut sender = client_socket.get_sender();

        sender.send("just one extra post-connect message...".to_string());

        loop {
            match client_socket.receive() {
                ServerEvent::Connection(address) => {
                    println!("Client on_connection()");
                }
                ServerEvent::Disconnection(address) => {
                    println!("Client on_disconnection()");
                }
                ServerEvent::Message(address, message) => {
                    println!("Client on_receive(): {:?}", message);
                }
                ServerEvent::Error(error) => {}
                ServerEvent::None => {
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
