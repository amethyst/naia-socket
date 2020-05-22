
cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        /// WebRTC Client ///
        pub use crate::webrtc_client_socket::WebrtcClientSocket;
        pub type ClientSocket = WebrtcClientSocket;
    }
    else {
        /// UDP Client ///
        pub use crate::udp_client_socket::UdpClientSocket;
        pub type ClientSocket = UdpClientSocket;
    }
}