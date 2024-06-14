use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ConversionError(std::num::TryFromIntError),
    EmptySnake,
    NegativePoint,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{}.", e),
            Self::ConversionError(e) => write!(f, "{}.", e),
            Self::EmptySnake => write!(f, "no snake to display."),
            Self::NegativePoint => write!(f, "negative coordinate.")
        }
    }
}

impl std::error::Error for Error {}


impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(value: std::num::TryFromIntError) -> Self {
        Error::ConversionError(value)
    }
}
