
mod socket;
use crate::client::socket::{ Socket, SocketImpl, instance, set_instance };

pub struct Client {

}

impl Client {
    pub fn new() -> Client {

        println!("hello client!");

        unsafe { set_instance(SocketImpl::new().unwrap()) };

        Client {}
    }

    pub fn on_connect(&mut self, func: fn()) {

    }

    pub fn on_disconnect(&mut self, func: fn()) {

    }

    pub fn connect(&mut self) {

    }

    pub fn send_message(&mut self) {

    }

    pub fn update(&mut self) {

    }

    pub fn receive(&mut self) {

    }
}
