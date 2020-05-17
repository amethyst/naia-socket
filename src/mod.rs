
use std::net::{SocketAddr};

use gaia_socket::server::{ServerSocket, ServerSocketImpl, SocketEvent};
use gaia_socket::shared::{find_my_ip_address};

use crate::internal_shared;

pub struct Server {
    //socket: ServerSocketImpl
}

impl Server {
    pub async fn new() -> Server {



        Server {
            //socket: server_socket
        }
    }

//    pub fn update(&mut self) {
//
//    }
//
//    pub fn connect(&self, listen_addr: SocketAddr) { //put a port in here..
//
//    }
//
//    pub fn on_connect(&self, func: fn()) { //function should have client, clientData, and callback?
//
//    }
//
//    pub fn on_disconnect(&self, func: fn()) { //function should have client
//
//    }
//
//    pub fn add_object(&self) {
//
//    }
//
//    pub fn remove_object(&self) {
//
//    }
//
//    pub fn send_message(&self) {
//
//    }
//
//    pub fn receive_message(&self) {
//    }
}
