use super::piece::Piece;
use super::piece::Color;


pub const BOARD_X : usize = 10;
pub const BOARD_Y : usize = 12;
pub const BOARD_SIZE : usize = BOARD_X * BOARD_Y;
pub const BLACK_ROW : usize = 2;
pub const WHITE_ROW : usize = 9;

const mailbox_indices :[usize; 64] = [   
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];

pub struct InvalidBoardErr {
    pub err: String
}


#[derive(Copy, Clone)]
pub struct Position {
    pub x : usize, 
    pub y : usize,
}

pub enum Square {
    Inside(Option<Piece>),
    Outside
}

pub struct Board {
    pub mailbox: [Square; BOARD_SIZE],
}

impl Board {
    /**
     * Position on the actual board, from 0 to 64
     */
    pub fn piece_at_board_index(&self, position : usize) -> &Square {
        &self.mailbox[mailbox_indices[position] as usize]
    }

    /**
     * Position on the actual board, from 0 to 120
     */
    pub fn piece_at_mailbox_index(&self, position : usize) -> &Square {
        &self.mailbox[position]
    }

    pub fn promote_flag(color : &Color) -> usize {
        match color {
            Color::WHITE => BLACK_ROW,
            Color::BLACK => WHITE_ROW,
        }
    }

    pub fn is_on_promote_flag(color : &Color, index: usize) -> bool {
        match color {
            Color::WHITE => index / BOARD_X == BLACK_ROW,
            Color::BLACK => index / BOARD_X == WHITE_ROW,
        }
    }

    pub fn to_mailbox_index(x: usize, y: usize) -> usize {
        return mailbox_indices[x + 8 * y];
    }

    pub fn get_color_fen(c : char) -> Color{
        if c.is_lowercase() {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }
    
    pub fn get_file(index : usize) -> char {
        match index % BOARD_X {
           0 => 'a', 
           1 => 'b', 
           2 => 'c', 
           3 => 'd', 
           4 => 'e', 
           5 => 'f', 
           6 => 'g', 
           7 => 'h', 
           _ => 'x',
        }
    }
    pub fn from_fen(notation : String) -> Result<Board,InvalidBoardErr>  {
        let mut mailbox = [(); BOARD_SIZE].map(|_| Outside);
        let mut offset : usize = 2 * BOARD_X + 1;
        let mut index : usize =  offset;
    
        use Square::*;
        for (i, c) in notation.chars().into_iter().enumerate() {
            match c.to_lowercase().next() {
                Some(current) => match current {
                    // TODO : VERIFY KING MOVED
                    'k' =>  {
                        mailbox[index] = Inside(Some(Piece::King(Board::get_color_fen(c), false)));
                    }
                    'q' =>  {
                        mailbox[index] = Inside(Some(Piece::Queen(Board::get_color_fen(c))));
                    }
                    // TODO : VERIFY TOWER MOVED
                    'r' =>  {
                        mailbox[index] = Inside(Some(Piece::Rook(Board::get_color_fen(c), false)));
                    }
                    'b' =>  {
                        mailbox[index] = Inside(Some(Piece::Bishop(Board::get_color_fen(c))));
                    }
                    'p' =>  {
                        mailbox[index] = Inside(Some(Piece::Pawn(Board::get_color_fen(c))));
                    }
                    'n' =>  {
                        mailbox[index] = Inside(Some(Piece::Knight(Board::get_color_fen(c))));
                    }
                    '1'..='8' => {
                        let empty_size = (c.to_digit(10).unwrap_or(1) - 1) as usize;
                        for i in 0..=empty_size {
                            mailbox[index + i] = Inside(None)
                        }
                        index += empty_size as usize;
                    }
                    '/' => {
                        if (index - offset) % 8 != 0 {
                            return Err(InvalidBoardErr{err: String::from(format!("Invalid return at index {}", i))})
                        }
                        index += 1;
                        offset += 2;
                    }
                    _ => return Err(InvalidBoardErr{err: String::from(format!("Could not identify the character {} at index {}", c, i))})
                }
                None => return Err(InvalidBoardErr{err: String::from(format!("Could not identify the character {} at index {}", c, i))})
            };
            index += 1;
        }
    
        Ok(Board{mailbox})
    }
    
    fn within_bounds(position : (usize, usize)) -> bool {
        position.0 < BOARD_X || position.1 < BOARD_Y
    }
}