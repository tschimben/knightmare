//! The chess board
use crate::fen::{FromFENChar, FromFENError, FromFENString, ToFENChar, ToFENString};

use super::{coordinate::Coordinate, piece::ColoredPiece};

/// A representation of the chess board
#[derive(Debug, Default)]
pub struct Board {
    /// The squares that (can) hold a piece
    /// indexed by `[file][rank]` starting at `A1`
    pub squares: [[Option<ColoredPiece>; 8]; 8],
}

impl Board {
    /// Gets a colored piece at `coord`
    /// # Arguments
    /// * `coord` - The coordinate to get the piece from
    pub fn get(&self, coord: Coordinate) -> Option<ColoredPiece> {
        self.squares[coord.file as usize][coord.rank as usize]
    }

    /// Returns a list of occupied fields in file-major form
    pub fn get_occupied_fields_fm(&self) -> Vec<(Coordinate, ColoredPiece)> {
        let mut pieces = Vec::new();

        for file in 0..8 {
            for rank in 0..8 {
                if let Some(piece) = self.squares[file][rank] {
                    pieces.push((
                        Coordinate::from_u8s(file as u8, rank as u8)
                            .expect("[DEV] Internal File and Rank mishap"),
                        piece,
                    ));
                }
            }
        }

        pieces
    }

    /// Returns a list of occupied fields in rank-major form
    pub fn get_occupied_fields_rm(&self) -> Vec<(Coordinate, ColoredPiece)> {
        let mut pieces = Vec::new();

        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.squares[file][rank] {
                    pieces.push((
                        Coordinate::from_u8s(file as u8, rank as u8)
                            .expect("[DEV] Internal File and Rank mishap"),
                        piece,
                    ));
                }
            }
        }

        pieces
    }
}

impl FromFENString for Board {
    fn from_fen(fen: &str) -> Result<Self, crate::fen::FromFENError>
    where
        Self: Sized,
    {
        let mut board = Self::default();

        let mut rank = 7u8;
        let mut file = 0u8;

        // Iterate over all piece characters
        for piece_char in fen.chars() {
            // A '/' indicates a new rank
            if piece_char == '/' {
                file = 0;
                rank -= 1;

                // Check for out of range ranks
                if rank > 8 {
                    return Err(FromFENError::InvalidRankLayout(rank));
                }

                continue;
            }

            // Try to parse a digit. In that case, add it to the file
            match piece_char.to_digit(10) {
                Some(num) => {
                    file += num as u8;
                    if file > 8 {
                        return Err(FromFENError::InvalidFileLayout(file));
                    }
                }
                // If we can't parse to a number, it can only be a placement
                None => {
                    let piece = ColoredPiece::from_fen(piece_char)?;
                    board.squares[file as usize][rank as usize] = Some(piece);

                    file += 1;
                    if file > 8 {
                        return Err(FromFENError::InvalidFileLayout(file));
                    }
                }
            }
        }

        Ok(board)
    }
}

impl ToFENString for Board {
    fn to_fen(&self) -> String {
        let mut ranks: Vec<String> = Vec::new();

        for rank in 0..8 {
            let mut string = String::new();
            let mut empty_counter: u8 = 0;
            for file in 0..8 {
                match self.squares[file as usize][rank as usize] {
                    Some(piece) => {
                        if empty_counter > 0 {
                            string.push_str(&empty_counter.to_string());
                            empty_counter = 0;
                        }
                        string.push(piece.to_fen());
                    }
                    None => {
                        empty_counter += 1;
                    }
                }
            }
            if empty_counter > 0 {
                string.push_str(&empty_counter.to_string());
            }
            ranks.push(string);
        }

        ranks.join("/")
    }
}
