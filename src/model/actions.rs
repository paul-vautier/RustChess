use crate::model::board::Square::*;
use crate::util::util;
use std::ops::{Deref, DerefMut};

use super::board::{Board, InvalidMoveError, Square};
use super::chess_actions::capture::Capture;
use super::chess_actions::castle::Castle;
use super::chess_actions::movement::Move;
use super::piece::{Color, Piece, self};

/**
 * Command pattern :
 * Move
 * Castle
 * Capture
 * Promotion
 */
pub trait ChessAction {
    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError>;
    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError>;
    fn is_valid(&self, board: &Board) -> bool;
    fn as_promotion(&self, color: &Color) -> Result<MovesList, String>;
    fn to_algebraic_notation(&self, board: &Board) -> String;
    fn target_square(&self) -> usize;
    fn start_square(&self) -> usize;
    fn double_forward(&self) -> Option<(usize, usize)>;
}

pub struct MovesList(pub Vec<Box<dyn ChessAction>>);

pub struct Pin {
    position: usize,
    restricting_axis: i32,
}

pub struct BoardAttackData {
    pub white_king: usize,
    pub black_king: usize,
    pins: Vec<Pin>,
    resolve_check: Vec<usize>,
}

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

pub fn can_king_move(board: &Board, king_color: &Color, position: usize) -> bool {
    for direction in piece::DIRECTIONS {
        if let Some((hit, piece)) = board.ray(position, direction) {
            if *piece.get_color() != *king_color && 
            ((piece.is_sliding() && piece.has_direction(-direction)) || piece.get_attack_direction().contains(&((position-hit) as i32)))    {
                return false;
            }
        }
    }
    for direction in piece::KNIGHT_OFFSETS {
        if let Inside(Some(Piece::Knight { color})) = board.piece_at_mailbox_index(util::add_usize(position, direction)) {
            if color != king_color {
                return false;
            }
        }
    }
    true
}

pub fn generates_moves(board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    let playing_color = board.color_turn();
    let king_position = board.get_king_by_color(&playing_color);
    let pins : Vec<usize> = vec![];
    let mut double_check = false;
    for direction in piece::DIRECTIONS {
        if let Some((position, piece)) = board.ray(king_position, direction) {
            if *piece.get_color() == playing_color {
                // If potential pin, sadly we can't combine condition as if let && are still unstable 
                if let Some((pos, behind)) = board.ray(position, direction) {
                    // Pinned
                    if *behind.get_color() != playing_color && piece.is_sliding() && piece.has_direction(-direction) {
                        
                    } 
                }
            } else {
                // King in check
                if piece.is_sliding() && piece.has_direction(-direction) {
                    if !pins.is_empty() {
                        double_check = true;
                        break;
                    }
                }
            }
        }
    }

    // King must move
    if double_check {
        if let Inside(Some(piece @Piece::King{ color: _, first_move: _ })) = board.piece_at_mailbox_index(king_position) {
            return piece.valid_moves(king_position, board)
        } else {
            panic!("invalid king position")
        }
    }

    for (index, piece) in board.iter() {
        match piece {
            Some(piece) => moves.append(&mut piece.valid_moves(index, board)),
            None => (),
        }
    }

    moves
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

pub fn pawn_captures(
    from: usize,
    to: usize,
    color: &Color,
    board: &Board,
) -> Option<Box<dyn ChessAction>> {
    if let Square::Inside(Some(piece)) = board.piece_at_mailbox_index(to) {
        if piece.get_color() != color {
            return Some(Box::new(Capture::new(Move::new(from, to), None, None)));
        }
    } else if let Some((ghost, pawn)) = board.double_pawn_move {
        if ghost == to {
            return Some(Box::new(Capture::new(
                Move::new(from, to),
                None,
                Some(pawn),
            )));
        }
    }
    None
}

/**
 * Trangression : we should check that the rook and king are on the last row
 */
pub fn castles(king_position: usize, piece: &Piece, board: &Board) -> MovesList {
    if let Piece::King {
        color: _,
        first_move,
    } = piece
    {
        if *first_move != u32::MAX {
            return MovesList(Vec::new());
        }
        let mut moves = MovesList(Vec::new());
        if let Some((
            pos,
            Piece::Rook {
                color: _,
                first_move,
            },
        )) = board.ray(king_position, -1)
        {
            if *first_move == u32::MAX {
                moves.push(Box::new(Castle::new(
                    Move::new(king_position, king_position - 2),
                    Move::new(pos, king_position - 1),
                )))
            }
        }
        if let Some((
            pos,
            Piece::Rook {
                color: _,
                first_move,
            },
        )) = board.ray(king_position, 1)
        {
            if *first_move == u32::MAX {
                moves.push(Box::new(Castle::new(
                    Move::new(king_position, king_position + 2),
                    Move::new(pos, king_position + 1),
                )))
            }
        }

        return moves;
    } else {
        return MovesList(Vec::new());
    }
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
