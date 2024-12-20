//! Functionality for working with the player colors
use crate::fen::{FromFENChar, FromFENError};

/// The color of player (black or white)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    /// The white player / piece
    White,
    /// The black player / piece
    Black,
}

impl FromFENChar for Color {
    fn from_fen(fen: char) -> Result<Self, crate::fen::FromFENError>
    where
        Self: Sized,
    {
        if !fen.is_ascii_alphabetic() {
            Err(FromFENError::InvalidPlacementSymbol(fen))
        } else {
            Ok(if fen.is_ascii_lowercase() {
                Self::Black
            } else {
                Self::White
            })
        }
    }
}
