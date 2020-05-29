use std::error::Error;
use std::fmt;
use std::net::SocketAddr;

#[derive(Debug)]
pub enum GaiaServerSocketError {
    Wrapped(Box<dyn Error>),
    SendError(SocketAddr),
}

impl fmt::Display for GaiaServerSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            GaiaServerSocketError::Wrapped(boxed_err) => fmt::Display::fmt(boxed_err.as_ref(), f),
            GaiaServerSocketError::SendError(addr) => fmt::Display::fmt(&addr, f),
        }
    }
}

impl Error for GaiaServerSocketError {}