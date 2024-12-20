use std::{fmt::Display, io};

use super::ErrorIn;

/// All the possible types of errors that can occur within Velocity
#[derive(Debug)]
#[repr(u16)]
pub enum ErrorType {
    /// An IO error
    IO(io::Error),
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for ErrorType {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}
impl ErrorIn for io::Error {}
