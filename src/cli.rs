use clap::Parser;
use knightmare::{
    error::Error,
    fen::{FromFENString, FEN_START},
    game::GameState,
};

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

        let game = GameState::from_fen(FEN_START).expect("LOPP");

        for (coordinate, piece) in game.board.get_occupied_fields_rm() {
            println!(
                "{piece} @ {coordinate} can move to {:?}",
                piece
                    .get_all_moves(&game.board, coordinate)
                    .iter()
                    .map(|m| { format!("{m}") })
                    .collect::<Vec<String>>()
            )
        }

        Ok(0)
    }
}
