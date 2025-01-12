//! The game and its state
use crate::fen::{FromFENError, FromFENString};

use super::board::Board;

/// A game state
pub struct GameState {
    pub board: Board,
    //pub side_to_move: Color,          //combine with others in bitmap?
    //pub casteling_ability: u8,        //combine with others in bitmap?
    //pub en_passant_target_square: u8, // in [0, 63]
    //pub halfmove_clock: u8,           // max 50
    //pub fullmove_counter: u16,
}

impl FromFENString for GameState {
    fn from_fen(fen: &str) -> Result<Self, FromFENError> {
        let elements: Vec<&str> = fen.split(" ").collect();
        if elements.len() < 6 {
            return Err(FromFENError::InvalidFormat);
        }
        let board = Board::from_fen(elements[0]).unwrap();
        println!("{:#?}", board);

        Ok(Self { board })
    }
}
