use crate::model::{board::Position, piece::Color};

mod model;
mod view;
fn main() {
    match  model::board::from_fen("8/pppppppp/8/8/8/8/PPPPPPPP/8".to_string()) {
        Ok(board) => {
            print!("{}", board);
            print!("{}", board.piece_at(&Position::new(0,1).unwrap()).unwrap().valid_moves(&Position::new(0,1).unwrap(), &Color::BLACK, &board))
        },
        Err(error) => print!("{}", error.err)
    }
}
