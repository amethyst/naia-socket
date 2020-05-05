//use std::net::SocketAddr;
//
//pub struct ClientMessage {
//    pub address: SocketAddr,
//    pub message: Option<String>,
//}
//
//impl ClientMessage {
//    pub fn new(address: SocketAddr, message: &str) -> ClientMessage {
//        ClientMessage {
//            address,
//            message: Some(message.parse().unwrap())
//        }
//    }
//
//    pub fn new_empty(address: SocketAddr) -> ClientMessage {
//        ClientMessage {
//            address,
//            message: None
//        }
//    }
//}