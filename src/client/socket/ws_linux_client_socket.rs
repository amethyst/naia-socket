use crate::Result;
use crate::client::socket::ClientSocket;

pub struct WsLinuxClientSocket {}

impl ClientSocket for WsLinuxClientSocket {
    unsafe fn new() -> Result<WsLinuxClientSocket> {
        println!("Hello WsLinuxClientSocket!");
        Ok(WsLinuxClientSocket {})
    }

    unsafe fn on_connection(&mut self, func: fn()) {}

    unsafe fn on_disconnection(&mut self, func: fn()) {}

    unsafe fn on_receive(&mut self, func: fn(&str)) {}

    unsafe fn on_error(&mut self, func: fn(&str)) {}

    unsafe fn connect<S>(&mut self, address: &str) {}

    unsafe fn disconnect(&mut self) {}

    unsafe fn send<S>(&mut self, msg: &str) {}
}
