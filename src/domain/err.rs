use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProjectIOError {
    IO(std::io::Error),
    // Serde(serde_json::Error),
    Custom(String),
    Regex(regex::Error),
}

impl Error for ProjectIOError {}

impl Display for ProjectIOError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for ProjectIOError {
    fn from(value: std::io::Error) -> Self {
        ProjectIOError::IO(value)
    }
}

/* impl From<serde_json::Error> for ProjectIOError {
    fn from(value: serde_json::Error) -> Self {
        ProjectIOError::Serde(value)
    }
} */

impl From<&str> for ProjectIOError {
    fn from(value: &str) -> Self {
        ProjectIOError::Custom(value.to_string())
    }
}

impl From<regex::Error> for ProjectIOError {
    fn from(value: regex::Error) -> Self {
        ProjectIOError::Regex(value)
    }
}