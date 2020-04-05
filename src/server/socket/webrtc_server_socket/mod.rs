use crate::Result;
use crate::server::socket::ServerSocket;
pub use crate::user::User;
use super::client_socket::ClientSocket;
use std::net::IpAddr;

pub struct WebrtcServerSocket {
    connect_function: Option<Box<dyn Fn(&ClientSocket)>>,
    receive_function: Option<Box<dyn Fn(&ClientSocket, &str)>>,
    disconnect_function: Option<Box<dyn Fn(IpAddr)>>,
}

impl ServerSocket for WebrtcServerSocket {
    fn new() -> WebrtcServerSocket {
        println!("Hello WebrtcServerSocket!");

        let new_server_socket = WebrtcServerSocket {
            connect_function: None,
            receive_function: None,
            disconnect_function: None
        };

        new_server_socket
    }

    fn listen(&self, address: &str) {

    }

    fn on_connection(&mut self, func: impl Fn(&ClientSocket) + 'static) {
        self.connect_function = Some(Box::new(func));
    }

    fn on_receive(&mut self, func: impl Fn(&ClientSocket, &str) + 'static) {
        self.receive_function = Some(Box::new(func));
    }

    fn on_disconnection(&mut self, func: impl Fn(IpAddr) + 'static) {
        self.disconnect_function = Some(Box::new(func));
    }
}
