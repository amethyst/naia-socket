
mod socket;

use crate::server::socket::{ServerSocket, ServerSocketImpl, instance, set_instance };
use std::net::SocketAddr;
use crate::shared::GaiaMessage;

pub struct Server {

}

impl Server {
    pub fn new() -> Server { //args should take a shared config, and a port

        println!("hello server!");

        unsafe { set_instance(ServerSocketImpl::new().unwrap()) };

        Server {}
    }

    pub fn connect(&self, listen_addr: SocketAddr) { //put a port in here..

    }

    pub fn on_connect(&self, func: fn()) { //function should have client, clientData, and callback?

    }

    pub fn on_disconnect(&self, func: fn()) { //function should have client

    }

    pub fn add_object(&self) {

    }

    pub fn remove_object(&self) {

    }

    pub fn send_message(&self) {

    }

    pub fn receive_message(&self) -> Option<Box<dyn GaiaMessage>> {
        let msg = Ga
    }

    pub fn update(&self) {

    }
}
