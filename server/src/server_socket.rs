cfg_if! {
    if #[cfg(feature = "use-webrtc")] {
        // WebRTC Server ///
        pub use crate::webrtc_server_socket::WebrtcServerSocket;
        /// ServerSocket is an alias for a socket abstraction using either UDP or WebRTC for communications
        pub type ServerSocket = WebrtcServerSocket;
    }
    else if #[cfg(feature = "use-udp")] {
        // UDP Server
        pub use crate::udp_server_socket::UdpServerSocket;
        /// ServerSocket is an alias for a socket abstraction using either UDP or WebRTC for communications
        pub type ServerSocket = UdpServerSocket;
    }
}
