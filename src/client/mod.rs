
mod socket;
use crate::client::socket::{ClientSocket, ClientSocketImpl};

pub struct Client {
    socket: ClientSocketImpl
}

impl Client {
    pub fn new() -> Client {

        println!("Client New!");

        Client {
            socket: ClientSocketImpl::new().unwrap()
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
