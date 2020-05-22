
extern crate log;

#[macro_use]
extern crate cfg_if;

#[cfg(feature = "use-udp")]
mod udp_server_socket;
#[cfg(feature = "use-webrtc")]
mod webrtc_server_socket;

mod error;
mod socket_event;
mod client_message;
mod message_sender;
mod server_socket;

pub use server_socket::ServerSocket;
pub use socket_event::SocketEvent;