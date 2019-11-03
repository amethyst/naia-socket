
use gaia_data_transport;

pub mod shared;

#[cfg(feature = "Client")]
pub mod client;

#[cfg(feature = "Client")]
pub fn main() {
    client::main();
}

#[cfg(feature = "Server")]
pub mod server;

#[cfg(feature = "Server")]
pub fn main() {
    server::main();
}

