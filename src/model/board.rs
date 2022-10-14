use std::collections::VecDeque;
use std::ops::Add;
use std::ops::Index;

use super::actions::ChessAction;

use super::piece::Color;
use super::piece::Piece;

pub const BOARD_X: usize = 10;
pub const BOARD_Y: usize = 12;
pub const BOARD_SIZE: usize = BOARD_X * BOARD_Y;
pub const MAX_PIECES_COUNT: usize = 32;
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
    pub start: usize,
    pub end: usize,
    pub reason: String,
}

pub struct InvalidRemovalError {
    pub position: usize,
    pub reason: String,
}

#[derive(Debug)]
pub enum Square {
    Inside(Option<Piece>),
    Outside,
}

pub enum CastleRights {
    All,
    KingSide,
    QueenSide,
    None,
}
pub struct Board {
    mailbox: [Square; BOARD_SIZE],
    pub double_pawn_move: Option<(usize, usize)>, // (ghost, pawn)
    pub history: VecDeque<Box<dyn ChessAction>>,
    pub turn: u32,
    pub white_king: usize,
    pub black_king: usize,
    pub pieces: [usize; MAX_PIECES_COUNT],
    pieces_map: [usize; BOARD_SIZE],
    num_pieces: usize,
    color_to_play: Color,
    black_castles_right: CastleRights,
    white_castles_right: CastleRights,
}

pub struct BoardIterator<'a> {
    pub index: usize,
    pub board: &'a Board,
}

impl Iterator for BoardIterator<'_> {
    type Item = (usize, Option<Piece>);
    fn next(&mut self) -> Option<(usize, Option<Piece>)> {
        if self.index >= 64 {
            return None;
        }
        let result = match self.board.mailbox[TO_MAILBOX[self.index]] {
            Square::Inside(option) => option,
            Square::Outside => panic!("Invalid board"),
        };
        self.index += 1;
        Some((TO_MAILBOX[self.index - 1], result))
    }
}

pub struct MailboxIterator<'a> {
    pub index: usize,
    pub board: &'a Board,
}

impl<'a> Iterator for MailboxIterator<'a> {
    type Item = (usize, &'a Square);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= BOARD_SIZE {
            return None;
        }
        let result = &self.board.mailbox[self.index];
        self.index += 1;
        Some((self.index - 1, result))
    }
}

pub struct PiecesIteraror<'a> {
    pub index: usize,
    pub board: &'a Board,
}

impl<'a> Iterator for PiecesIteraror<'a> {
    type Item = (usize, &'a Piece);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.board.num_pieces {
            return None;
        }
        let result = match &self.board.mailbox[self.board.pieces[self.index]] {
            Square::Inside(Some(piece)) => piece,
            sq => panic!(
                "Invalid board : no piece were found on piece iterator {:?}, {:?}, {}",
                sq, self.board.mailbox, self.index
            ),
        };
        self.index += 1;
        Some((self.board.pieces[self.index - 1], result))
    }
}

