
use crate::Result;
pub use crate::user::User;

pub trait ServerSocket {
    unsafe fn new() -> Result<Self> where Self: Sized;

    unsafe fn on_connection(&mut self, func: fn(User));

    unsafe fn on_disconnection(&mut self, func: fn(User));

    unsafe fn on_receive(&mut self, func: fn(User, &str));

    unsafe fn on_error(&mut self, func: fn(&str));

    unsafe fn listen<S>(&mut self, address: &str);
}

/// Proto Linux Server
#[cfg(feature = "WsServer")]
mod ws_server_socket;

#[cfg(feature = "WsServer")]
pub use self::ws_server_socket::WsServerSocket;

#[cfg(feature = "WsServer")]
pub type ServerSocketImpl = WsServerSocket;

/// Final Server ///
#[cfg(feature = "WebrtcServer")]
mod webrtc_server_socket;

#[cfg(feature = "WebrtcServer")]
pub use self::webrtc_server_socket::WebrtcServerSocket;

#[cfg(feature = "WebrtcServer")]
pub type ServerSocketImpl = WebrtcServerSocket;

static mut SOCKET_BACKEND_INSTANCE: Option<ServerSocketImpl> = None;

pub unsafe fn instance() -> &'static mut ServerSocketImpl {
    SOCKET_BACKEND_INSTANCE.as_mut().unwrap()
}

pub unsafe fn set_instance(instance: ServerSocketImpl) {
    SOCKET_BACKEND_INSTANCE = Some(instance);
}