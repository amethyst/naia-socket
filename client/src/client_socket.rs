use std::fmt::Debug;

use naia_socket_shared::LinkConditionerConfig;

use super::{error::NaiaClientSocketError, message_sender::MessageSender, packet::Packet};

cfg_if! {
    if #[cfg(feature = "multithread")] {
        /// Defines the functionality of a Naia Client Socket
        pub trait ClientSocketTrait: Debug + Send + Sync {
            /// Receive a new packet from the socket, or a tick event
            fn receive(&mut self) -> Result<Option<Packet>, NaiaClientSocketError>;
            /// Gets a MessageSender you can use to send messages through the Server
            /// Socket
            fn get_sender(&mut self) -> MessageSender;
            /// Wraps the current socket in a LinkConditioner
            fn with_link_conditioner(
                self: Box<Self>,
                config: &LinkConditionerConfig,
            ) -> Box<dyn ClientSocketTrait>;
        }
    } else {
        /// Defines the functionality of a Naia Client Socket
        pub trait ClientSocketTrait: Debug {
            /// Receive a new packet from the socket, or a tick event
            fn receive(&mut self) -> Result<Option<Packet>, NaiaClientSocketError>;
            /// Gets a MessageSender you can use to send messages through the Server
            /// Socket
            fn get_sender(&mut self) -> MessageSender;
            /// Wraps the current socket in a LinkConditioner
            fn with_link_conditioner(
                self: Box<Self>,
                config: &LinkConditionerConfig,
            ) -> Box<dyn ClientSocketTrait>;
        }
    }
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        // WebRTC Client //
        pub use crate::webrtc_client_socket::WebrtcClientSocket;
        /// ClientSocket is an alias for a socket abstraction using either UDP or WebRTC for communications
        pub type ClientSocket = WebrtcClientSocket;
    }
    else {
        // UDP Client //
        pub use crate::udp_client_socket::UdpClientSocket;
        /// ClientSocket is an alias for a socket abstraction using either UDP or WebRTC for communications
        pub type ClientSocket = UdpClientSocket;
    }
}