impl Board {
    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            index: 0,
            board: self,
        }
    }

    pub fn pieces_iter(&self) -> PiecesIteraror {
        PiecesIteraror {
            index: 0,
            board: self,
        }
    }

    pub fn mailbox_iter(&self) -> MailboxIterator {
        MailboxIterator {
            index: 0,
            board: self,
        }
    }
    pub fn is_inside(&self, position: usize) -> bool {
        TO_BOARD[position] != -1
    }
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

    pub fn move_piece(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<Option<Piece>, InvalidMoveError> {
        let current = match self.remove_piece(start) {
            Some(piece) => piece,
            None => {
                return Err(InvalidMoveError {
                    start,
                    end,
                    reason: "start is empty".to_string(),
                })
            }
        };

        let option = self.remove_piece(end);

        self.add_piece(end, current)
            .map_err(|removal| InvalidMoveError {
                start,
                end,
                reason: removal.reason,
            })?;

        Ok(option)
    }

    pub fn remove_piece(&mut self, position: usize) -> Option<Piece> {
        match &mut self.mailbox[position] {
            Square::Inside(ref mut option @ Some(_)) => {
                let index = self.pieces_map[position];
                self.pieces[index] = self.pieces[self.num_pieces - 1];
                self.pieces_map[self.pieces[index]] = index;
                self.num_pieces -= 1;
                option.take()
            }
            _ => None,
        }
    }

    pub fn add_piece(&mut self, position: usize, piece: Piece) -> Result<(), InvalidRemovalError> {
        match &mut self.mailbox[position] {
            Square::Inside(option) => {
                if option.is_some() {
                    return Err(InvalidRemovalError {
                        position,
                        reason: "cannot add a piece to a non empty square".to_string(),
                    });
                }
                self.pieces[self.num_pieces] = position;
                self.pieces_map[position] = self.num_pieces;
                self.num_pieces += 1;
                *option = Some(piece);
                Ok(())
            }
            Square::Outside => Err(InvalidRemovalError {
                position,
                reason: "cannot add a piece outside the board".to_string(),
            }),
        }
    }

    pub fn get_king_by_color(&self, color: &Color) -> usize {
        match color {
            Color::WHITE => self.white_king,
            Color::BLACK => self.black_king,
        }
    }

    pub fn do_move(&mut self, mut action: Box<dyn ChessAction>) {
        match action.execute(self) {
            Ok(_) => {
                self.double_pawn_move = None;

                match &mut self.mailbox[action.target_square()] {
                    Square::Inside(ref mut option) => match option.as_mut() {
                        Some(piece) => match piece {
                            Piece::Rook {
                                color: _,
                                first_move,
                            } => {
                                if *first_move >= self.turn - 1 {
                                    *first_move = self.turn;
                                }
                            }
                            Piece::King { color, first_move } => {
                                if *first_move >= self.turn - 1 {
                                    *first_move = self.turn;
                                }

                                match color {
                                    Color::WHITE => self.white_king = action.target_square(),
                                    Color::BLACK => self.black_king = action.target_square(),
                                }
                            }
                            Piece::Pawn { color: _ } => {
                                self.double_pawn_move = action.double_forward()
                            }
                            _ => (),
                        },
                        None => (),
                    },
                    Square::Outside => (),
                }
                self.history.push_back(action);
                self.turn += 1;
                self.color_to_play = self.color_to_play.next();
            }
            Err(err) => println!("do : {}, action: {:?}, \n{}", err.reason, action, self),
        }
    }

    pub fn undo_last_move(&mut self) {
        match self.history.pop_back() {
            Some(mut action) => match action.undo(self) {
                Ok(_) => {
                    match &mut self.mailbox[action.start_square()] {
                        Square::Inside(ref mut option) => match option.as_mut() {
                            Some(piece) => match piece {
                                Piece::Rook {
                                    color: _,
                                    first_move,
                                } => {
                                    if *first_move >= self.turn - 1 {
                                        *first_move = u32::MAX;
                                    }
                                }
                                Piece::King { color, first_move } => {
                                    if *first_move >= self.turn - 1 {
                                        *first_move = u32::MAX;
                                    }

                                    match color {
                                        Color::WHITE => self.white_king = action.start_square(),
                                        Color::BLACK => self.black_king = action.start_square(),
                                    }
                                }
                                _ => (),
                            },
                            None => (),
                        },
                        Square::Outside => (),
                    };
                    self.color_to_play = self.color_to_play.next();
                    self.turn -= 1;
                    if let Some(action) = self.history.back() {
                        self.double_pawn_move = action.double_forward();
                    } else {
                        self.double_pawn_move = None;
                    }
                }
                Err(err) => println!("undo : {}, action : {:?}, \n{}", err.reason, action, self),
            },
            None => (),
        };
    }

    pub fn ray(&self, position: usize, direction: i32) -> Option<(usize, &Piece)> {
        let mut position = (position as i32 + direction) as usize;
        loop {
            match self.mailbox[position] {
                Square::Inside(ref option) => {
                    if let Some(piece) = option {
                        return Some((position, piece));
                    }
                }
                Square::Outside => return None,
            }
            position = (position as i32 + direction) as usize;
        }
    }

    pub fn is_on_promote_flag(color: &Color, index: usize) -> bool {
        match color {
            Color::WHITE => index / BOARD_X == BLACK_ROW,
            Color::BLACK => index / BOARD_X == WHITE_ROW,
        }
    }

    pub fn color_turn(&self) -> &Color {
        &self.color_to_play
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
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => 'x',
        }
    }

    pub fn get_column(index: usize) -> char {
        (10 - index / BOARD_X).to_string().chars().nth(0).unwrap()
    }

    pub fn empty() -> Self {
        use Square::*;

        let mailbox = [(); BOARD_SIZE].map(|_| Outside);
        Board {
            mailbox,
            double_pawn_move: None,
            history: VecDeque::new(),
            turn: 1,
            white_king: 0,
            black_king: 0,
            pieces: [(); MAX_PIECES_COUNT].map(|_| 0),
            pieces_map: [(); BOARD_SIZE].map(|_| 0),
            num_pieces: 0,
            color_to_play: Color::WHITE,
        }
    }

    pub fn set_piece_inside(&mut self, position: usize, piece: Piece) {
        use Square::*;
        self.mailbox[position] = Inside(None);
        self.add_piece(position, piece);
    }

    pub fn from_fen(notation: String) -> Result<Self, InvalidBoardErr> {
        let mut offset: usize = 2 * BOARD_X + 1;
        let mut index: usize = offset;
        let mut white_king = None;
        let mut black_king = None;
        let mut board = Board::empty();
        use Square::*;
        for (i, c) in notation.chars().into_iter().enumerate() {
            if index < BOARD_SIZE {
                match c.to_lowercase().next() {
                    Some(current) => match current {
                        // TODO : VERIFY KING MOVED
                        'k' => {
                            let color = Board::get_color_fen(c);

                            match color {
                                Color::WHITE => {
                                    if let None = white_king {
                                        white_king = Some(index)
                                    } else {
                                        return Err(InvalidBoardErr {
                                            err: "Multiple black kings where found on the board"
                                                .to_string(),
                                        });
                                    }
                                }
                                Color::BLACK => {
                                    if let None = black_king {
                                        black_king = Some(index)
                                    } else {
                                        return Err(InvalidBoardErr {
                                            err: "Multiple black kings where found on the board"
                                                .to_string(),
                                        });
                                    }
                                }
                            }
                            board.set_piece_inside(
                                index,
                                Piece::King {
                                    color: Board::get_color_fen(c),
                                    first_move: u32::MAX,
                                },
                            )
                        }
                        'q' => board.set_piece_inside(
                            index,
                            Piece::Queen {
                                color: Board::get_color_fen(c),
                            },
                        ),
                        // TODO : VERIFY TOWER MOVED
                        'r' => board.set_piece_inside(
                            index,
                            Piece::Rook {
                                color: Board::get_color_fen(c),
                                first_move: u32::MAX,
                            },
                        ),
                        'b' => board.set_piece_inside(
                            index,
                            Piece::Bishop {
                                color: Board::get_color_fen(c),
                            },
                        ),
                        'p' => board.set_piece_inside(
                            index,
                            Piece::Pawn {
                                color: Board::get_color_fen(c),
                            },
                        ),
                        'n' => board.set_piece_inside(
                            index,
                            Piece::Knight {
                                color: Board::get_color_fen(c),
                            },
                        ),
                        '1'..='8' => {
                            let empty_size = (c.to_digit(10).unwrap_or(1) - 1) as usize;
                            for i in 0..=empty_size {
                                board.mailbox[index + i] = Inside(None)
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
            }
            index += 1;
        }
        board.double_pawn_move = None;
        board.white_king = white_king.unwrap();
        board.black_king = black_king.unwrap();
        Ok(board)
    }
}
