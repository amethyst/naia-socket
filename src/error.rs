use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum NaiaClientSocketError {
    Message(String),
    Wrapped(Box<dyn Error + Send>),
}

impl fmt::Display for NaiaClientSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            NaiaClientSocketError::Message(msg) => write!(f, "Naia Client Socket Error: {}", msg),
            NaiaClientSocketError::Wrapped(boxed_err) => fmt::Display::fmt(boxed_err.as_ref(), f),
        }
    }
}

impl Error for NaiaClientSocketError {}
