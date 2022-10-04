use std::collections::HashMap;

use crate::util::util;

use super::actions;
use super::actions::MovesList;
use super::actions::PinState;
use super::board::Board;
use super::board::Square;
use super::board::BOARD_X;

pub const KNIGHT_OFFSETS: [i32; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];
pub const DIRECTIONS: [i32; 8] = [-10, -1, 1, 10, -11, -9, 9, 11];
pub const BLACK_PAWN: [i32; 2] = [11, 9];
pub const WHITE_PAWN: [i32; 2] = [-11, -9];

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
    WHITE,
    BLACK,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Piece {
    Pawn { color: Color },
    Bishop { color: Color },
    Knight { color: Color },
    Rook { color: Color, first_move: u32 }, // Turn where the Rook first moved
    Queen { color: Color },
    King { color: Color, first_move: u32 }, // Turn where the King first moved
}

fn pawn_moves(
    position: usize,
    color: &Color,
    board: &Board,
    resolve_check: &Vec<usize>,
    pins: &HashMap<usize, PinState>,
) -> MovesList {
    let mut moves = MovesList(Vec::new());
    let direction: i32 = match color {
        Color::WHITE => -1,
        Color::BLACK => 1,
    } * BOARD_X as i32;

    let take_right = util::add_usize(position, direction - 1);
    let take_left = util::add_usize(position, direction + 1);

    moves.extend(actions::pawn_captures(position, take_right, color, board));
    moves.extend(actions::pawn_captures(position, take_left, color, board));

    // Push one square
    if let Square::Inside(option) =
        board.piece_at_mailbox_index(util::add_usize(position, direction))
    {
        if option.is_some() {
            return moves;
        }
    };

    moves.append(&mut actions::get_moves_for_piece_and_direction(
        position,
        direction,
        false,
        &Piece::Pawn { color: *color },
        board,
        resolve_check,
        pins,
    ));

    // Push 2 squares
    if Board::is_on_pawn_flag(color, position) {
        if let Square::Inside(option) =
            board.piece_at_mailbox_index(util::add_usize(position, 2 * direction))
        {
            if option.is_some() {
                return moves;
            }
        };

        moves.append(&mut actions::get_moves_for_piece_and_direction(
            position,
            2 * direction,
            false,
            &Piece::Pawn { color: *color },
            board,
            resolve_check,
            pins,
        ));
    }

    return moves;
}

fn moves_from_slice(
    position: usize,
    directions: &[i32],
    piece: &Piece,
    board: &Board,
    resolve_check: &Vec<usize>,
    pins: &HashMap<usize, PinState>,
) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in directions {
        moves.append(&mut actions::get_moves_for_piece_and_direction(
            position,
            *direction,
            piece.is_sliding(),
            piece,
            board,
            resolve_check,
            pins,
        ))
    }
    return moves;
}

impl Piece {
    pub fn valid_moves(
        &self,
        position: usize,
        board: &Board,
        resolve_check: &Vec<usize>,
        pins: &HashMap<usize, PinState>,
    ) -> MovesList {
        use Piece::*;
        if *self.get_color() != board.color_turn() {
            return MovesList(Vec::new());
        }
        match self {
            Pawn { color } => pawn_moves(position, color, board, resolve_check, pins),
            King {
                color,
                first_move: _,
            } => {
                let mut moves = MovesList(Vec::new());
                for direction in DIRECTIONS {
                    if actions::can_king_move(board, color, position, direction) {
                        moves.append(&mut actions::get_moves_for_piece_and_direction(
                            position,
                            direction,
                            false,
                            self,
                            board,
                            &vec![],
                            &HashMap::new(),
                        ))
                    }
                }
                moves.append(&mut actions::castles(position, self, board));
                moves
            }
            _ => moves_from_slice(
                position,
                self.get_direction(),
                self,
                board,
                resolve_check,
                pins,
            ),
        }
    }

    pub fn get_color(&self) -> &Color {
        use Piece::*;
        match self {
            Pawn { color } => color,
            Bishop { color } => color,
            Knight { color } => color,
            Rook { color, .. } => color,
            Queen { color } => color,
            King { color, .. } => color,
        }
    }

    pub fn is_sliding(&self) -> bool {
        match self {
            Piece::Pawn { .. } => false,
            Piece::Bishop { .. } => true,
            Piece::Knight { .. } => false,
            Piece::Rook { .. } => true,
            Piece::Queen { .. } => true,
            Piece::King { .. } => false,
        }
    }

    pub fn get_direction(&self) -> &[i32] {
        match self {
            Piece::Pawn { .. } => panic!("The pawn is a special case"),
            Piece::Bishop { .. } => &DIRECTIONS[4..8],
            Piece::Knight { .. } => &KNIGHT_OFFSETS,
            Piece::Rook { .. } => &DIRECTIONS[0..4],
            Piece::Queen { .. } => &DIRECTIONS[0..8],
            Piece::King { .. } => &DIRECTIONS[0..8],
        }
    }

    pub fn get_attack_direction(&self) -> &[i32] {
        match self {
            Piece::Pawn { color } => match color {
                Color::WHITE => &WHITE_PAWN,
                Color::BLACK => &BLACK_PAWN,
            },
            Piece::Bishop { .. } => &DIRECTIONS[4..8],
            Piece::Knight { .. } => &KNIGHT_OFFSETS,
            Piece::Rook { .. } => &DIRECTIONS[0..4],
            Piece::Queen { .. } => &DIRECTIONS[0..8],
            Piece::King { .. } => &DIRECTIONS[0..8],
        }
    }

    pub fn has_direction(&self, direction: i32) -> bool {
        self.get_direction().contains(&direction)
    }
}
