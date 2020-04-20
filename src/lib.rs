
mod internal_shared;

pub mod shared;

#[cfg(any(feature = "WebrtcClient", feature = "UdpClient"))]
mod client;

#[cfg(any(feature = "WebrtcClient", feature = "UdpClient"))]
pub use client::{ClientSocket, ClientSocketImpl};

#[cfg(any(feature = "WebrtcServer", feature = "UdpServer"))]
mod server;

#[cfg(any(feature = "WebrtcServer", feature = "UdpServer"))]
pub use server::{ServerSocket, ServerSocketImpl};