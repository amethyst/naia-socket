use crate::Result;

pub trait ClientSocket {
    unsafe fn new() -> Result<Self> where Self: Sized;

    unsafe fn on_connection(&mut self, func: fn());

    unsafe fn on_disconnection(&mut self, func: fn());

    unsafe fn on_receive(&mut self, func: fn(&str));

    unsafe fn on_error(&mut self, func: fn(&str));

    unsafe fn connect<S>(&mut self, address: &str);

    unsafe fn disconnect(&mut self);

    unsafe fn send<S>(&mut self, msg: &str);
}

/// Linux Client
#[cfg(feature = "WsLinuxClient")]
mod ws_linux_client_socket;

#[cfg(feature = "WsLinuxClient")]
pub use ws_linux_client_socket::WsLinuxClientSocket;

#[cfg(feature = "WsLinuxClient")]
pub type ClientSocketImpl = WsLinuxClientSocket;

/// Proto Wasm Client ///
#[cfg(feature = "WsWasmClient")]
mod ws_wasm_client_socket;

#[cfg(feature = "WsWasmClient")]
pub use self::ws_wasm_client_socket::WsWasmClientSocket;

#[cfg(feature = "WsWasmClient")]
pub type ClientSocketImpl = WsWasmClientSocket;

/// Final Wasm Client ///
#[cfg(feature = "WebrtcWasmClient")]
mod webrtc_client_socket;

#[cfg(feature = "WebrtcWasmClient")]
pub use self::webrtc_client_socket::WebrtcClientSocket;

#[cfg(feature = "WebrtcWasmClient")]
pub type ClientSocketImpl = WebrtcClientSocket;

static mut SOCKET_BACKEND_INSTANCE: Option<ClientSocketImpl> = None;

pub unsafe fn instance() -> &'static mut ClientSocketImpl {
    SOCKET_BACKEND_INSTANCE.as_mut().unwrap()
}

pub unsafe fn set_instance(instance: ClientSocketImpl) {
    SOCKET_BACKEND_INSTANCE = Some(instance);
}