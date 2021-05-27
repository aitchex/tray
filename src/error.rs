use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidImage,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidImage => write!(f, "Invalid Image"),
        }
    }
}
