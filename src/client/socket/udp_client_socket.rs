use crate::Result;
use crate::client::socket::ClientSocket;

pub struct UdpClientSocket {

}

impl ClientSocket for UdpClientSocket {
    fn new() -> Result<UdpClientSocket> {
        println!("Hello UdpClientSocket!");
        Ok(UdpClientSocket {})
    }

    fn on_connection(&self, func: fn()) {

    }

    fn on_disconnection(&self, func: fn()) {

    }

    fn on_receive(&self, func: fn(&str)) {

    }

    fn on_error(&self, func: fn(&str)) {

    }

    fn connect<S>(&self, address: &str) {

    }

    fn disconnect(&self) {

    }

    fn send<S>(&self, msg: &str) {

    }
}
