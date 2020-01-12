use crate::Result;
use crate::server::socket::ServerSocket;
pub use crate::user::User;

pub struct WebrtcServerSocket {

}

impl ServerSocket for WebrtcServerSocket {
    unsafe fn new() -> Result<WebrtcServerSocket> {
        println!("Hello WebrtcServerSocket!");
        Ok(WebrtcServerSocket {})
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
