
use crate::Result;
pub use crate::user::User;

pub trait ServerSocket {
    fn new() -> Result<Self> where Self: Sized;

    fn on_connection(&self, func: fn(User));

    fn on_disconnection(&self, func: fn(User));

    fn on_receive(&self, func: fn(User, &str));

    fn on_error(&self, func: fn(&str));

    fn listen<S>(&self, address: &str);
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