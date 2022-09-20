use super::piece::Piece;
use super::piece::Color;


pub const BOARD_X : usize = 10;
pub const BOARD_Y : usize = 12;
pub const BOARD_SIZE : usize = BOARD_X * BOARD_Y;
pub const BLACK_ROW : i32 = 0;
pub const WHITE_ROW : i32 = 8;

pub struct InvalidBoardErr {
    pub err: String
}

pub struct Position {
    pub x : usize, 
    pub y : usize,
}

impl Position {
    pub fn new(x : usize, y : usize) -> Result<Position, String> {
        if !within_bounds((x, y)) {
            return Err("position not within bounds".to_string());
        }
    
        Ok(Position {x, y})
    }
}
impl Clone for Position {
    fn clone(&self) -> Self {
        Position{x: self.x, y: self.y}
    }
}

pub enum Square {
    Inside(Option<Piece>),
    Outside
}

pub struct Board {
    pub tiles: [Square; BOARD_SIZE],
}

impl Board {
    pub fn piece_at(&self, position : &Position) -> &Square {
        &self.tiles[position.x + position.y * BOARD_X]
    }

    pub fn promote_flag(color : &Color) -> usize {
        match color {
            Color::WHITE => 0,
            Color::BLACK => 7,
        }
    }
}

pub fn get_color_fen(c : char) -> Color{
    if c.is_lowercase() {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

pub fn get_file(file_no : usize) -> char {
    match file_no {
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
pub fn add_usize(value : usize, offset : i32) -> usize {
    let val = if offset.is_negative() {
        match value.checked_sub(offset as usize) {
            Some(new) => new,
            None => panic!("Index out of bounds {} - {}", value, offset),
        }
    } else {
        match value.checked_add(offset as usize) {
            Some(new) => new,
            None => panic!("Index out of bounds {} + {}", value, offset),
        }
    };
    val
}
pub fn from_fen(notation : String) -> Result<Board,InvalidBoardErr>  {
    let mut tiles = [(); BOARD_SIZE].map(|_| Outside);
    let mut offset : usize = 2 * BOARD_X + 1;
    let mut index : usize =  offset;

    use Square::*;
    for (i, c) in notation.chars().into_iter().enumerate() {
        match c.to_lowercase().next() {
            Some(current) => match current {
                // TODO : VERIFY KING MOVED
                'k' =>  {
                    tiles[index] = Inside(Some(Piece::King(get_color_fen(c), false)));
                }
                'q' =>  {
                    tiles[index] = Inside(Some(Piece::Queen(get_color_fen(c))));
                }
                // TODO : VERIFY TOWER MOVED
                'r' =>  {
                    tiles[index] = Inside(Some(Piece::Rook(get_color_fen(c), false)));
                }
                'b' =>  {
                    tiles[index] = Inside(Some(Piece::Bishop(get_color_fen(c))));
                }
                'p' =>  {
                    tiles[index] = Inside(Some(Piece::Pawn(get_color_fen(c))));
                }
                'n' =>  {
                    tiles[index] = Inside(Some(Piece::Knight(get_color_fen(c))));
                }
                '1'..='8' => {
                    let empty_size = (c.to_digit(10).unwrap_or(1) - 1) as usize;
                    for i in 0..=empty_size {
                        tiles[index + i] = Inside(Some(Piece::Queen(get_color_fen(c))))
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

    Ok(Board{tiles})
}

fn within_bounds(position : (usize, usize)) -> bool {
    position.0 < BOARD_X || position.1 < BOARD_Y
}