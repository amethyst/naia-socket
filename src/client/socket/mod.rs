use crate::Result;

mod server_socket;
use server_socket::ServerSocket;

pub trait ClientSocket {
    fn new() -> Self;

    fn connect(&mut self, address: &str);

    fn update(&mut self);

    fn send(&mut self, msg: &str);

    fn on_connection(&mut self, func: impl Fn(&ServerSocket) + 'static);

    fn on_receive(&mut self, func: impl Fn(&ServerSocket, &str) + 'static);

    fn on_disconnection(&mut self, func: impl Fn() + 'static);
}

/// UDP Client ///
#[cfg(feature = "UdpClient")]
mod udp_client_socket;

#[cfg(feature = "UdpClient")]
pub use self::udp_client_socket::UdpClientSocket;

#[cfg(feature = "UdpClient")]
pub type ClientSocketImpl = UdpClientSocket;

/// WebRTC Client ///
#[cfg(feature = "WebrtcClient")]
mod webrtc_client_socket;

#[cfg(feature = "WebrtcClient")]
pub use self::webrtc_client_socket::WebrtcClientSocket;

#[cfg(feature = "WebrtcClient")]
pub type ClientSocketImpl = WebrtcClientSocket;