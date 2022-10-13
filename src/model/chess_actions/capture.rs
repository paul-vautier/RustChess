use crate::model::{
    actions::{ChessAction, MovesList},
    board::{self, Board, InvalidMoveError},
    piece::{Color, Piece},
};

use super::{movement::Move, promote::Promote};

pub struct Capture {
    pub position: Move,
    pub piece: Option<Piece>,
    pub en_passant_position: Option<usize>,
}
impl Capture {
    pub fn new(position: Move, piece: Option<Piece>, en_passant_position: Option<usize>) -> Self {
        Capture {
            position,
            piece,
            en_passant_position,
        }
    }
}
/**
 * Clones without cloning the actual piece
 */
impl Clone for Capture {
    fn clone(&self) -> Self {
        Capture {
            position: self.position.clone(),
            piece: None,
            en_passant_position: self.en_passant_position,
        }
    }
}

impl ChessAction for Capture {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        self.piece = board.move_piece(self.position.start, self.position.end)?;
        if let Some(en_passant_position) = self.en_passant_position {
            self.piece = board.remove_piece(en_passant_position);
        }
        assert!(self.piece.is_some());
        Ok(())
    }

    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        board.move_piece(self.position.end, self.position.start)?;

        let piece_pos = self
            .en_passant_position
            .map(|position| {
                board.double_pawn_move = Some((self.position.end, position));
                position
            })
            .unwrap_or(self.position.end);

        board
            .add_piece(piece_pos, self.piece.take().unwrap())
            .map_err(|error| InvalidMoveError {
                start: self.position.start,
                end: self.position.end,
                reason: error.reason,
            })?;

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

    /**
     * TODO : Deambiguous moves
     */
    fn to_algebraic_notation(&self, board: &Board) -> String {
        let piece_char = match board.piece_at_mailbox_index(self.position.start) {
            board::Square::Inside(option) => match option {
                Some(piece) => match piece {
                    Piece::Pawn { .. } => Board::get_file(self.position.start),
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
            Outside => {
                panic!("Should not have happened : A move was created without a valid square")
            }
        };
        let string = piece_char.to_string();
        string
            + "x"
            + Board::get_file(self.position.end).to_string().as_str()
            + self.position.end.to_string().as_str()
    }

    fn target_square(&self) -> usize {
        self.position.target_square()
    }

    fn start_square(&self) -> usize {
        self.position.start
    }

    fn double_forward(&self) -> Option<(usize, usize)> {
        None
    }
}
