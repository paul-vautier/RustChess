use super::piece::Piece;
use super::piece::Color;


pub const BOARD_SIZE : usize = 8;
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
            return Err("start not within bounds".to_string());
        }
    
        Ok(Position {x, y})
    }
}
impl Clone for Position {
    fn clone(&self) -> Self {
        Position{x: self.x, y: self.y}
    }
}

pub struct Board {
    pub tiles: [Option<Piece>; BOARD_SIZE * BOARD_SIZE],
}

impl Board {
    pub fn piece_at(&self, position : &Position) -> &Option<Piece> {
        &self.tiles[position.x * BOARD_SIZE + position.y]
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
    let mut tiles = [(); BOARD_SIZE * BOARD_SIZE].map(|_| None);
    let mut index : usize = 0;

    for (i, c) in notation.chars().into_iter().enumerate() {
        match c.to_lowercase().next() {
            Some(current) => match current {
                // TODO : VERIFY KING MOVED
                'k' =>  {
                    tiles[index] = Some(Piece::King(get_color_fen(c), false));
                }
                'q' =>  {
                    tiles[index] = Some(Piece::Queen(get_color_fen(c)));
                }
                // TODO : VERIFY TOWER MOVED
                'r' =>  {
                    tiles[index] = Some(Piece::Rook(get_color_fen(c), false));
                }
                'b' =>  {
                    tiles[index] = Some(Piece::Bishop(get_color_fen(c)));
                }
                'p' =>  {
                    tiles[index] = Some(Piece::Pawn(get_color_fen(c)));
                }
                'n' =>  {
                    tiles[index] = Some(Piece::Knight(get_color_fen(c)));
                }
                '1'..='8' => {
                    index += (c.to_digit(10).unwrap_or(1) - 1) as usize;
                }
                '/' => {
                    if index % 8 != 0 {
                        return Err(InvalidBoardErr{err: String::from(format!("Invalid return at index {}", i))})
                    }
                    index -= 1;
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
    position.0 > BOARD_SIZE || position.1 > BOARD_SIZE
}