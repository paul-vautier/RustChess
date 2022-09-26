use std::collections::VecDeque;
use tetra::math::num_integer::Average;


use super::actions::ChessAction;
use super::piece::Color;
use super::piece::Piece;

pub const BOARD_X: usize = 10;
pub const BOARD_Y: usize = 12;
pub const BOARD_SIZE: usize = BOARD_X * BOARD_Y;
pub const BLACK_ROW: usize = 2;
pub const WHITE_ROW: usize = 9;

pub const TO_MAILBOX: [usize; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 63, 64, 65, 66, 67, 68, 71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88, 91, 92, 93, 94, 95, 96, 97, 98,
];
pub const TO_BOARD: [i32; 120] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3,
    4, 5, 6, 7, -1, -1, 8, 9, 10, 11, 12, 13, 14, 15, -1, -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
    -1, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, 32, 33, 34, 35, 36, 37, 38, 39, -1, -1, 40, 41, 42,
    43, 44, 45, 46, 47, -1, -1, 48, 49, 50, 51, 52, 53, 54, 55, -1, -1, 56, 57, 58, 59, 60, 61, 62,
    63, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];
pub struct InvalidBoardErr {
    pub err: String,
}

pub struct InvalidMoveError {
    pub start : usize,
    pub end : usize,
    pub reason : String,
}

pub struct InvalidRemovalError {
    pub position: usize,
    pub reason : String,
}



pub enum Square {
    Inside(Option<Piece>),
    Outside,
}

pub struct Board {
    pub mailbox: [Square; BOARD_SIZE],
    pub en_passant_position : Option<(usize,usize)>, // (ghost, pawn)
    pub history : VecDeque<Box<dyn ChessAction>>, 
    pub turn : u32,
}

impl Board {
    /**
     * Position on the actual board, from 0 to 64
     */
    pub fn piece_at_board_index(&self, position: usize) -> &Option<Piece> {
        match &self.mailbox[TO_MAILBOX[position] as usize] {
            Square::Inside(option) => option,
            Square::Outside => &None,
        }
    }

    /**
     * Position on the actual board, from 0 to 120
     */
    pub fn piece_at_mailbox_index(&self, position: usize) -> &Square {
        &self.mailbox[position]
    }

    pub fn move_piece(&mut self, start: usize, end: usize) -> Result<Option<Piece>, InvalidMoveError> {
        let current = match self.remove_piece(start){
            Some(piece) => piece,
            None => return Err(InvalidMoveError{start, end, reason : "start is empty".to_string()}),
        };

        let mut option = self.remove_piece(end);
        self.remove_piece(start); 

        self.add_piece(end, current).map_err(|removal| InvalidMoveError{start, end, reason: removal.reason})?;

        Ok(option)
    }

    pub fn remove_piece(&mut self, position : usize) -> Option<Piece>{
        match &mut self.mailbox[position] {
            Square::Inside(option) => option.take(),
            Square::Outside => None,
        }
    }    
    
    pub fn add_piece(&mut self, position : usize, piece: Piece) -> Result<(), InvalidRemovalError>{
        match &mut self.mailbox[position] {
            Square::Inside(option) => {
                if option.is_some() {
                    return Err(InvalidRemovalError{position, reason: "cannot add a piece to a non empty square".to_string()})
                }
                *option = Some(piece);
                Ok(())
            },
            Square::Outside => Err(InvalidRemovalError{position, reason: "cannot add a piece outside the board".to_string()}),
        }
    }

    pub fn do_move(&mut self, mut action : Box<dyn ChessAction>) {
        if let Ok(()) = action.execute(self) {
            self.en_passant_position = None;
            
            self.turn+=1;
            match &mut self.mailbox[action.target_square()] {
                Square::Inside(ref mut option) => match option.as_mut() {
                    Some(piece) => match piece {
                        Piece::Rook { color: _, first_move } |  Piece::King { color: _, first_move } => 
                        {
                            println!("first move {}", first_move);
                            if *first_move > self.turn {
                                *first_move = self.turn;
                            }
                            println!("first move {}", first_move);

                        }
                        Piece::Pawn{color: _} => {
                            if action.target_square().abs_diff(action.start_square()) == 2 * BOARD_X {
                                self.en_passant_position = Some((action.start_square().average_floor(&action.target_square()), action.target_square()));
                            }                
                        }
                        _ => (),
                    },
                    None => (),
                },
                Square::Outside => (),
            }
            self.history.push_back(action)
        }
    }

