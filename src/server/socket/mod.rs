
use crate::Result;
pub use crate::user::User;

pub trait Socket {
    unsafe fn new() -> Result<Self> where Self: Sized;

    unsafe fn on_connection(&mut self, func: fn(User));

    unsafe fn on_disconnection(&mut self, func: fn(User));

    unsafe fn on_receive(&mut self, func: fn(User, &str));

    unsafe fn on_error(&mut self, func: fn(&str));

    unsafe fn listen<S>(&mut self, address: &str);
}

/// Proto Linux Server
#[cfg(feature = "ProtoLinuxServer")]
mod proto_socket;

#[cfg(feature = "ProtoLinuxServer")]
pub use self::proto_socket::ProtoSocketBackend;

#[cfg(feature = "ProtoLinuxServer")]
pub type SocketImpl = ProtoSocketBackend;



/// Final Server ///
#[cfg(feature = "FinalLinuxServer")]
mod final_socket;

#[cfg(feature = "FinalLinuxServer")]
pub use self::final_socket::FinalSocketBackend;

#[cfg(feature = "FinalLinuxServer")]
pub type SocketImpl = FinalSocketBackend;




static mut SOCKET_BACKEND_INSTANCE: Option<SocketImpl> = None;

pub unsafe fn instance() -> &'static mut SocketImpl {
    SOCKET_BACKEND_INSTANCE.as_mut().unwrap()
}

pub unsafe fn set_instance(instance: SocketImpl) {
    SOCKET_BACKEND_INSTANCE = Some(instance);
}
