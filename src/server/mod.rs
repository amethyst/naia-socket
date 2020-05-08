
use async_trait::async_trait;

mod client_event;
pub use client_event::ClientEvent;
mod client_message;
pub use client_message::ClientMessage;
mod message_sender;
pub use message_sender::MessageSender;

#[async_trait]
pub trait ServerSocket {
    async fn bind(address: &str) -> Self;

    async fn receive(&mut self) -> ClientEvent;

    fn get_sender(&mut self) -> MessageSender;
}

/// Proto Linux Server
#[cfg(feature = "UdpServer")]
mod udp_server_socket;

#[cfg(feature = "UdpServer")]
pub use self::udp_server_socket::UdpServerSocket;

#[cfg(feature = "UdpServer")]
pub type ServerSocketImpl = UdpServerSocket;

/// Final Server ///
#[cfg(feature = "WebrtcServer")]
mod webrtc_server_socket;

#[cfg(feature = "WebrtcServer")]
pub use self::webrtc_server_socket::WebrtcServerSocket;

#[cfg(feature = "WebrtcServer")]
pub type ServerSocketImpl = WebrtcServerSocket;