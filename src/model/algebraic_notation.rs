use super::{board::Board, actions::ChessAction, chess_actions::movement::Move};

pub fn from_algebraic_notation(board : &Board, notation : String) -> impl ChessAction {

    Move{start: 0, end : 0}
}
