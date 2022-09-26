use crate::model::{
    actions::{ChessAction, MovesList},
    board::{Board, InvalidMoveError},
    piece::Color,
};

use super::movement::Move;
pub struct Castle {
    pub king: Move,
    pub rook: Move,
}
impl Castle {
    pub fn new(king: Move, rook: Move) -> Self {
        Castle { king, rook }
    }
}
impl ChessAction for Castle {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        todo!();
    }

    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        todo!();
    }

    fn is_valid(&self, board: &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self, color: &Color) -> Result<MovesList, String> {
        Err("Cannot call 'as_promotion' on move 'Castle'".to_string())
    }

    fn to_algebraic_notation(&self, board: &Board) -> String {
        if (self.king.start % 8 > self.rook.start) {
            String::from("OwOwO")
        } else {
            String::from("OwO")
        }
    }

    fn target_square(&self) -> usize {
        self.king.end
    }

    fn start_square(&self) -> usize {
        self.king.start
    }

    fn double_forward(&self) -> Option<(usize, usize)> {
        None
    }
}
