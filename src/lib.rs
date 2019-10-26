use gaia_data_transport;

#[cfg(feature = "Server")]
use gaia_data_transport::server::Server;

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
    let server = Server::new();
}

#[cfg(feature = "Client")]
fn main_client() {
    let client = Client::new();
}