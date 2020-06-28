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
