use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Serde(serde_json::Error),
    Custom(String),
    Png(png::DecodingError),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Custom(value.to_string())
    }
}

impl From<png::DecodingError> for Error {
    fn from(value: png::DecodingError) -> Self {
        Error::Png(value)
    }
}