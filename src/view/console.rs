use crate::model::board::{Square::*, TO_MAILBOX, TO_BOARD};
use crate::model::board::{BOARD_SIZE, BOARD_X};
use crate::model::{board::Board, piece::Color, piece::Piece};

use colored::ColoredString;
use colored::Colorize;
use fmt::Formatter;
use fmt::Result;
use std::fmt;

fn get_colored_string_piece(icon: String, color: &Color) -> ColoredString {
    match color {
        Color::WHITE => icon.white(),
        Color::BLACK => icon.black(),
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let icon = match self {
            Piece::Pawn { color } => get_colored_string_piece("♟".to_string(), color),
            Piece::Bishop { color } => get_colored_string_piece("♝".to_string(), color),
            Piece::Knight { color } => get_colored_string_piece("♞".to_string(), color),
            Piece::Rook { color, .. } => get_colored_string_piece("♜".to_string(), color),
            Piece::Queen { color } => get_colored_string_piece("♛".to_string(), color),
            Piece::King { color, .. } => get_colored_string_piece("♚".to_string(), color),
        };
        write!(f, "{}", icon)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut cells = [(); 64].map(|_| "⚠".red());
        let mut index: usize = 0;
        for (_, piece) in self.iter() {
            let colored_cell = match piece {
                Some(piece) => piece.to_string(),
                None => " ".to_string(),
            };

            cells[index] = if ((index % 2 + (index / 8 % 2)) % 2) == 0 {
                colored_cell.to_string().on_truecolor(120, 80, 0)
            } else {
                colored_cell.to_string().on_truecolor(153, 102, 0)
            };

            if let Some((ghost, _)) = self.double_pawn_move {
                if ghost == TO_MAILBOX[index] {
                    cells[index] = colored_cell.to_string().on_truecolor(100, 200, 100);
                }
            }
            index += 1;
        }

        for (indice, cell) in cells.iter().enumerate() {
            if indice % 8 == 0 {
                writeln!(f)?;
            }

            match write!(f, "{}", cell) {
                Ok(_) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }
}
