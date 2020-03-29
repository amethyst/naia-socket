use crate::Result;

pub trait Sender {
    fn send(&self, msg: &str);
}

pub trait ClientSocket {
    fn new() -> Self;

    fn connect(&self, address: &str);

    fn send(&self, msg: &str);

    fn disconnect(&self);

    fn on_connection(&mut self, func: impl Fn(&Sender) + 'static);

    fn on_receive(&mut self, func: impl Fn(&Sender, &str) + 'static);

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