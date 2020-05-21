
use std::net::SocketAddr;

use super::error::GaiaClientSocketError;
use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;

pub trait ClientSocket {
    fn bind(address: &str) -> Self;

    fn receive(&mut self) -> Result<SocketEvent, GaiaClientSocketError>;

    fn get_sender(&mut self) -> MessageSender;

    fn server_address(&self) -> SocketAddr;
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        /// WebRTC Client ///
        pub use crate::webrtc_client_socket::WebrtcClientSocket;
        pub type ClientSocketImpl = WebrtcClientSocket;
    }
    else {
        /// UDP Client ///
        pub use crate::udp_client_socket::UdpClientSocket;
        pub type ClientSocketImpl = UdpClientSocket;
    }
}