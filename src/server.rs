

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
        Server {}
    }

    pub fn onConnect(&mut self, func: fn()) { //function should have client, clientData, and callback?

    }

    pub fn onDisconnect(&mut self, func: fn()) { //function should have client

    }

    pub fn addEntity(&mut self) {

    }

    pub fn removeEntity(&mut self) {

    }

    pub fn getNextMessage(&mut self) {

    }

    pub fn update(&mut self) {

    }
}