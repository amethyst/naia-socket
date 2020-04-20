use crate::server::ServerSocket;
use super::client_socket::ClientSocket;

pub struct WebrtcServerSocket {
    connect_function: Option<Box<dyn Fn(&ClientSocket)>>,
    receive_function: Option<Box<dyn Fn(&ClientSocket, &str)>>,
    disconnect_function: Option<Box<dyn Fn(&ClientSocket)>>,
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

    fn on_error(&mut self, func: impl Fn<(&ClientSocket, &str), Output=_>) {
        unimplemented!()
    }

    fn on_disconnection(&mut self, func: impl Fn(&ClientSocket) + 'static) {
        self.disconnect_function = Some(Box::new(func));
    }
}
//
//fn get_available_port(ip: &str) -> Option<u16> {
//    (8000..9000)
//        .find(|port| port_is_available(ip, *port))
//}
//
//fn port_is_available(ip: &str, port: u16) -> bool {
//    match TcpListener::bind((ip, port)) {
//        Ok(_) => true,
//        Err(_) => false,
//    }
//}