use crate::Result;
use crate::server::socket::Socket;
pub use crate::user::User;

pub struct FinalSocketBackend {

}

impl Socket for FinalSocketBackend {
    unsafe fn new() -> Result<FinalSocketBackend> {
        println!("Hello FinalSocketBackend!");
        Ok(FinalSocketBackend {})
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
