

use crate::gaia_data_transport::server::{ Server, ServerImpl, instance, set_instance};

pub fn main() {
    unsafe { set_instance(ServerImpl::new().unwrap()) };
}

pub(crate) fn server() -> &'static mut ServerImpl {
    unsafe { instance() }
}