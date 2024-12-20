//! Chess pieces
use crate::fen::{FromFENChar, FromFENError, ToFENChar};

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

impl ToFENChar for ColoredPiece {
    fn to_fen(&self) -> char {
        if self.color == Color::White {
            self.piece.to_fen().to_ascii_uppercase()
        } else {
            self.piece.to_fen().to_ascii_lowercase()
        }
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

impl ToFENChar for Piece {
    fn to_fen(&self) -> char {
        match self {
            Piece::Pawn => 'p',
            Piece::Rook => 'r',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }
}
