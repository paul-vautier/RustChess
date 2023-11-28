use std::{env, time::Instant};
use tetra::ContextBuilder;
use view::tetra_state::{TetraState, SQUARE_SIZE};

use crate::model::board::Board;
mod generator;
mod model;
mod view;

const FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
const TURN: u32 = 1;

fn main() {
    let has_bench = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .chunks(2)
        .map(|window| (window[0].clone(), window[1].clone()))
        .find(|(key, val)| key == "--bench");
    if let Some((_, value)) = has_bench {
        bench(value.parse::<u32>().unwrap());
    } else {
        run();
    }
}

fn bench(depth : u32) {
    let now = Instant::now();
    let mut board = {
        let this = Board::from_fen(FEN.to_string());
        match this {
            Ok(t) => t,
            Err(e) => panic!("Invalid board {}", e.err),
        }
    };
    board.turn = TURN;
    println!(
        "count: {}",
        generator::generator::count_actions(&mut board, depth, true)
    );
    println!("elapsed: {}", now.elapsed().as_millis());
}

fn run() -> tetra::Result {
    let mut board = {
        let this = Board::from_fen(FEN.to_string());
        match this {
            Ok(t) => t,
            Err(e) => panic!("Invalid board {}", e.err),
        }
    };
    board.turn = TURN;

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
