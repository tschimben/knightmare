//! Chess pieces
use crate::fen::{FromFENChar, FromFENError, ToFENChar};

use super::{board::Board, color::Color};

/// A chess piece that is in some color
#[derive(Clone, Copy, Debug)]
pub struct ColoredPiece {
    /// The piece
    pub piece: Piece,
    /// The color it's in
    pub color: Color,
}

pub struct BoardSquare {
    pub rank: u8,
    pub file: u8,
}

impl BoardSquare {
    pub fn coordinates(&self) -> (char, char) {
        let file = match self.file {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!(),
        };
        let rank = match self.rank {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => panic!(),
        };
        (file, rank)
    }
}

pub trait Move {
    fn get_all_moves(&self, board: &Board, starting_square: BoardSquare) -> Vec<BoardSquare>;
}

impl ColoredPiece {
    pub fn get_all_moves(&self, board: &Board, starting_square: BoardSquare) -> Vec<BoardSquare> {
        match self.piece {
            Piece::Pawn => Vec::new(),
            Piece::Rook => {
                let mut targets = Vec::new();
                for file in (starting_square.file + 1)..8 {
                    if let Some(piece) = board.squares[file as usize][starting_square.rank as usize]
                    {
                        if piece.color != self.color {
                            targets.push(BoardSquare {
                                file,
                                rank: starting_square.rank,
                            });
                            break;
                        }
                    }
                }
                for file in (0..starting_square.file).rev() {
                    if let Some(piece) = board.squares[file as usize][starting_square.rank as usize]
                    {
                        if piece.color != self.color {
                            targets.push(BoardSquare {
                                file,
                                rank: starting_square.rank,
                            });
                            break;
                        }
                    }
                }
                for rank in (starting_square.rank + 1)..8 {
                    if let Some(piece) = board.squares[starting_square.file as usize][rank as usize]
                    {
                        if piece.color != self.color {
                            targets.push(BoardSquare {
                                file: starting_square.file,
                                rank,
                            });
                            break;
                        }
                    }
                }
                for rank in (0..starting_square.rank).rev() {
                    if let Some(piece) = board.squares[starting_square.file as usize][rank as usize]
                    {
                        if piece.color != self.color {
                            targets.push(BoardSquare {
                                file: starting_square.file,
                                rank,
                            });
                            break;
                        }
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
