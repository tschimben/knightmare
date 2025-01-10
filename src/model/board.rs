//! The chess board
use crate::fen::{FromFENChar, FromFENError, FromFENString, ToFENChar, ToFENString};

use super::piece::ColoredPiece;

/// A representation of the chess board
#[derive(Debug, Default)]
pub struct Board {
    /// The squares that (can) hold a piece
    pub squares: [[Option<ColoredPiece>; 8]; 8],
}

impl FromFENString for Board {
    fn from_fen(fen: &str) -> Result<Self, crate::fen::FromFENError>
    where
        Self: Sized,
    {
        let mut board = Self::default();

        let mut rank = 0u8;
        let mut file = 0u8;

        // Iterate over all piece characters
        for piece_char in fen.chars() {
            // A '/' indicates a new rank
            if piece_char == '/' {
                file = 0;
                rank += 1;

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
