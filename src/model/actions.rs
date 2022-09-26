use crate::model::board::Square::*;
use crate::util::util;
use std::ops::{Deref, DerefMut};

use super::board::{Board, InvalidMoveError};
use super::piece::{Color, Piece};

/**
 * Command pattern :
 * Move
 * Castle
 * Capture
 * En Passant
 */
pub trait ChessAction {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError>; 
    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError>;
    fn is_valid(&self, board: &Board) -> bool;
    fn as_promotion(&self, color: &Color) -> Result<MovesList, String>;
    fn to_algebraic_notation(&self, board: &Board) -> String;
    fn target_square(&self) -> usize;
    fn start_square(&self) -> usize;
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
pub struct Castle {
    pub king: Move,
    pub rook: Move,
}
impl Castle {
    pub fn new(king: Move, rook: Move) -> Self {
        Castle { king, rook }
    }
}
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

impl ChessAction for Castle {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError>{
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
}

impl ChessAction for Capture {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        self.piece = board.move_piece(self.position.start, self.position.end)?;
        Ok(())
    }

    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        board.move_piece(self.position.end, self.position.start)?;
        let piece_pos = self.en_passant_position.map(|position| position).unwrap_or(self.position.end);
        board.add_piece(piece_pos, self.piece.take().unwrap())
            .map_err(|error| InvalidMoveError{start : self.position.start, end: self.position.end, reason: error.reason})?;
       Ok(()) 
    }

    fn is_valid(&self, board: &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self, color: &Color) -> Result<MovesList, String> {
        Ok(MovesList(vec![
            Box::new(Promote {
                piece: Piece::Bishop { color: *color },
                previous_action: Box::new(self.clone()),
            }),
            Box::new(Promote {
                piece: Piece::Rook {
                    color: *color,
                    first_move: 0,
                },
                previous_action: Box::new(self.clone()),
            }),
            Box::new(Promote {
                piece: Piece::Queen { color: *color },
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
}

impl ChessAction for Move {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError>{
        board.move_piece(self.start, self.end)?;
        Ok(())
    }

    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError>{
        board.move_piece(self.end, self.start)?;
        Ok(())
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
            Outside => {
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
}

impl ChessAction for Promote {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError>{
        todo!()
        //TODO : Set rook moved
    }

    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError>{
        //TODO : Unset rook moved
        todo!()
    }

    fn is_valid(&self, board: &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self, color: &Color) -> Result<MovesList, String> {
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
}

pub fn get_moves_for_piece_and_direction(
    start: usize,
    direction: i32,
    is_slide: bool,
    current_piece: &Piece,
    board: &Board,
) -> MovesList {
    let mut moves = MovesList(Vec::new());
    let mut end = util::add_usize(start, direction);
    loop {
        let move_option: Option<Box<dyn ChessAction>> = match board.piece_at_mailbox_index(end) {
            Outside => break,
            Inside(option) => match option {
                Some(piece) => {
                    if piece.get_color() != current_piece.get_color() {
                        let capture = Capture::new(Move::new(start, end), None, None);
                        moves.push(Box::new(capture));
                    }
                    break;
                }
                None => Some(Box::new(Move::new(start, end))),
            },
        };

        moves.append(&mut to_promotion(move_option, current_piece, end));
        if !is_slide {
            break;
        }

        end = util::add_usize(end, direction);
    }
    moves
}

fn to_promotion(
    move_option: Option<Box<dyn ChessAction>>,
    current_piece: &Piece,
    end: usize,
) -> MovesList {
    move_option
        .map(|retrieved_move| {
            if *current_piece
                == (Piece::Pawn {
                    color: *current_piece.get_color(),
                })
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
