use std::collections::VecDeque;

use super::actions::ChessAction;

use super::piece::Color;
use super::piece::Piece;

pub const BOARD_X: usize = 8;
pub const BOARD_Y: usize = 8;
pub const BOARD_SIZE: usize = BOARD_X * BOARD_Y;
pub const MAX_PIECES_COUNT: usize = 32;
pub const BLACK_ROW: usize = 1;
pub const WHITE_ROW: usize = 7;

#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct BitBoard(u64);

pub enum CastleRights {
    All,
    KingSide,
    QueenSide,
    None,
}

pub struct InvalidBoardErr {
    pub err: String,
}

pub struct InvalidRemovalError {
    pub position: usize,
    pub reason: String,
}

pub struct InvalidMoveError {
    pub start: usize,
    pub end: usize,
    pub reason: String,
}
const PIECE_TYPE_COUNT: usize = 16;
const PLAYER_COUNT: usize = 2;
pub struct Board {
    pub double_pawn_move: Option<(usize, usize)>, // (ghost, pawn)
    pub history: VecDeque<ChessAction>,
    pub turn: u32,
    pub player_turn : bool,
    pub white_king: usize,
    pub black_king: usize,
    pub pieces: [Option<Piece>; BOARD_SIZE],
    pub bitboards: [[BitBoard ; PIECE_TYPE_COUNT]; PLAYER_COUNT],
    num_pieces: usize,
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
        if self.index >= BOARD_SIZE {
            return None;
        }
        self.index += 1;
        Some((self.index, self.board.pieces[self.index - 1]))
    }
}

pub struct PiecesIterator<'a> {
    pub index: usize,
    pub board: &'a Board,
}

impl Iterator for PiecesIterator<'_> {
    type Item = (usize, Option<Piece>);
    fn next(&mut self) -> Option<(usize, Option<Piece>)> {
        if self.index >= BOARD_SIZE {
            return None;
        }
        self.index += 1;
        Some((self.index, self.board.pieces[self.index - 1]))
    }
}
impl Board {
    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            index: 0,
            board: self,
        }
    }

    /**
     * Position on the actual board, from 0 to 64
     */
    pub fn piece_at(&self, position: usize) -> Option<&Piece> {
        self.pieces[position].as_ref()
    }

    pub fn move_piece(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<Option<Piece>, InvalidMoveError> {
        todo!()
    }

    pub fn remove_piece(&mut self, position: usize) -> Option<Piece> {
        todo!()
    }

    pub fn add_piece(&mut self, position: usize, piece: Piece) -> Result<(), InvalidRemovalError> {
        todo!()
    }

    pub fn get_king_by_color(&self, color: &Color) -> usize {
        match color {
            Color::WHITE => self.white_king,
            Color::BLACK => self.black_king,
        }
    }

    pub fn do_move(&mut self, mut action: ChessAction) {
        match action.execute(self) {
            Ok(_) => {
                self.double_pawn_move = None;

                match &mut self.pieces[action.target_square()] {
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
                        Piece::Pawn { color: _ } => self.double_pawn_move = action.double_forward(),
                        _ => (),
                    },
                    None => (),
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
                    match &mut self.pieces[action.start_square()] {
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
            if let Some(piece) = self.pieces[position] {
                return Some((position, &piece));
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
        Board {
            double_pawn_move: None,
            history: VecDeque::new(),
            turn: 1,
            white_king: 0,
            black_king: 0,
            pieces: [None; BOARD_SIZE],
            num_pieces: 0,
            color_to_play: Color::WHITE,
            white_castles_right: CastleRights::All,
            black_castles_right: CastleRights::All,
        }
    }

    pub fn set_piece(&mut self, position: usize, piece: Piece) {
        self.add_piece(position, piece);
    }

    pub fn from_fen(notation: String) -> Result<Self, InvalidBoardErr> {
        let mut white_king = None;
        let mut black_king = None;
        let mut board = Board::empty();
        let mut index = 0;
        for (i, c) in notation.chars().into_iter().enumerate() {
            if index < BOARD_SIZE {
                match c {
                    // TODO : VERIFY KING MOVED
                    'k' => {
                        if let None = black_king {
                            black_king = Some(index)
                        } else {
                            return Err(InvalidBoardErr {
                                err: "Multiple black kings where found on the board".to_string(),
                            });
                        }
                        board.set_piece(
                            index,
                            Piece::BlackKing {
                                first_move: u32::MAX,
                            },
                        )
                    }
                    'q' => board.set_piece(
                        index,
                        Piece::Queen {
                            color: Board::get_color_fen(c),
                        },
                    ),
                    // TODO : VERIFY TOWER MOVED
                    'r' => board.set_piece(
                        index,
                        Piece::Rook {
                            color: Board::get_color_fen(c),
                            first_move: u32::MAX,
                        },
                    ),
                    'b' => board.set_piece(
                        index,
                        Piece::Bishop {
                            color: Board::get_color_fen(c),
                        },
                    ),
                    'p' => board.set_piece(index, Piece::BlackPawn),
                    'n' => board.set_piece(index, Piece::BlackKnight),
                    '1'..='8' => {
                        let empty_size = (c.to_digit(10).unwrap_or(1) - 1) as usize;
                        for i in 0..=empty_size {
                            board.pieces[index + i] = None
                        }
                        index += empty_size as usize;
                    }
                    '/' => {
                        if index % 8 != 0 {
                            return Err(InvalidBoardErr {
                                err: String::from(format!("Invalid return at index {}", i)),
                            });
                        }
                        index += 1;
                    }
                    _ => {
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
