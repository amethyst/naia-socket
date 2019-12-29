use crate::Result;
use crate::server::socket::Socket;
pub use crate::user::User;

pub struct ProtoSocketBackend {

}

impl Socket for ProtoSocketBackend {
    unsafe fn new() -> Result<ProtoSocketBackend> {
        println!("Hello ProtoSocketBackend!");
        Ok(ProtoSocketBackend {})
    }

    unsafe fn on_connection(&mut self, func: fn(User)){

    }

    unsafe fn on_disconnection(&mut self, func: fn(User)) {

    }

    unsafe fn on_receive(&mut self, func: fn(User, &str)) {

    }

    unsafe fn on_error(&mut self, func: fn(&str)) {

    }

    unsafe fn listen<S>(&mut self, address: &str) {

    }
}
