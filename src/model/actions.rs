use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::board::{Board, BOARD_SIZE, InvalidMoveError};
use super::piece::Color;

pub struct ChessAction{}

impl ChessAction {
    pub fn to_algebraic_notation(&self, board : &Board) -> String {
    }

    fn execute(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        todo!()
    }
    fn undo(&mut self, board: &mut Board) -> Result<(), InvalidMoveError> {
        todo!()
    }
    fn as_promotion(&self, color: &Color) -> Result<MovesList, String> {
        todo!()
    }
    fn to_algebraic_notation(&self, board: &Board) -> String {
        todo!()
    }
    fn target_square(&self) -> usize {
        todo!()
    }
    fn start_square(&self) -> usize {
        todo!()
    }
    fn double_forward(&self) -> Option<(usize, usize)> {
        todo!()
    }
}
pub struct MovesList(pub Vec<ChessAction>);

pub struct BoardPins(pub [Option<PinState>; BOARD_SIZE]);

pub enum PinState {
    Pinned(i32),
    Locked,
}
pub struct BoardAttackData {
    pub white_king: usize,
    pub black_king: usize,
    pins: BoardPins,
    resolve_check: Vec<usize>,
}

impl MovesList {
    pub fn to_algebraic_notation(&self, board: &Board) -> String {
        let mut result = String::from("");
        for (index, current) in self.0.iter().enumerate() {
            result += format!(
                "{0} : {1}\n",
                index.to_string(),
                current.to_algebraic_notation(board)
            )
            .as_str()
        }
        String::from(result)
    }
}

impl Deref for MovesList {
    type Target = Vec<ChessAction>;
    fn deref(&self) -> &Vec<ChessAction> {
        &self.0
    }
}

impl DerefMut for MovesList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for BoardPins {
    type Target = HashMap<usize, PinState>;
    fn deref(&self) -> &HashMap<usize, PinState> {
        &self.0
    }
}

impl DerefMut for BoardPins {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BoardPins {
    pub fn can_move_in_direction(&self, pos: usize, direction: i32) -> bool {
        if let Some(state) = self.get(&pos) {
            match state {
                PinState::Pinned(dir) => direction == *dir || -direction == *dir,
                PinState::Locked => return false,
            }
        } else {
            true
        }
    }
}

pub fn generate_moves(board: &Board) -> MovesList {
    todo!()
}
