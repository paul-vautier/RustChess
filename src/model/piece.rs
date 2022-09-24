use crate::util::util;

use super::actions;
use super::actions::MovesList;
use super::board::BOARD_X;
use super::board::Board;
use super::board::Square;



const KNIGHT_OFFSETS : [i32; 8] = [-21, -19,-12, -8, 8, 12, 19, 21];
const LATERAL_OFFSET : [i32; 4] = [-11, -9,  9, 11];
const DIAGONAL_OFFSET : [i32; 4] = [ -10, -1,  1, 10];
const KING_OFFSET : [i32; 8] = [-11, -10, -9, -1, 1,  9, 10, 11];

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    WHITE,
    BLACK,
} 

#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color, u32), // Turn where the Rook first moved
    Queen(Color),
    King(Color, u32), // Turn where the King first moved
}

fn pawn_moves(position: usize, color: &Color, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    let direction: i32 = match color {
        Color::WHITE => -1,
        Color::BLACK => 1,
    };

    if let Square::Inside(option) = board.piece_at_mailbox_index(util::add_usize(position, BOARD_X as i32 * direction)) {
        if option.is_some() {
            return moves;
        }
    };

    moves.append(
        &mut actions::get_moves_for_piece_and_position(
            position,
            util::add_usize(position, BOARD_X as i32 * direction),
            &Piece::Pawn(*color),
            board,
        ),
    );

    if let Square::Inside(option) = board.piece_at_mailbox_index(util::add_usize(position, 2 * BOARD_X as i32 * direction)) {
        if option.is_some() {
            return moves;
        }
    };

    moves.append(
        &mut actions::get_moves_for_piece_and_position(
            position,
            util::add_usize(position, 2 * BOARD_X as i32 * direction),
            &Piece::Pawn(*color),
            board,
        ),
    );
    return moves;
}

fn diagonal_moves(position: usize, piece: Piece, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in DIAGONAL_OFFSET.iter() {
        moves.append(&mut slide_in_direction(position, &piece, board, *direction))
    }
    return moves;
}
fn lateral_moves(position: usize, piece: Piece, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in LATERAL_OFFSET.iter() {
        moves.append(&mut slide_in_direction(position, &piece, board, *direction))
    }
    return moves;
}

fn knight_moves(position: usize, color: &Color, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in KNIGHT_OFFSETS.iter() {
        moves.append(&mut actions::get_moves_for_piece_and_position(position, util::add_usize(position, *direction), &Piece::Knight(*color), board))
    }
    return moves;
}

fn king_moves(position: usize, king: Piece, board: &Board) -> MovesList {
    let mut moves = MovesList(Vec::new());
    for direction in KING_OFFSET.iter() {
        moves.append(&mut actions::get_moves_for_piece_and_position(position, util::add_usize(position, *direction),  &king, board))
    }
    return moves;
}

fn slide_in_direction(position: usize, piece: &Piece, board: &Board, direction : i32) -> MovesList {
    let mut moves = MovesList(Vec::new());
    let mut curr = util::add_usize(position,  direction);
    loop {
        let retrieved = &mut actions::get_moves_for_piece_and_position(position, curr, piece, board);
        if retrieved.is_empty() {
            break;
        }
        moves.append(retrieved);
        curr = util::add_usize(curr, direction)
    }
    return moves;
}
impl Piece {
    pub fn valid_moves(&self, position: usize, color: &Color, board: &Board) -> MovesList {
        use Piece::*;
        let moves = match self {
            Pawn(c) => pawn_moves(position, c, board),
            Bishop(c) => diagonal_moves(position, Bishop(*color), board),
            Knight(c) => knight_moves(position, c, board),
            Rook(c, t) => lateral_moves(position, Rook(*c, *t), board),
            Queen(c) => {
                let mut lateral = lateral_moves(position, Queen(*color), board);
                lateral.append(&mut diagonal_moves(position, Queen(*color), board));
                lateral
            }
            King(c, t) => king_moves(position, King(*c, *t), board),
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
