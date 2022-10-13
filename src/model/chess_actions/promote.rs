use crate::model::{
    actions::{ChessAction, MovesList},
    board::{Board, InvalidMoveError},
    piece::{Color, Piece},
};
#[derive(Debug)]
pub struct Promote {
    pub piece: Piece,
    pub previous_action: Box<dyn ChessAction>,
}

impl Promote {
    pub fn new(piece: Piece, previous_action: Box<dyn ChessAction>) -> Self {
        Promote {
            piece,
            previous_action,
        }
    }
}

impl ChessAction for Promote {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        todo!()
        //TODO : Set rook moved
    }

    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        //TODO : Unset rook moved
        todo!()
    }

    fn as_promotion(&self, _color: &Color) -> Result<MovesList, String> {
        Err("Cannot call 'as_promotion' on move 'Promotion'".to_string())
    }

    fn to_algebraic_notation(&self, board: &Board) -> String {
        "promote".to_string()
    }

    fn target_square(&self) -> usize {
        self.previous_action.target_square()
    }

    fn start_square(&self) -> usize {
        self.previous_action.start_square()
    }

    fn double_forward(&self) -> Option<(usize, usize)> {
        None
    }
}
