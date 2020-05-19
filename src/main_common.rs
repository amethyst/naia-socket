
extern crate log;

#[cfg(not(target_arch = "wasm32"))]
use gaia_socket_shared::{find_my_ip_address};

#[cfg(not(target_arch = "wasm32"))]
use crate::app_native::App;

#[cfg(target_arch = "wasm32")]
use crate::app_wasm::App;

#[cfg(target_arch = "wasm32")]
const SERVER_IP_ADDRESS: &str = "192.168.1.6";

const SERVER_PORT: &str = "3179";

pub fn main_common() {

    // Uncomment the line below to enable logging. You don't need it if something else (e.g. quicksilver) is logging for you
    set_logger(log::Level::Info);

    info!("Client Example Started");

    #[cfg(target_arch = "wasm32")]
    let server_socket_address = SERVER_IP_ADDRESS.to_owned() + ":" + SERVER_PORT;

    #[cfg(not(target_arch = "wasm32"))]
    let server_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;

    let mut app = App::new(&server_socket_address);

    app.start_loop();
}

fn set_logger(level: log::Level) {
    #[cfg(target_arch = "wasm32")]
    web_logger::custom_init(web_logger::Config { level });

    #[cfg(not(target_arch = "wasm32"))]
    simple_logger::init_with_level(level).expect("A logger was already initialized");
}