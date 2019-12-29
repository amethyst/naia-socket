use crate::Result;

pub trait Socket {
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
#[cfg(feature = "LinuxClient")]
mod linux_socket;

#[cfg(feature = "LinuxClient")]
pub use linux_socket::LinuxSocketBackend;

#[cfg(feature = "LinuxClient")]
pub type SocketImpl = LinuxSocketBackend;


/// Proto Wasm Client ///
#[cfg(feature = "ProtoWasmClient")]
mod proto_wasm_socket;

#[cfg(feature = "ProtoWasmClient")]
pub use self::proto_wasm_socket::ProtoWasmSocketBackend;

#[cfg(feature = "ProtoWasmClient")]
pub type SocketImpl = ProtoWasmSocketBackend;


/// Final Wasm Client ///
#[cfg(feature = "FinalWasmClient")]
mod final_wasm_socket;

#[cfg(feature = "FinalWasmClient")]
pub use self::final_wasm_socket::FinalWasmSocketBackend;

#[cfg(feature = "FinalWasmClient")]
pub type SocketImpl = FinalWasmSocketBackend;


static mut SOCKET_BACKEND_INSTANCE: Option<SocketImpl> = None;

pub unsafe fn instance() -> &'static mut SocketImpl {
    SOCKET_BACKEND_INSTANCE.as_mut().unwrap()
}

pub unsafe fn set_instance(instance: SocketImpl) {
    SOCKET_BACKEND_INSTANCE = Some(instance);
}
