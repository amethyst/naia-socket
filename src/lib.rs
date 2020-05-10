
#[macro_use]
extern crate log;

mod internal_shared;

pub mod shared;

#[cfg(any(feature = "WebrtcClient", feature = "UdpClient"))]
pub mod client;
//
//#[cfg(any(feature = "WebrtcClient", feature = "UdpClient"))]
//pub use client::{ClientSocket, ClientSocketImpl, SocketEvent};

#[cfg(any(feature = "WebrtcServer", feature = "UdpServer"))]
pub mod server;
//
//#[cfg(any(feature = "WebrtcServer", feature = "UdpServer"))]
//pub use server::{ServerSocket, ServerSocketImpl, SocketEvent};