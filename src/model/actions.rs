use super::board::{Position, Board};
use super::piece::{self, Piece, Color};

/**
 * Command pattern : 
 * Move
 * Castle
 * Capture
 * En Passant
 */
pub trait ChessAction {
    fn execute(&self, board : &mut Board);
    fn undo(&self, board : &mut Board);
    fn is_valid(&self, board : &Board) -> bool;
    fn as_promotion(&self) -> Result<Moves, String>;
    fn to_algebraic_notation(&self) -> String;
}

pub struct Moves(pub Vec<Box<dyn ChessAction>>);
pub struct Move {
    pub start: Position,
    pub end: Position,
}

impl Clone for Move {
    fn clone(&self) -> Self {
        Move{start: self.start.clone(), end :  self.start.clone()}
    }
}

pub struct Promote {
    pub piece: Piece,
    pub previous_action : Box<dyn ChessAction>,
}

pub struct Castle {
    pub king: Move,
    pub rook: Move
}

pub struct Capture {
    pub position: Move,
    pub piece: Option<Piece>,
    pub en_passant: bool,
}

impl Clone for Capture {
    fn clone(&self) -> Self {
        Capture{position : self.position.clone(), piece : None, en_passant : self.en_passant}
    }
}

impl ChessAction for Castle {
    fn execute(&self, board : &mut Board) {
        
    }

    fn undo(&self, board : &mut Board) {

    }

    fn is_valid(&self, board : &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self) -> Result<Moves, String> {
        todo!()
    }

    fn to_algebraic_notation(&self) -> String {
        todo!()
    }
}

impl ChessAction for Capture {
    fn execute(&self, board : &mut Board) {
        
    }

    fn undo(&self, board : &mut Board) {

    }

    fn is_valid(&self, board : &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self) -> Result<Moves, String> {
        todo!()
    }

    fn to_algebraic_notation(&self) -> String {
        todo!()
    }
}

impl ChessAction for Move {
    fn execute(&self, board : &mut Board) {
     //TODO : Set rook moved  
    }

    fn undo(&self, board : &mut Board) {
     //TODO : Unset rook moved

    }

    fn is_valid(&self, board : &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self) -> Result<Moves, String> {
        todo!()
    }

    fn to_algebraic_notation(&self) -> String {
        todo!()
    }
}

impl ChessAction for Promote {
    fn execute(&self, board : &mut Board) {
     //TODO : Set rook moved  
    }

    fn undo(&self, board : &mut Board) {
     //TODO : Unset rook moved

    }

    fn is_valid(&self, board : &Board) -> bool {
        todo!()
    }

    fn as_promotion(&self) -> Result<Moves, String> {
        todo!()
    }

    fn to_algebraic_notation(&self) -> String {
        todo!()
    }
}