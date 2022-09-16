use super::piece::Piece;


pub const BOARD_SIZE : usize = 8;

pub struct Board {
    tiles: [Option<Piece>; BOARD_SIZE * BOARD_SIZE],
}

pub struct Position {
    x : usize, 
    y : usize,
}

impl Position {
    fn new(x : usize, y : usize) -> Result<Position, String> {
        if !within_bounds((x, y)) {
            return Err("start not within bounds".to_string());
        }

        Ok(Position {x, y})
    }
}

impl Board {
    fn piece_at(&self, position : Position) -> Option<Piece> {
        self.tiles[position.x * BOARD_SIZE + position.y]
    }
}


fn within_bounds(position : (usize, usize)) -> bool {
    position.0 < 0 || position.0 > BOARD_SIZE || position.1 < 0 || position.1 > BOARD_SIZE
}