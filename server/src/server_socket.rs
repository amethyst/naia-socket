cfg_if! {
    if #[cfg(feature = "use-webrtc")] {
        /// WebRTC Server ///
        pub use crate::webrtc_server_socket::WebrtcServerSocket;
        pub type ServerSocket = WebrtcServerSocket;
    }
    else if #[cfg(feature = "use-udp")] {
        /// UDP Server
        pub use crate::udp_server_socket::UdpServerSocket;
        pub type ServerSocket = UdpServerSocket;
    }
}
