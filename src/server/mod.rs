
use std::net::{SocketAddr};
use futures_util::{SinkExt};

use gaia_socket::{ServerSocket, ServerSocketImpl, ClientEvent};
use crate::internal_shared::find_ip_address;

const DEFAULT_PORT: &str = "3179";

pub struct Server {
    //socket: ServerSocketImpl
}

impl Server {
    pub async fn new() -> Server { //args should take a shared config, and a port

        let current_socket_address = find_ip_address::get() + ":" + DEFAULT_PORT;
        println!("Webserver Listening on: {}", current_socket_address);

        let mut server_socket = ServerSocketImpl::bind(current_socket_address.as_str()).await;

        let mut sender = server_socket.get_sender();

        loop {
            match server_socket.receive().await {
                Ok(ClientEvent::Connection(address)) => {
                    println!("Server on_connection(), connected to {}", address);

                    let msg: String = "hello new client!".to_string();
                    if let Err(err) = sender.send(ClientEvent::Message(address, msg)).await {
                        println!("error!");
                    }
                }
                Ok(ClientEvent::Disconnection(address)) => {
                    println!("Server on_disconnection(): {:?}", address);
                }
                Ok(ClientEvent::Message(address, message)) => {
                    println!("Server on_receive(): {}", message);

                    println!("Server send(): {}", message);
                    if let Err(err) = sender.send(ClientEvent::Message(address, message)).await {
                        println!("error!");
                    }
                }
                Ok(ClientEvent::Tick) => {}
                Err(error) => {}
            }
        }

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
