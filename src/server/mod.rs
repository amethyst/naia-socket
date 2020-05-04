
use gaia_socket::{ServerSocket, ServerSocketImpl, ClientMessage};
use std::net::{SocketAddr, IpAddr};
use std::sync::{Arc,Mutex};
use crate::internal_shared::find_ip_address;
use std::borrow::Borrow;

const DEFAULT_PORT: &str = "3179";

pub struct Server {
    //socket: ServerSocketImpl
}

impl Server {
    pub async fn new() -> Server { //args should take a shared config, and a port

        let mut server_socket = ServerSocketImpl::new();

        let server_socket_sender_1 = server_socket.get_sender();
        let server_socket_sender_2 = server_socket.get_sender();

        server_socket.on_connection(move |client_message| {
            println!("Server on_connection(), connected to {}", client_message.address);

            let msg: String = "hello new client!".to_string();
            server_socket_sender_1.send(ClientMessage::new(client_message.address, msg.as_str()));
        });

        server_socket.on_receive(move |client_message| {
            if let Some(message) = &client_message.message {
                println!("Server on_receive(): {}", message);

                let new_string = client_message.message.as_ref().unwrap();
                let new_client_message = ClientMessage {
                    address: client_message.address,
                    message: Some(message.clone())
                };

                println!("Server send(): {}", new_client_message.message.as_ref().unwrap());
                server_socket_sender_2.send(new_client_message);
            }
        });

        server_socket.on_disconnection(|client_message| {
            println!("Server on_disconnection(): {:?}", client_message.address);
        });

        let current_socket_address = find_ip_address::get() + ":" + DEFAULT_PORT;
        println!("Webserver Listening on: {}", current_socket_address);
        server_socket.listen(current_socket_address.as_str())
            .await;

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
