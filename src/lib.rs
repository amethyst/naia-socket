
#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        #[macro_use]
        extern crate serde_derive;

        mod webrtc_client_socket;
    }
    else {
        mod udp_client_socket;
    }
}

mod error;
mod socket_event;
mod message_sender;
mod client_socket;
mod packet;

pub use client_socket::ClientSocket;
pub use socket_event::SocketEvent;
pub use message_sender::MessageSender;
pub use naia_socket_shared::{Config, find_my_ip_address};
pub use packet::Packet;
pub use error::NaiaClientSocketError;