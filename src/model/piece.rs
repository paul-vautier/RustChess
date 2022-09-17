use super::actions;
use super::actions::Capture;
use super::actions::ChessAction;
use super::actions::Move;
use super::board;
use super::board::Board;
use super::board::Position;
#[derive(PartialEq)]
pub enum Color {
    WHITE,
    BLACK
}

impl Clone for Color {
    fn clone(&self) -> Self {
        match self {
            Self::WHITE => Self::WHITE,
            Self::BLACK => Self::BLACK,
        }
    }
}

#[derive(PartialEq)]
pub enum Piece {
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color, bool),
    Queen(Color),
    King(Color, bool)
}

fn get_move(start : &Position, current_piece : Piece, x : usize, y : usize, board : &Board) -> Vec<Box<dyn ChessAction>> {
    let mut end = Position{x : 0, y : 0};
    let move_option : Option<Box<dyn ChessAction>> = match Position::new(x, y).map(|end_pos| {
        end = end_pos;
        board.piece_at(&end)
    }) {
        Ok(option) => match option {
            Some(piece) => {
                if piece.getColor() != current_piece.getColor() {
                    let position = Move{
                        start: start.clone(), 
                        end
                    };
                    let en_passant = current_piece == Piece::Pawn(current_piece.getColor().clone()) && start.y != position.end.y;
                    let capture = Capture{
                        position,
                        piece : None,
                        en_passant
                    };
                    Some(Box::new(capture))
                } else {
                    None
                }
            }
            None => Some(Box::new(Move{start: start.clone(), end}))
        }
        _ => None,
    };
    move_option.map(|retrieved_move| {
        if current_piece == Piece::Pawn(current_piece.getColor().clone()) && Board::promote_flag(current_piece.getColor()) == y {
            match retrieved_move.as_promotion() {
                Ok(promotions) => promotions,
                Err(_) => vec![retrieved_move],
            }
        } else {
            vec![retrieved_move]
        }
    }).unwrap_or_else(Vec::new)
}
/**
 * Compares the current pieces in a position
 */
fn compare_with(position: &Position, piece : &Piece, board : &Board) -> bool {
    match board.piece_at(position) {
        Some(board_piece) => board_piece == piece,
        None => false,
    }
}

fn pawn_moves(position: &Position, color: &Color, board : &Board) -> Vec<Box<dyn ChessAction>> {
    let mut moves : Vec<Box<dyn ChessAction>> = Vec::new();
    let direction : i32 = match color {
        Color::WHITE => -1,
        Color::BLACK => 1,
    };
    moves.append(&mut get_move(position, Piece::Pawn(color.clone()), position.x, board::add_usize(position.y, direction), board));
    moves.append(&mut get_move(position, Piece::Pawn(color.clone()), position.x, board::add_usize(position.y, 2 * direction), board));
    return moves
}

fn diagonal_moves(position: &Position, color: &Color, board : &Board) -> Vec<Box<dyn ChessAction>> {
    return vec![]
}
fn lateral_moves(position: &Position, color: &Color, board : &Board) -> Vec<Box<dyn ChessAction>> {
    return vec![]
}

fn knight_moves(position: &Position, color: &Color, board : &Board) -> Vec<Box<dyn ChessAction>> {
    return vec![]

}

fn king_moves(position: &Position, color: &Color, board: &Board) -> Vec<Box<dyn ChessAction>>{
    return vec![]
}

impl Piece {
    pub fn valid_moves(&self, position: &Position, color: &Color, board : &Board) -> Vec<Box<dyn ChessAction>> {
        use Piece::*;
        let moves = match self {
            Pawn(c) => pawn_moves(position, c, board),
            Bishop(c) => diagonal_moves(position, c, board),
            Knight(c) => knight_moves(position, c, board),
            Rook(c, _) => lateral_moves(position, c, board),
            Queen(c) =>  {
                let mut lateral =  lateral_moves(position, c, board);
                lateral.append(&mut diagonal_moves(position, c, board));
                lateral
            },
            King(c, _) => king_moves(position, c, board)
        };

        moves
    }

    pub fn getColor(&self) -> &Color { 
        use Piece::*;
        match self {
            Pawn(c) => c,
            Bishop(c) => c,
            Knight(c) => c,
            Rook(c, _) => c,
            Queen(c) =>  c,
            King(c, _) => c
        }
    }
}