use super::board;
use super::piece;

/**
 * Command pattern : 
 * Move
 * Castle
 * Capture
 * En Passant
 */
trait ChessAction {
    fn execute(&self, board : &mut board::Board);
    fn undo(&self, board : &mut board::Board);
}

pub struct Move {
    start: board::Position,
    end: board::Position,
}
pub struct Castle {
    king: Move,
    rook: Move
}

pub struct Capture {
    position: Move,
    piece: piece::Piece
}

pub struct EnPassant {
    position: Move,
    piece: piece::Piece
}

enum Actions {
    Castle,
    Capture,
    EnPassant,
    Move,
}

impl ChessAction for Castle {
    fn execute(&self, board : &mut board::Board) {
        
    }

    fn undo(&self, board : &mut board::Board) {

    }
}

impl ChessAction for Capture {
    fn execute(&self, board : &mut board::Board) {
        
    }

    fn undo(&self, board : &mut board::Board) {

    }
}

impl ChessAction for EnPassant {
    fn execute(&self, board : &mut board::Board) {
        
    }

    fn undo(&self, board : &mut board::Board) {

    }
}

impl ChessAction for Move {
    fn execute(&self, board : &mut board::Board) {
     //TODO : Set rook moved  
    }

    fn undo(&self, board : &mut board::Board) {
     //TODO : Unset rook moved

    }
}