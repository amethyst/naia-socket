
use async_trait::async_trait;
use futures_channel::mpsc;
use std::error::Error;

mod client_event;
pub use client_event::ClientEvent;

#[async_trait]
pub trait ServerSocket {
    async fn bind(address: &str) -> ServerSocketImpl;

    async fn receive(&mut self) -> Result<ClientEvent, Box<dyn Error>>;

    fn get_sender(&mut self) -> mpsc::Sender<ClientEvent>;
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