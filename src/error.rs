use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error, String), // The file path or render step name.
    Serde(serde_json::Error),
    Custom(String),
    Image(image::ImageError),
    Regex(regex::Error),
    ParseInt(std::num::ParseIntError),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

impl From<image::ImageError> for Error {
    fn from(value: image::ImageError) -> Self {
        Error::Image(value)
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Error::Regex(value)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}