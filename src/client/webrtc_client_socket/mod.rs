use crate::Result;
use crate::client::socket::ClientSocket;

pub struct WebrtcClientSocket {

}

impl ClientSocket for WebrtcClientSocket {
    fn new() -> Result<WebrtcClientSocket> {
        println!("Hello WebrtcClientSocket!");
        Ok(WebrtcClientSocket {})
    }
}
