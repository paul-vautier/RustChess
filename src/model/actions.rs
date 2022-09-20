use super::board::{Position, Board, self};
use super::piece::{Piece};

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
    fn as_promotion(&self) -> Result<MovesList, String>;
    fn to_algebraic_notation(&self, board: &Board) -> String;
}

pub struct MovesList(pub Vec<Box<dyn ChessAction>>);

impl MovesList {
    pub fn to_algebraic_notation(&self, board : &Board) -> String {
        let mut result = String::from("");
        for (index, current) in self.0.iter().enumerate() {
            result += format!("{0} : {1}\n", index.to_string(), current.to_algebraic_notation(board)).as_str()
        }
        String::from(result)
    }
}
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

    fn as_promotion(&self) -> Result<MovesList, String> {
        todo!()
    }

    fn to_algebraic_notation(&self, board: &Board) -> String {
        if (self.king.start.x > self.rook.start.x) {
            String::from("OwOwO")
        } else {
            String::from("OwO")
        }
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

    fn as_promotion(&self) -> Result<MovesList, String> {
        todo!()
    }

    /**
     * TODO : Deambiguous moves
     */
    fn to_algebraic_notation(&self, board: &Board) -> String {
        let piece_char = match board.piece_at(&self.position.start) {
            super::board::Square::Inside(option) => match option {
                Some(piece) => match piece {
                    Piece::Pawn(_) => board::get_file(self.position.start.x),
                    Piece::Bishop(_) => 'B',
                    Piece::Knight(_) => 'N',
                    Piece::Rook(_, _) => 'R',
                    Piece::Queen(_) => 'Q',
                    Piece::King(_, _) => 'K',
                },
                None => panic!("Should not have happened : A move was created without a valid piece"),
            },
            Outside => panic!("Should not have happened : A move was created without a valid square"),
        };
        let string = piece_char.to_string();
        string + "x" + board::get_file(self.position.end.x).to_string().as_str() + self.position.end.y.to_string().as_str()
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

    fn as_promotion(&self) -> Result<MovesList, String> {
        todo!()
    }

    fn to_algebraic_notation(&self, board: &Board) -> String {
        let piece_char = match board.piece_at(&self.start) {
            super::board::Square::Inside(option) => match option {
                Some(piece) => match piece {
                    Piece::Pawn(_) => board::get_file(self.start.x),
                    Piece::Bishop(_) => 'B',
                    Piece::Knight(_) => 'N',
                    Piece::Rook(_, _) => 'R',
                    Piece::Queen(_) => 'Q',
                    Piece::King(_, _) => 'K',
                },
                None => panic!("Should not have happened : A move was created without a valid piece"),
            },
            Outside => panic!("Should not have happened : A move was created without a valid square"),
        };
        let string = piece_char.to_string();
        string + board::get_file(self.end.x).to_string().as_str() + self.end.y.to_string().as_str()
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

    fn as_promotion(&self) -> Result<MovesList, String> {
        todo!()
    }

    fn to_algebraic_notation(&self, board: &Board) -> String {
        todo!()
    }
}