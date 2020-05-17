
use std::net::SocketAddr;

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;

pub trait ClientSocket {
    fn bind(address: &str) -> Self;

    fn receive(&mut self) -> SocketEvent;

    fn get_sender(&mut self) -> MessageSender;

    fn server_address(&self) -> SocketAddr;
}

/// UDP Client ///
#[cfg(not(target_arch = "wasm32"))]
pub use crate::udp_client_socket::UdpClientSocket;

#[cfg(not(target_arch = "wasm32"))]
pub type ClientSocketImpl = UdpClientSocket;

/// WebRTC Client ///
#[cfg(target_arch = "wasm32")]
pub use crate::webrtc_client_socket::WebrtcClientSocket;

#[cfg(target_arch = "wasm32")]
pub type ClientSocketImpl = WebrtcClientSocket;