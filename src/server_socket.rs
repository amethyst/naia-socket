
use async_trait::async_trait;

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;

#[async_trait]
pub trait ServerSocket {
    async fn bind(address: &str) -> Self;

    async fn receive(&mut self) -> SocketEvent;

    fn get_sender(&mut self) -> MessageSender;
}

/// Proto Linux Server
#[cfg(feature = "use-udp")]
pub use crate::udp_server_socket::UdpServerSocket;

#[cfg(feature = "use-udp")]
pub type ServerSocketImpl = UdpServerSocket;

/// Final Server ///
#[cfg(feature = "use-webrtc")]
pub use crate::webrtc_server_socket::WebrtcServerSocket;

#[cfg(feature = "use-webrtc")]
pub type ServerSocketImpl = WebrtcServerSocket;