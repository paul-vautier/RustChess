use std::ops::{Deref, DerefMut};
use crate::model::board::Square::*;

use super::board::{self, Board};
use super::piece::{Color, Piece};

/**
 * Command pattern :
 * Move
 * Castle
 * Capture
 * En Passant
 */
pub trait ChessAction {
    fn execute(&self, board: &mut Board);
    fn undo(&self, board: &mut Board);
    fn is_valid(&self, board: &Board) -> bool;
    fn as_promotion(&self, color: &Color) -> Result<MovesList, String>;
    fn to_algebraic_notation(&self, board: &Board) -> String;
}

pub struct MovesList(pub Vec<Box<dyn ChessAction>>);

impl MovesList {
    pub fn to_algebraic_notation(&self, board: &Board) -> String {
        let mut result = String::from("");
        for (index, current) in self.0.iter().enumerate() {
            result += format!(
                "{0} : {1}\n",
                index.to_string(),
                current.to_algebraic_notation(board)
            )
            .as_str()
        }
        String::from(result)
    }
}

impl Deref for MovesList {
    type Target = Vec<Box<dyn ChessAction>>;
    fn deref(&self) -> &Vec<Box<dyn ChessAction>> {
        &self.0
    }
}

impl DerefMut for MovesList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
pub struct Move {
    pub start: usize,
    pub end: usize,
}

impl Clone for Move {
    fn clone(&self) -> Self {
        Move {
            start: self.start.clone(),
            end: self.start.clone(),
        }
    }
}

pub struct Promote {
    pub piece: Piece,
    pub previous_action: Box<dyn ChessAction>,
}

pub struct Castle {
    pub king: Move,
    pub rook: Move,
}

pub struct Capture {
    pub position: Move,
    pub piece: Option<Piece>,
    pub en_passant: bool,
}

/**
 * Clones without cloning the actual piece
 */
impl Clone for Capture {
    fn clone(&self) -> Self {
        Capture {
            position: self.position.clone(),
            piece: None,
            en_passant: self.en_passant,
        }
    }
}

impl ChessAction for Castle {
    fn execute(&self, board: &mut Board) {}

    fn undo(&self, board: &mut Board) {}

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
}

impl ChessAction for Capture {
    fn execute(&self, board: &mut Board) {}

    fn undo(&self, board: &mut Board) {}

    fn is_valid(&self, board: &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self, color: &Color) -> Result<MovesList, String> {
        Ok(MovesList(vec![
            Box::new(Promote {
                piece: Piece::Bishop(*color),
                previous_action: Box::new(self.clone()),
            }),
            Box::new(Promote {
                piece: Piece::Rook(*color, u32::MAX),
                previous_action: Box::new(self.clone()),
            }),
            Box::new(Promote {
                piece: Piece::Queen(*color),
                previous_action: Box::new(self.clone()),
            }),
        ]))
    }

    /**
     * TODO : Deambiguous moves
     */
    fn to_algebraic_notation(&self, board: &Board) -> String {
        let piece_char = match board.piece_at_mailbox_index(self.position.start) {
            super::board::Square::Inside(option) => match option {
                Some(piece) => match piece {
                    Piece::Pawn(_) => Board::get_file(self.position.start),
                    Piece::Bishop(_) => 'B',
                    Piece::Knight(_) => 'N',
                    Piece::Rook(_, _) => 'R',
                    Piece::Queen(_) => 'Q',
                    Piece::King(_, _) => 'K',
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
}

impl ChessAction for Move {
    fn execute(&self, board: &mut Board) {
        //TODO : Set rook moved
    }

    fn undo(&self, board: &mut Board) {
        //TODO : Unset rook moved
    }

    fn is_valid(&self, board: &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self, color: &Color) -> Result<MovesList, String> {
        todo!()
    }

    fn to_algebraic_notation(&self, board: &Board) -> String {
        let piece_char = match board.piece_at_mailbox_index(self.start) {
            super::board::Square::Inside(option) => match option {
                Some(piece) => match piece {
                    Piece::Pawn(_) => Board::get_file(self.start),
                    Piece::Bishop(_) => 'B',
                    Piece::Knight(_) => 'N',
                    Piece::Rook(_, _) => 'R',
                    Piece::Queen(_) => 'Q',
                    Piece::King(_, _) => 'K',
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
        string + Board::get_file(self.end).to_string().as_str() + self.end.to_string().as_str()
    }
}

impl ChessAction for Promote {
    fn execute(&self, board: &mut Board) {
        //TODO : Set rook moved
    }

    fn undo(&self, board: &mut Board) {
        //TODO : Unset rook moved
    }

    fn is_valid(&self, board: &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self, color: &Color) -> Result<MovesList, String> {
        Err("Cannot call 'as_promotion' on move 'Promotion'".to_string())
    }

    fn to_algebraic_notation(&self, board: &Board) -> String {
        todo!()
    }
}


pub fn get_moves_for_piece_and_position(
    start: usize,
    end: usize,
    current_piece: &Piece,
    board: &Board,
) -> MovesList {
    let move_option: Option<Box<dyn ChessAction>> = match board.piece_at_mailbox_index(end) {
            Outside => Some(Box::new(Move {
                start: start.clone(),
                end,
            })),

            Inside(option) => match option {
                Some(piece) => {
                    if piece.get_color() != current_piece.get_color() {
                        let position = Move {
                            start: start.clone(),
                            end,
                        };
                        let en_passant = *current_piece
                            == Piece::Pawn(*current_piece.get_color())
                            && start != position.end;
                        let capture = Capture {
                            position,
                            piece: None,
                            en_passant,
                        };
                        Some(Box::new(capture))
                    } else {
                        None
                    }
                }
                None => Some(Box::new(Move {
                    start: start.clone(),
                    end,
                })),
            },
        };
    move_option
        .map(|retrieved_move| {
            if *current_piece == Piece::Pawn(*current_piece.get_color())
                && Board::is_on_promote_flag(current_piece.get_color(), end) 
            {
                match retrieved_move.as_promotion(current_piece.get_color()) {
                    Ok(promotions) => promotions,
                    Err(_) => MovesList(vec![retrieved_move]),
                }
            } else {
                MovesList(vec![retrieved_move])
            }
        })
        .unwrap_or(MovesList(Vec::new()))
}

pub fn get_pawn_capture() -> MovesList {
    return MovesList(Vec::new())
}