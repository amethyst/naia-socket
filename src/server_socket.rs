
use async_trait::async_trait;

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;

#[async_trait]
pub trait ServerSocket {
    async fn bind(address: &str) -> Self;

    async fn receive(&mut self) -> SocketEvent;

    fn get_sender(&mut self) -> MessageSender;
}

cfg_if! {
    if #[cfg(feature = "use-webrtc")] {
        /// WebRTC Server ///
        pub use crate::webrtc_server_socket::WebrtcServerSocket;
        pub type ServerSocketImpl = WebrtcServerSocket;
    }
    else if #[cfg(feature = "use-udp")] {
        /// UDP Server
        pub use crate::udp_server_socket::UdpServerSocket;
        pub type ServerSocketImpl = UdpServerSocket;
    }
}