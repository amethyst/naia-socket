
mod socket_event;
pub use socket_event::SocketEvent;
mod message_sender;
pub use message_sender::MessageSender;

pub trait ClientSocket {
    fn bind(address: &str) -> Self;

    fn receive(&mut self) -> SocketEvent;

    fn get_sender(&mut self) -> MessageSender;
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