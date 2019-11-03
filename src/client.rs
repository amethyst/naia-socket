
use crate::gaia_data_transport::client::{ Client, ClientImpl, instance, set_instance};

pub fn main() {
    unsafe { set_instance(ClientImpl::new().unwrap()) };
}

pub(crate) fn client() -> &'static mut ClientImpl {
    unsafe { instance() }
}
