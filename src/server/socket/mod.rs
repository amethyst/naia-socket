
mod client_socket;
use client_socket::ClientSocket;
use std::net::IpAddr;

pub trait ServerSocket {
    fn new() -> Self;

    fn listen(&self, address: &str);

    fn on_connection(&mut self, func: impl Fn(&ClientSocket) + 'static);

    fn on_receive(&mut self, func: impl Fn(&ClientSocket, &str) + 'static);

    fn on_disconnection(&mut self, func: impl Fn(IpAddr) + 'static);
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