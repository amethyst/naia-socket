use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GaiaClientSocketError {
    Wrapped(Box<dyn Error + Send>)
}

impl fmt::Display for GaiaClientSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            GaiaClientSocketError::Wrapped(boxed_err) => fmt::Display::fmt(boxed_err.as_ref(), f),
        }
    }
}

impl Error for GaiaClientSocketError {}