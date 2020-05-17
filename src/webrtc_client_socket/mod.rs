
use std::net::{SocketAddr, Ipv4Addr, IpAddr};

extern crate webrtc_client_dep;

use crate::{ClientSocket};
use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
//use crate::internal_shared::{CLIENT_HANDSHAKE_MESSAGE, SERVER_HANDSHAKE_MESSAGE};

pub struct WebrtcClientSocket {
    //address: SocketAddr
}

impl ClientSocket for WebrtcClientSocket {

    fn bind(address: &str) -> WebrtcClientSocket {
        info!("Hello WebrtcClientSocket!");

        webrtc_client_dep::setup_webrtc_stuff();

        WebrtcClientSocket {
            //address: address.parse().unwrap()
        }
    }

    fn receive(&mut self) -> SocketEvent {
        return SocketEvent::None;
    }

    fn get_sender(&mut self) -> MessageSender {
        return MessageSender::new();
    }

    fn server_address(&self) -> SocketAddr {
        //return self.address;
        return SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,168,1,5)), 0);
    }
}
