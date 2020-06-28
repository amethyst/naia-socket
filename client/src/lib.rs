#![deny(missing_docs,
    missing_debug_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces, unused_qualifications)]

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

mod client_socket;
mod error;
mod message_sender;
mod packet;
mod socket_event;

pub use client_socket::ClientSocket;
pub use error::NaiaClientSocketError;
pub use message_sender::MessageSender;
pub use naia_socket_shared::{find_my_ip_address, Config};
pub use packet::Packet;
pub use socket_event::SocketEvent;
