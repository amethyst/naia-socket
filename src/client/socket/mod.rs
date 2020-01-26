use crate::Result;

pub trait ClientSocket {
    fn new() -> Self;

    fn connect(&self, address: &str);

    fn on_connection(&self, func: fn());

    fn on_disconnection(&self, func: fn());

    fn on_receive(&mut self, func: impl Fn(&str) + 'static);

    fn on_error(&self, func: fn(&str));

    fn disconnect(&self);

    fn send(&self, msg: &str);
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