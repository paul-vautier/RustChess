use crate::model::actions::{ChessAction, MovesList};
use crate::model::board::{BOARD_SIZE, BOARD_X};
use crate::model::{piece::Piece, board::Board, piece::Color};
use crate::model::board::Square::*;

use std::fmt;
use colored::ColoredString;
use colored::Colorize;
use fmt::Formatter;
use fmt::Result;


fn get_colored_string_piece(icon : String, color: &Color) -> ColoredString{
    match color {
        Color::WHITE => icon.white(),
        Color::BLACK => icon.black(),
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let icon = match self {
            Piece::Pawn(c) => get_colored_string_piece("♟".to_string(), c),
            Piece::Bishop(c) => get_colored_string_piece("♞".to_string(), c),
            Piece::Knight(c) => get_colored_string_piece("♝".to_string(), c),
            Piece::Rook(c, _) => get_colored_string_piece("♜".to_string(), c),
            Piece::Queen(c) =>  get_colored_string_piece("♛".to_string(), c),
            Piece::King(c, _) => get_colored_string_piece("♚".to_string(), c)
        };
        write!(f, "{}", icon)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut cells = [(); BOARD_SIZE].map(|_| "⚠".red());
         for (indice, square) in self.tiles.iter().enumerate() {
            let colored_cell = match square { 
                Inside(option) => match option {
                    Some(piece) => piece.to_string(),
                    None => " ".to_string()
                },
                Outside => " ".to_string()
            };
            
            cells[indice] = if ((indice % 2 + (indice/8 % 2)) % 2) == 0 {
                colored_cell.to_string().on_truecolor(120, 80, 0)
            } else { 
                colored_cell.to_string().on_truecolor(153, 102, 0)
            };
        };

        for (indice, cell) in cells.iter().enumerate() {
            if indice % BOARD_X == 0 {
                writeln!(f)?;
            }

            match write!(f, "{}", cell) {
                Ok(_) => {},
                Err(error) => return Err(error)
            }
        }
        Ok(())
    }
}