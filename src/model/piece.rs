use crate::util::util;

use super::actions;
use super::actions::MovesList;
use super::board::Board;
use super::board::Square;
use super::board::BOARD_X;

const KNIGHT_OFFSETS: [i32; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];
const DIRECTION: [i32; 8] = [-10, -1, 1, 10, -11, -9, 9, 11];

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

fn moves_from_slice(
    position: usize,
    directions: &[i32],
    piece: &Piece,
    board: &Board,
) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in directions.iter() {
        moves.append(&mut actions::get_moves_for_piece_and_direction(
            position,
            *direction,
            piece.is_sliding(),
            piece,
            board,
        ))
    }
    return moves;
}

impl Piece {
    pub fn valid_moves(&self, position: usize, board: &Board) -> MovesList {
        use Piece::*;
        let mut moves = MovesList(Vec::new());
        match self {
            Pawn { color } => pawn_moves(position, color, board),
            piece => {
                moves.append(&mut moves_from_slice(position, self.get_direction(), self, board));
                if let Piece::King { color: _, first_move: _ } = piece{
                    moves.append(&mut actions::castles(position, self, board))   
                }
                moves
            }
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
            Piece::Bishop { .. } => &DIRECTION[4..8],
            Piece::Knight { .. } => &KNIGHT_OFFSETS,
            Piece::Rook { .. } => &DIRECTION[0..4],
            Piece::Queen { .. } => &DIRECTION[0..8],
            Piece::King { .. } => &DIRECTION[0..8],
        }
    }
}
