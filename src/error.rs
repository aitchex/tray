use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidImage,
    NotFound(String),
    OutOfRange(String),
    UnknownError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidImage => write!(f, "Invalid Image"),
            Error::NotFound(ref err) => write!(f, "Not found: {}", err),
            Error::OutOfRange(ref err) => write!(f, "Out of range: {}", err),
            Error::UnknownError(ref err) => write!(f, "Unknown Error: {}", err),
        }
    }
}
