use core::panic;

use crate::model::{board::Position, piece::Color};
use crate::model::board::{Square::*, Board};
mod model;
mod util;
mod view;
fn main() {
    match Board::from_fen("8/pppppppp/8/8/8/8/PPPPPPPP/8".to_string()) {
        Ok(board) => {
            print!("{}", board);
            print!("\n\n\n\n{0}", match  board.piece_at_mailbox_index(Board::to_mailbox_index(0, 1)) {
                    Inside(option) => {
                        option.as_ref().unwrap().valid_moves(Board::to_mailbox_index(0, 1), &Color::BLACK, &board).to_algebraic_notation(&board)
                    },
                    Outside => panic!("explicit panic"),
                }
            )
        },
        Err(error) => print!("{}", error.err)
    }
}
