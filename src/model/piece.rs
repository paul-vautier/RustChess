#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
    WHITE,
    BLACK,
}

impl Color {
    pub fn next(&self) -> Self {
        match self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        }
    }
}

#[repr(u8)]
pub enum Piece {
    None = 0b0000,
    WhitePawn = 0b0001,
    WhiteKnight = 0b0010,
    WhiteBishop = 0b0011,
    WhiteRook = 0b0100,
    WhiteQueen = 0b0101,
    WhiteKing = 0b0110,
    BlackPawn = 0b1001,
    BlackKnight = 0b1010,
    BlackBishop = 0b1011,
    BlackRook = 0b1100,
    BlackQueen = 0b1101,
    BlackKing = 0b1110,
}

impl Piece {
    pub fn get_color(&self) -> Color {
        if *self as u8 & 0b1000 == 0 {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }
}
