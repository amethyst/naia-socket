use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GaiaServerSocketError {
    Wrapped(Box<dyn Error>)
}

impl fmt::Display for GaiaServerSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            GaiaServerSocketError::Wrapped(boxed_err) => fmt::Display::fmt(boxed_err.as_ref(), f),
        }
    }
}

impl Error for GaiaServerSocketError {}