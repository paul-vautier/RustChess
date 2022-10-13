use std::{thread, time::Instant};
use tetra::ContextBuilder;
use view::tetra_state::{TetraState, SQUARE_SIZE};

use crate::model::board::Board;
mod generator;
mod model;
mod view;

const fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
const default: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
const turn: u32 = 1;
fn main() {
    bench();
}

fn bench() {
    let now = Instant::now();
    let mut board = {
        let this = Board::from_fen(fen.to_string());
        match this {
            Ok(t) => t,
            Err(e) => panic!("Invalid board {}", e.err),
        }
    };
    board.turn = turn;
    println!(
        "count: {}",
        generator::generator::count_actions(&mut board, 5, true)
    );
    println!("elapsed: {}", now.elapsed().as_millis());
}

fn run() -> tetra::Result {
    let mut board = {
        let this = Board::from_fen(fen.to_string());
        match this {
            Ok(t) => t,
            Err(e) => panic!("Invalid board {}", e.err),
        }
    };
    board.turn = turn;

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
