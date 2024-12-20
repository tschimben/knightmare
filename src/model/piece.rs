//! Chess pieces
use crate::fen::{FromFENChar, FromFENError};

use super::color::Color;

/// A chess piece that is in some color
#[derive(Clone, Copy, Debug)]
pub struct ColoredPiece {
    /// The piece
    pub piece: Piece,
    /// The color it's in
    pub color: Color,
}

impl FromFENChar for ColoredPiece {
    fn from_fen(fen: char) -> Result<Self, FromFENError>
    where
        Self: Sized,
    {
        Ok(Self {
            piece: Piece::from_fen(fen)?,
            color: Color::from_fen(fen)?,
        })
    }
}

/// A chess piece
#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl FromFENChar for Piece {
    fn from_fen(fen: char) -> Result<Self, FromFENError>
    where
        Self: Sized,
    {
        match fen.to_ascii_lowercase() {
            'p' => Ok(Piece::Pawn),
            'n' => Ok(Piece::Knight),
            'b' => Ok(Piece::Bishop),
            'q' => Ok(Piece::Queen),
            'r' => Ok(Piece::Rook),
            'k' => Ok(Piece::King),
            _ => Err(FromFENError::InvalidPlacementSymbol(fen)),
        }
    }
}
