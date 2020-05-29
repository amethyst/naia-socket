
extern crate log;

#[macro_use]
extern crate cfg_if;

#[cfg(feature = "use-udp")]
mod udp_server_socket;
#[cfg(feature = "use-webrtc")]
mod webrtc_server_socket;

mod error;
mod socket_event;
mod message_sender;
mod server_socket;
mod packet;

pub use server_socket::ServerSocket;
pub use socket_event::SocketEvent;
pub use message_sender::MessageSender;
pub use gaia_socket_shared::{Config, find_my_ip_address};
pub use packet::Packet;
pub use error::GaiaServerSocketError;