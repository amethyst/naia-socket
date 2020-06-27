use std::error::Error;
use std::fmt;
use std::net::SocketAddr;

#[derive(Debug)]
pub enum NaiaServerSocketError {
    Wrapped(Box<dyn Error>),
    SendError(SocketAddr),
}

impl fmt::Display for NaiaServerSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            NaiaServerSocketError::Wrapped(boxed_err) => fmt::Display::fmt(boxed_err.as_ref(), f),
            NaiaServerSocketError::SendError(addr) => fmt::Display::fmt(&addr, f),
        }
    }
}

impl Error for NaiaServerSocketError {}