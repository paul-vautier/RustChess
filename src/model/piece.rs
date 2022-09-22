use crate::util::util;

use super::actions;
use super::actions::MovesList;
use super::board;
use super::board::BOARD_X;
use super::board::Board;
use super::board::Position;
#[derive(PartialEq)]
pub enum Color {
    WHITE,
    BLACK,
}

impl Clone for Color {
    fn clone(&self) -> Self {
        match self {
            Self::WHITE => Self::WHITE,
            Self::BLACK => Self::BLACK,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Piece {
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color, bool),
    Queen(Color),
    King(Color, bool),
}

fn pawn_moves(position: usize, color: &Color, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    let direction: i32 = match color {
        Color::WHITE => -1,
        Color::BLACK => 1,
    };
    
    moves.append(
        &mut actions::get_moves_for_piece_and_position(
            position,
            util::add_usize(position, BOARD_X as i32 * direction),
            Piece::Pawn(color.clone()),
            board,
        ),
    );
    moves.append(
        &mut actions::get_moves_for_piece_and_position(
            position,
            util::add_usize(position, 2 * BOARD_X as i32 * direction),
            Piece::Pawn(color.clone()),
            board,
        ),
    );
    return moves;
}

fn diagonal_moves(position: usize, color: &Color, board: &Board) -> MovesList {
    return MovesList(vec![]);
}
fn lateral_moves(position: usize, color: &Color, board: &Board) -> MovesList {
    return MovesList(vec![]);
}

fn knight_moves(position: usize, color: &Color, board: &Board) -> MovesList {
    return MovesList(vec![]);
}

fn king_moves(position: usize, color: &Color, board: &Board) -> MovesList {
    return MovesList(vec![]);
}

impl Piece {
    pub fn valid_moves(&self, position: usize, color: &Color, board: &Board) -> MovesList {
        use Piece::*;
        let moves = match self {
            Pawn(c) => pawn_moves(position, c, board),
            Bishop(c) => diagonal_moves(position, c, board),
            Knight(c) => knight_moves(position, c, board),
            Rook(c, _) => lateral_moves(position, c, board),
            Queen(c) => {
                let mut lateral = lateral_moves(position, c, board);
                lateral.append(&mut diagonal_moves(position, c, board));
                lateral
            }
            King(c, _) => king_moves(position, c, board),
        };

        moves
    }

    pub fn get_color(&self) -> &Color {
        use Piece::*;
        match self {
            Pawn(c) => c,
            Bishop(c) => c,
            Knight(c) => c,
            Rook(c, _) => c,
            Queen(c) => c,
            King(c, _) => c,
        }
    }
}
