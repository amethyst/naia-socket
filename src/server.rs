

//use crate::gaia_data_transport::server::{ Server, ServerImpl, instance, set_instance};
//
//pub fn main() {
//    unsafe { set_instance(ServerImpl::new().unwrap()) };
//}
//
//pub(crate) fn server() -> &'static mut ServerImpl {
//    unsafe { instance() }
//}

pub struct Server {

}

impl Server {
    pub fn new() -> Server { //args should take a shared config, and a port
        println!("hello server!");
        Server {}
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
