use crate::Result;
use crate::server::socket::ServerSocket;
pub use crate::user::User;

pub struct WsServerSocket {

}

impl ServerSocket for WsServerSocket {
    fn new() -> Result<WsServerSocket> {
        println!("Hello WsServerSocket!");
        Ok(WsServerSocket {})
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
