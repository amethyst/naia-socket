
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate log;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
mod app;

#[cfg(target_arch = "wasm32")]
mod loop_wasm;

#[cfg(target_arch = "wasm32")]
use crate::loop_wasm::start_loop;

#[cfg(target_arch = "wasm32")]
pub use crate::app::App;

const SERVER_IP_ADDRESS: &str = "192.168.1.6";
const SERVER_PORT: &str = "3179";

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() {
    // Uncomment the line below to enable logging. You don't need it if something else (e.g. quicksilver) is logging for you
    web_logger::custom_init(web_logger::Config { level: log::Level::Info });

    info!("Client Example Started");

    let server_socket_address = SERVER_IP_ADDRESS.to_owned() + ":" + SERVER_PORT;

    let mut app = App::new(&server_socket_address);

    start_loop(app);
}