//! Chess pieces
use crate::fen::{FromFENChar, FromFENError, ToFENChar};

use super::{board::Board, color::Color, coordinate::Coordinate};

use std::fmt::{Debug, Display};

/// A chess piece that is in some color
#[derive(Clone, Copy, Debug)]
pub struct ColoredPiece {
    /// The piece
    pub piece: Piece,
    /// The color it's in
    pub color: Color,
}

impl Display for ColoredPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.color, self.piece)
    }
}

impl ColoredPiece {
    pub fn get_all_moves(&self, board: &Board, starting_square: Coordinate) -> Vec<Coordinate> {
        match self.piece {
            Piece::Pawn => Vec::new(),
            Piece::Rook => {
                let mut targets = Vec::new();

                // Files positive
                let mut start = starting_square;
                while let Some(coordinate) = start.next_file() {
                    start = coordinate;
                    if let Some(piece) = board.get(coordinate) {
                        if piece.color != self.color {
                            targets.push(coordinate);
                        }
                        break;
                    } else {
                        targets.push(coordinate)
                    }
                }

                // Files negative
                let mut start = starting_square;
                while let Some(coordinate) = start.prev_file() {
                    start = coordinate;
                    if let Some(piece) = board.get(coordinate) {
                        if piece.color != self.color {
                            targets.push(coordinate);
                        }
                        break;
                    } else {
                        targets.push(coordinate)
                    }
                }

                // Ranks positive
                let mut start = starting_square;
                while let Some(coordinate) = start.next_rank() {
                    start = coordinate;
                    if let Some(piece) = board.get(coordinate) {
                        if piece.color != self.color {
                            targets.push(coordinate);
                        }
                        break;
                    } else {
                        targets.push(coordinate)
                    }
                }

                // Ranks negative
                let mut start = starting_square;
                while let Some(coordinate) = start.prev_rank() {
                    start = coordinate;
                    if let Some(piece) = board.get(coordinate) {
                        if piece.color != self.color {
                            targets.push(coordinate);
                        }
                        break;
                    } else {
                        targets.push(coordinate)
                    }
                }

                targets
            }
            Piece::Knight => Vec::new(),
            Piece::Bishop => Vec::new(),
            Piece::Queen => Vec::new(),
            Piece::King => Vec::new(),
        }
    }
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

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Pawn => "Pawn",
            Self::Rook => "Rook",
            Self::Knight => "Knight",
            Self::Bishop => "Bishop",
            Self::Queen => "Queen",
            Self::King => "King",
        };

        write!(f, "{string}")
    }
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