    pub fn undo_last_move(&mut self) {
        match self.history.pop_back() {
            Some(mut action) => {
                if let Ok(()) = action.undo(self) {
                    self.turn-=1;
                    match &mut self.mailbox[action.start_square()] {
                        Square::Inside(ref mut option) => match option.as_mut() {
                            Some(piece) => match piece {
                                Piece::Rook { color: _, first_move } |  Piece::King { color: _, first_move } => 
                                {
                                    println!("first move {}", first_move);
                                    if *first_move > self.turn {
                                        *first_move = u32::MAX;
                                    }
                                    println!("first move {}", first_move);
                                },
                                Piece::Pawn{color: _} => {
                                    if action.target_square().abs_diff(action.start_square()) == 2 * BOARD_X {
                                        self.en_passant_position = Some((action.start_square().average_floor(&action.target_square()), action.target_square()));
                                    }                
                                }
                                _ => (),
                            }
                            None => (),
                        },
                        Square::Outside => (),
                    };
                }
            },
            None => (),
        };
    }

    /**
     * Position on the actual board, from 0 to 120
     */
    pub fn piece_at_mailbox_index_as_mut(&mut self, position: usize) -> &mut Square {
        &mut self.mailbox[position]
    }

    pub fn promote_flag(color: &Color) -> usize {
        match color {
            Color::WHITE => BLACK_ROW,
            Color::BLACK => WHITE_ROW,
        }
    }

    pub fn is_on_promote_flag(color: &Color, index: usize) -> bool {
        match color {
            Color::WHITE => index / BOARD_X == BLACK_ROW,
            Color::BLACK => index / BOARD_X == WHITE_ROW,
        }
    }

    pub fn is_on_pawn_flag(color: &Color, index: usize) -> bool {
        match color {
            Color::WHITE => index / BOARD_X == WHITE_ROW - 1,
            Color::BLACK => index / BOARD_X == BLACK_ROW + 1,
        }
    }

    pub fn get_color_fen(c: char) -> Color {
        if c.is_lowercase() {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    pub fn get_file(index: usize) -> char {
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
    pub fn from_fen(notation: String) -> Result<Board, InvalidBoardErr> {
        let mut mailbox = [(); BOARD_SIZE].map(|_| Outside);
        let mut offset: usize = 2 * BOARD_X + 1;
        let mut index: usize = offset;

        use Square::*;
        for (i, c) in notation.chars().into_iter().enumerate() {
            match c.to_lowercase().next() {
                Some(current) => match current {
                    // TODO : VERIFY KING MOVED
                    'k' => {
                        mailbox[index] = Inside(Some(Piece::King {
                            color: Board::get_color_fen(c),
                            first_move: u32::MAX,
                        }));
                    }
                    'q' => {
                        mailbox[index] = Inside(Some(Piece::Queen {
                            color: Board::get_color_fen(c),
                        }));
                    }
                    // TODO : VERIFY TOWER MOVED
                    'r' => {
                        mailbox[index] = Inside(Some(Piece::Rook {
                            color: Board::get_color_fen(c),
                            first_move: u32::MAX,
                        }));
                    }
                    'b' => {
                        mailbox[index] = Inside(Some(Piece::Bishop {
                            color: Board::get_color_fen(c),
                        }));
                    }
                    'p' => {
                        mailbox[index] = Inside(Some(Piece::Pawn {
                            color: Board::get_color_fen(c),
                        }));
                    }
                    'n' => {
                        mailbox[index] = Inside(Some(Piece::Knight {
                            color: Board::get_color_fen(c),
                        }));
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
                            return Err(InvalidBoardErr {
                                err: String::from(format!("Invalid return at index {}", i)),
                            });
                        }
                        index += 1;
                        offset += 2;
                    }
                    _ => {
                        return Err(InvalidBoardErr {
                            err: String::from(format!(
                                "Could not identify the character {} at index {}",
                                c, i
                            )),
                        })
                    }
                },
                None => {
                    return Err(InvalidBoardErr {
                        err: String::from(format!(
                            "Could not identify the character {} at index {}",
                            c, i
                        )),
                    })
                }
            };
            index += 1;
        }

        Ok(Board { mailbox , en_passant_position: None, history: VecDeque::new(), turn: 0})
    }
}
