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

    fn on_connection(&self, func: fn(User)){

    }

    fn on_disconnection(&self, func: fn(User)) {

    }

    fn on_receive(&self, func: fn(User, &str)) {

    }

    fn on_error(&self, func: fn(&str)) {

    }

    fn listen<S>(&self, address: &str) {

    }
}
