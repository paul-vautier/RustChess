use crate::util::util;

use super::actions;
use super::actions::Move;
use super::actions::MovesList;
use super::board::Board;
use super::board::Square;
use super::board::BOARD_X;

const KNIGHT_OFFSETS: [i32; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];
const DIAGONAL_OFFSET: [i32; 4] = [-11, -9, 9, 11];
const LATERAL_OFFSET: [i32; 4] = [-10, -1, 1, 10];
const KING_OFFSET: [i32; 8] = [-11, -10, -9, -1, 1, 9, 10, 11];

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    WHITE,
    BLACK,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn { color: Color },
    Bishop { color: Color },
    Knight { color: Color },
    Rook { color: Color, first_move: u32 }, // Turn where the Rook first moved
    Queen { color: Color },
    King { color: Color, first_move: u32 }, // Turn where the King first moved
}

fn pawn_moves(position: usize, color: &Color, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    let direction: i32 = match color {
        Color::WHITE => -1,
        Color::BLACK => 1,
    } * BOARD_X as i32;

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
        ));
    }
    return moves;
}

fn pawn_captures(position: usize, color: &Color, board: &Board) -> MovesList {
    MovesList(Vec::new())
}

fn diagonal_moves(position: usize, piece: Piece, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in DIAGONAL_OFFSET.iter() {
        moves.append(&mut actions::get_moves_for_piece_and_direction(
            position, *direction, true, &piece, board,
        ))
    }
    return moves;
}
fn lateral_moves(position: usize, piece: Piece, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in LATERAL_OFFSET.iter() {
        moves.append(&mut actions::get_moves_for_piece_and_direction(
            position, *direction, true, &piece, board,
        ))
    }
    return moves;
}

fn knight_moves(position: usize, color: &Color, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in KNIGHT_OFFSETS.iter() {
        moves.append(&mut actions::get_moves_for_piece_and_direction(
            position,
            *direction,
            false,
            &Piece::Knight { color: *color },
            board,
        ))
    }
    return moves;
}

fn king_moves(position: usize, king: Piece, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in KING_OFFSET.iter() {
        moves.append(&mut actions::get_moves_for_piece_and_direction(
            position, *direction, false, &king, board,
        ))
    }
    return moves;
}

impl Piece {
    pub fn valid_moves(&self, position: usize, color: &Color, board: &Board) -> MovesList {
        use Piece::*;
        let moves = match self {
            Pawn { color } => pawn_moves(position, color, board),
            Bishop { color } => diagonal_moves(position, Bishop { color: *color }, board),
            Knight { color } => knight_moves(position, color, board),
            Rook { color, first_move } => lateral_moves(
                position,
                Rook {
                    color: *color,
                    first_move: *first_move,
                },
                board,
            ),
            Queen { color } => {
                let mut lateral = lateral_moves(position, Queen { color: *color }, board);
                lateral.append(&mut diagonal_moves(
                    position,
                    Queen { color: *color },
                    board,
                ));
                lateral
            }
            King { color, first_move } => king_moves(
                position,
                King {
                    color: *color,
                    first_move: *first_move,
                },
                board,
            ),
        };

        moves
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
}
