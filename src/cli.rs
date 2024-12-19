use std::fmt::Debug;

use clap::Parser;
use knightmare::error::Error;
use log::info;
use std::fmt;

const FEN_START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// The builder tool for AcaciaLinux
#[derive(Parser)]
pub struct Cli {
    /// The log level to operate on (0 = info, 1 = debug, * = trace)
    #[arg(long = "loglevel", short = 'v', default_value_t = 0, global = true)]
    pub loglevel: u8,
}

impl Cli {
    pub fn run(&self) -> Result<i32, Error> {
        if std::env::var("RUST_LOG").is_err() {
            match &self.loglevel {
                0 => {}
                1 => std::env::set_var("RUST_LOG", "info"),
                2 => std::env::set_var("RUST_LOG", "debug"),
                _ => std::env::set_var("RUST_LOG", "trace"),
            }
        }
        pretty_env_logger::init();

        println!("Knightmare\n");

        let game_state = GameState::from_fen(FEN_START).expect("LOPP");

        Ok(0)
    }
}

#[derive(Debug)]
enum FromFENError {
    InvalidFormat,
    InvalidSymbol(char),
}

struct GameState {
    board: Board,
    side_to_move: Color,          //combine with others in bitmap?
    casteling_ability: u8,        //combine with others in bitmap?
    en_passant_target_square: u8, // in [0, 63]
    halfmove_clock: u8,           // max 50
    fullmove_counter: u16,
}

impl GameState {
    pub fn from_fen(fen: &str) -> Result<Self, FromFENError> {
        let elements: Vec<&str> = fen.split(" ").collect();
        if elements.len() < 6 {
            return Err(FromFENError::InvalidFormat);
        }
        let mut square_index = 0;
        let mut board = Board::default();
        for piece_char in elements[0].chars() {
            if piece_char == '/' {
                continue;
            }
            match piece_char.to_digit(10) {
                Some(num) => square_index += num,
                None => match ColoredPiece::from_fen(piece_char) {
                    Some(piece) => {
                        board.squares[(square_index / 8) as usize][(square_index % 8) as usize] =
                            Some(piece);
                        square_index += 1;
                    }
                    None => return Err(FromFENError::InvalidSymbol(piece_char)),
                },
            }
        }
        println!("{:#?}", board);
        todo!();
    }
    pub fn to_fen(fen: &str) -> Self {
        todo!();
    }
}

#[derive(Debug)]
struct Board {
    squares: [[Option<ColoredPiece>; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            squares: [[None; 8]; 8],
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ColoredPiece {
    piece: Piece,
    color: Color,
}

impl ColoredPiece {
    pub fn from_fen(fen: char) -> Option<Self> {
        Some(Self {
            piece: Piece::from_fen(fen)?,
            color: Color::from_fen(fen)?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    pub fn from_fen(fen: char) -> Option<Piece> {
        match fen.to_ascii_lowercase() {
            'p' => Some(Piece::Pawn),
            'n' => Some(Piece::Knight),
            'b' => Some(Piece::Bishop),
            'q' => Some(Piece::Queen),
            'r' => Some(Piece::Rook),
            'k' => Some(Piece::King),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Color {
    White,
    Black,
}

impl Color {
    pub fn from_fen(fen: char) -> Option<Self> {
        if !fen.is_ascii_alphabetic() {
            None
        } else {
            Some(if fen.is_ascii_lowercase() {
                Self::Black
            } else {
                Self::White
            })
        }
    }
}
