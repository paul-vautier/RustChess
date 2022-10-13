use tetra::math::num_integer::Average;

use crate::model::{
    actions::{ChessAction, MovesList},
    board::{self, Board, InvalidMoveError, BOARD_X, TO_BOARD},
    piece::{Color, Piece},
};

use super::promote::Promote;

#[derive(Debug)]
pub struct Move {
    pub start: usize,
    pub end: usize,
}

impl Move {
    pub fn new(start: usize, end: usize) -> Self {
        Move { start, end }
    }
}
impl Clone for Move {
    fn clone(&self) -> Self {
        Move {
            start: self.start.clone(),
            end: self.start.clone(),
        }
    }
}

impl ChessAction for Move {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        if let Some(piece) = board.move_piece(self.start, self.end)? {
            return Err(InvalidMoveError {
                start: self.start,
                end: self.end,
                reason: format!(
                    "'{}' captured '{}' during movement instead of capture",
                    board
                        .piece_at_board_index(TO_BOARD[self.end] as usize)
                        .unwrap(),
                    piece
                ),
            });
        }
        Ok(())
    }

    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        board.move_piece(self.end, self.start)?;
        Ok(())
    }

    fn as_promotion(&self, color: &Color) -> Result<MovesList, String> {
        Ok(MovesList(vec![
            Box::new(Promote::new(
                Piece::Bishop { color: *color },
                Box::new(self.clone()),
            )),
            Box::new(Promote::new(
                Piece::Knight { color: *color },
                Box::new(self.clone()),
            )),
            Box::new(Promote::new(
                Piece::Rook {
                    color: *color,
                    first_move: 0,
                },
                Box::new(self.clone()),
            )),
            Box::new(Promote::new(
                Piece::Queen { color: *color },
                Box::new(self.clone()),
            )),
        ]))
    }

    fn to_algebraic_notation(&self, board: &Board) -> String {
        let piece_char = match board.piece_at_mailbox_index(self.start) {
            board::Square::Inside(option) => match option {
                Some(piece) => match piece {
                    Piece::Pawn { .. } => Board::get_file(self.start),
                    Piece::Bishop { .. } => 'B',
                    Piece::Knight { .. } => 'N',
                    Piece::Rook { .. } => 'R',
                    Piece::Queen { .. } => 'Q',
                    Piece::King { .. } => 'K',
                },
                None => {
                    panic!("Should not have happened : A move was created without a valid piece")
                }
            },
            board::Square::Outside => {
                panic!("Should not have happened : A move was created without a valid square")
            }
        };
        let string = piece_char.to_string();
        string + Board::get_file(self.end).to_string().as_str() + self.end.to_string().as_str()
    }

    fn target_square(&self) -> usize {
        self.end
    }

    fn start_square(&self) -> usize {
        self.start
    }

    fn double_forward(&self) -> Option<(usize, usize)> {
        if self.end.abs_diff(self.start) == 2 * BOARD_X {
            return Some((self.start.average_floor(&self.end), self.end));
        }
        None
    }
}
