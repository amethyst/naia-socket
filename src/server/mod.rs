
mod socket;
use crate::server::socket::{ServerSocket, ServerSocketImpl, instance, set_instance };

pub struct Server {

}

impl Server {
    pub fn new() -> Server { //args should take a shared config, and a port

        println!("hello server!");

        unsafe { set_instance(ServerSocketImpl::new().unwrap()) };

        Server {}
    }

    pub fn connect(&mut self) { //put a port in here..

    }

    pub fn on_connect(&mut self, func: fn()) { //function should have client, clientData, and callback?

    }

    pub fn on_disconnect(&mut self, func: fn()) { //function should have client

    }

    pub fn add_object(&mut self) {

    }

    pub fn remove_object(&mut self) {

    }

    pub fn send_message(&mut self) {

    }

    pub fn receive_message(&mut self) {

    }

    pub fn update(&mut self) {

    }
}
