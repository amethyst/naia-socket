
use std::{fmt,
          error::Error};

#[derive(Debug)]
pub enum GaiaError {
    GeneralError(String)
}

impl fmt::Display for GaiaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for GaiaError {
    fn description(&self) -> &str {
        "some description"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
