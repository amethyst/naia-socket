use gaia_data_transport;

#[cfg(feature = "Server")]
use crate::gaia_data_transport::server::{ Server, ServerImpl, instance, set_instance};

#[cfg(feature = "Server")]
pub(crate) fn server() -> &'static mut ServerImpl {
    unsafe { instance() }
}

#[cfg(feature = "Client")]
use gaia_data_transport::client::Client;


pub fn main() {
    #[cfg(feature = "Server")]
    main_server();

    #[cfg(feature = "Client")]
    main_client();
}

#[cfg(feature = "Server")]
fn main_server() {
    unsafe { set_instance(ServerImpl::new().unwrap()) };
}

#[cfg(feature = "Client")]
fn main_client() {
    let client = Client::new();
}