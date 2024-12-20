//! Stuff to work with FEN notation strings

/// The starting position of a chess game in FEN notation
pub const FEN_START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Possible errors that can arise when parsing FEN notation strings
#[derive(Debug)]
pub enum FromFENError {
    /// The whole string is in an invalid format and cannot be parsed
    InvalidFormat,
    /// The parser encountered an invalid placement symbol
    InvalidPlacementSymbol(char),
    /// An invalid count / layout for files has been parsed
    InvalidFileLayout(u8),
    /// An invalid count / layout for ranks has been parsed
    InvalidRankLayout(u8),
}

/// Parse FEN strings
pub trait FromFENString {
    /// Parses a string in FEN notation to [Self]
    /// # Arguments
    /// * `fen` - The FEN representation string
    /// # Returns
    /// [Self] or a [FromFENError}
    fn from_fen(fen: &str) -> Result<Self, FromFENError>
    where
        Self: Sized;
}

/// Parse FEN characters
pub trait FromFENChar {
    /// Parses a character in FEN notation to [Self]
    /// # Arguments
    /// * `fen` - The FEN representation character
    /// # Returns
    /// [Self] or a [FromFENError}
    fn from_fen(fen: char) -> Result<Self, FromFENError>
    where
        Self: Sized;
}
