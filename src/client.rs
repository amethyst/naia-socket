
//use crate::gaia_data_transport::client::{ Client, ClientImpl, instance, set_instance};
//
//pub fn main() {
//    unsafe { set_instance(ClientImpl::new().unwrap()) };
//}
//
//pub(crate) fn client() -> &'static mut ClientImpl {
//    unsafe { instance() }
//}

pub struct Client {

}

impl Client {
    pub fn new() -> Client {
        println!("hello client!");
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
