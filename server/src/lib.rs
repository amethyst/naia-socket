extern crate log;

#[macro_use]
extern crate cfg_if;

#[cfg(feature = "use-udp")]
mod udp_server_socket;
#[cfg(feature = "use-webrtc")]
mod webrtc_server_socket;

mod error;
mod message_sender;
mod packet;
mod server_socket;
mod socket_event;

pub use error::NaiaServerSocketError;
pub use message_sender::MessageSender;
pub use naia_socket_shared::{find_my_ip_address, Config};
pub use packet::Packet;
pub use server_socket::ServerSocket;
pub use socket_event::SocketEvent;
