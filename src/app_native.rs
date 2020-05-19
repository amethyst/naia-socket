
use gaia_client_socket::{ClientSocket, ClientSocketImpl, SocketEvent, MessageSender};

pub use crate::app::App;

impl App {
    pub fn start_loop(&mut self) {
        loop {
            self.update();
        }
    }
}