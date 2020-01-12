use crate::Result;

pub trait ClientSocket {
    unsafe fn new() -> Result<Self> where Self: Sized;

    fn on_connection(&self, func: fn());

    fn on_disconnection(&self, func: fn());

    fn on_receive(&self, func: fn(&str));

    fn on_error(&self, func: fn(&str));

    fn connect<S>(&self, address: &str);

    fn disconnect(&self);

    fn send<S>(&self, msg: &str);
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