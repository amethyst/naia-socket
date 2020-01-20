use crate::Result;
use crate::server::socket::ServerSocket;
use super::client_socket::ClientSocket;

pub struct UdpServerSocket {

}

impl ServerSocket for UdpServerSocket {
    fn new() -> Result<UdpServerSocket> {
        println!("Hello UdpServerSocket!");
        Ok(UdpServerSocket {})
    }

    fn on_connection(&self, func: fn(ClientSocket)){

    }

    fn on_disconnection(&self, func: fn(ClientSocket)) {

    }

    fn on_receive(&self, func: fn(ClientSocket, &str)) {

    }

    fn on_error(&self, func: fn(&str)) {

    }

    fn listen<S>(&self, address: &str) {

    }
}
