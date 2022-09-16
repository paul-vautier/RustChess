use super::actions;
use super::board::Board;
use super::board::Position;
enum Color {
    WHITE,
    BLACK
}
pub enum Piece {
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color, bool),
    Queen(Color),
    King(Color, bool)
}

fn pawn_moves(position: &Position, board : &Board) -> Vec<actions::Move>{
    return vec![]
}

fn diagonal_moves(position: &Position, board : &Board)-> Vec<actions::Move> {
    return vec![]

}
fn lateral_moves(position: &Position, board : &Board)-> Vec<actions::Move> {
    return vec![]

}

fn knight_moves(position: &Position, board : &Board)-> Vec<actions::Move> {
    return vec![]

}

fn king_moves(position: &Position, board: &Board) -> Vec<actions::Move>{
    return vec![]
}

impl Piece {
    pub fn valid_moves(&self, position: &Position, board : &Board) -> Vec<actions::Move> {
        use Piece::*;
        let moves = match self {
            Pawn(c) => pawn_moves(position, board),
            Bishop(c) => diagonal_moves(position, board),
            Knight(c) => knight_moves(position, board),
            Rook(c, _) => lateral_moves(position, board),
            Queen(c) =>  {
                let mut lateral =  lateral_moves(position, board);
                lateral.append(&mut diagonal_moves(position, board));
                lateral
            },
            King(c, _) => king_moves(position, board)
        };

        moves
    }
}