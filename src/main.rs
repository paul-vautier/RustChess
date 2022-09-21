use core::panic;

use crate::model::{board::Position, piece::Color};
use crate::model::board::Square::*;
mod model;
mod view;
fn main() {
    match  model::board::from_fen("8/pppppppp/8/8/8/8/PPPPPPPP/8".to_string()) {
        Ok(board) => {
            print!("{}", board);
            print!("\n\n\n\n{0}", match  board.piece_at(&Position::new(1,3).expect("Expected to find a piece")) {
                    Inside(option) => {
                        option.as_ref().unwrap().valid_moves(&Position::new(1,3).unwrap(), &Color::BLACK, &board).to_algebraic_notation(&board)
                    },
                    Outside => panic!("explicit panic"),
                }
            )
        },
        Err(error) => print!("{}", error.err)
    }
}
