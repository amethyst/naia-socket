use gaia_data_transport;

#[cfg(feature = "Server")]
use crate::gaia_data_transport::server::{ Server, ServerImpl, instance, set_instance};

#[cfg(feature = "Server")]
pub fn main() {
    unsafe { set_instance(ServerImpl::new().unwrap()) };
}

#[cfg(feature = "Server")]
pub(crate) fn server() -> &'static mut ServerImpl {
    unsafe { instance() }
}




#[cfg(feature = "Client")]
use crate::gaia_data_transport::client::{ Client, ClientImpl, instance, set_instance};

#[cfg(feature = "Client")]
pub fn main() {
    unsafe { set_instance(ClientImpl::new().unwrap()) };
}

#[cfg(feature = "Client")]
pub(crate) fn client() -> &'static mut ClientImpl {
    unsafe { instance() }
}
