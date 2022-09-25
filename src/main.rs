use tetra::ContextBuilder;
use view::tetra_state::{TetraState, SQUARE_SIZE};

use crate::model::board::Board;
mod model;
mod util;
mod view;
fn main() -> tetra::Result {
    let board = {
        let this = Board::from_fen("rnbqkbnr/pppppppp/8/8/3BRp2/8/PPPPPPPP/RNBQKBNR".to_string());
        match this {
            Ok(t) => t,
            Err(e) => panic!("Invalid board {}", e.err),
        }
    };

    let mut context = ContextBuilder::new(
        "Hello, world!",
        8 * SQUARE_SIZE as i32,
        8 * SQUARE_SIZE as i32,
    )
    .quit_on_escape(true)
    .show_mouse(true)
    .build()?;

    context.run(|ctx| TetraState::new(ctx, board))
}
