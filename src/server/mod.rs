
mod socket;

use crate::server::socket::{ServerSocket, ServerSocketImpl};
use std::net::SocketAddr;
use crate::shared::GaiaMessage;

pub struct Server {
    socket: ServerSocketImpl
}

impl Server {
    pub fn new() -> Server { //args should take a shared config, and a port

        println!("Server New!");

        let server_socket = ServerSocketImpl::new().unwrap();

        /* Server listens at some port
3. Server has a receive_message() callback
4. inside of receive_message() callback, it echoes back to client the same message with some appended thang*/

        server_socket.on_receive(|client_socket, message| {

        });

        Server {
            socket: server_socket
        }
    }

    pub fn update(&mut self) {
        self.socket.update();
    }

    pub fn connect(&self, listen_addr: SocketAddr) { //put a port in here..

    }

    pub fn on_connect(&self, func: fn()) { //function should have client, clientData, and callback?

    }

    pub fn on_disconnect(&self, func: fn()) { //function should have client

    }

    pub fn add_object(&self) {

    }

    pub fn remove_object(&self) {

    }

    pub fn send_message(&self) {

    }

    pub fn receive_message(&self) {
    }
}
