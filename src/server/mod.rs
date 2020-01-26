
mod socket;

use crate::server::socket::{ServerSocket, ServerSocketImpl};
use std::net::SocketAddr;
use crate::shared::GaiaMessage;
const SERVER_ADDR: &str = "127.0.0.1:12351";

pub struct Server {
    //socket: ServerSocketImpl
}

impl Server {
    pub fn new() -> Server { //args should take a shared config, and a port

        println!("Server New!");

        let mut server_socket = ServerSocketImpl::new();

        /* Server listens at some port
3. Server has a receive_message() callback
4. inside of receive_message() callback, it echoes back to client the same message with some appended thang*/

        server_socket.on_receive(|client_socket, msg| {
            println!("real. Received {:?} from {:?}", msg, client_socket.ip);
        });

        server_socket.listen(SERVER_ADDR);

        Server {
            //socket: server_socket
        }
    }

    pub fn update(&mut self) {

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
