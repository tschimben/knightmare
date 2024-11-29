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

impl ErrorType {
    pub fn get_code(&self) -> u32 {
        let self_code = unsafe { *(self as *const Self as *const u16) };

        let sub_code: u16 = match self {
            _ => 0xFFFF,
        };

        (self_code as u32) << 16 | sub_code as u32
    }
}

impl From<io::Error> for ErrorType {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}
impl ErrorIn for io::Error {}
