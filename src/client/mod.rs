
mod socket;
use crate::client::socket::{ClientSocket, ClientSocketImpl};

const SERVER_ADDR: &str = "127.0.0.1:12351";

pub struct Client {
    //socket: ClientSocketImpl
}

impl Client {
    pub fn new() -> Client {

        println!("Client New!");

        let mut client_socket = ClientSocketImpl::new();

        client_socket.on_connection(|sender| {
            println!("Client on_connection(), sending hello server!");
            let msg: String = "hello server!".to_string();
            sender.send(msg.as_str());
        });

        client_socket.on_receive(|sender, msg| {
            println!("Client on_receive(): {:?}", msg);

            //let response_msg = msg.to_owned() + "|";

            //sender.send(response_msg.as_str());
        });

        client_socket.connect(SERVER_ADDR);

        Client {
            //socket: server_socket
        }
    }

    pub fn update(&self) { // Maybe clients don't need update functions eventually...
        println!("Client Update!");
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
