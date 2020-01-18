use crate::Result;
use crate::server::socket::ServerSocket;
pub use crate::user::User;

pub struct UdpServerSocket {

}

impl ServerSocket for UdpServerSocket {
    fn new() -> Result<UdpServerSocket> {
        println!("Hello UdpServerSocket!");
        Ok(UdpServerSocket {})
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
