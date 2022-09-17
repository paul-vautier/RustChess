use super::{board::{Board, self, Position},
 actions::{ChessAction, Move}};


pub fn from_algebraic_notation(board : &Board, notation : String) -> impl ChessAction {

    Move{start: Position::new(0, 0).unwrap(), end : Position::new(0, 0).unwrap()}
}