
#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate log;

#[cfg(not(target_arch = "wasm32"))]
use gaia_socket_shared::{find_my_ip_address};

#[cfg(not(target_arch = "wasm32"))]
mod app;

#[cfg(not(target_arch = "wasm32"))]
mod app_native;

#[cfg(not(target_arch = "wasm32"))]
use crate::app_native::App;

const SERVER_PORT: &str = "3179";

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Uncomment the line below to enable logging. You don't need it if something else (e.g. quicksilver) is logging for you
    simple_logger::init_with_level(log::Level::Info).expect("A logger was already initialized");

    info!("Client Example Started");

    let server_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;

    let mut app = App::new(&server_socket_address);

    app.start_loop();
}