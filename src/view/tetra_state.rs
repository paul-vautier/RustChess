use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use tetra::{
    graphics::{self, mesh::Mesh, DrawParams, Rectangle, Texture},
    input::{self, MouseButton, Key},
    math::Vec2,
    Context, State, TetraError,
};

use crate::model::{
    actions::{ChessAction},
    board::{Board, Square, TO_BOARD, TO_MAILBOX},
    piece::{self, Color, Piece},
};
const PAWN_OFFSET: f32 = 22.0;
const QUEEN_OFFSET: f32 = 2.0;
const KNIGHT_OFFSET: f32 = 17.0;
const BISHOP_OFFSET: f32 = 11.0;
const ROOK_OFFSET: f32 = 17.0;
pub const SQUARE_SIZE: f32 = 150.0;

struct PiecesAsset {
    king: Texture,
    rook: Texture,
    pawn: Texture,
    queen: Texture,
    bishop: Texture,
    knight: Texture,
}

impl PiecesAsset {
    pub fn load(ctx: &mut Context, folder: &Path) -> tetra::Result<PiecesAsset> {
        Ok(PiecesAsset {
            knight: Texture::new(ctx, folder.join("knight.png"))?,
            king: Texture::new(ctx, folder.join("king.png"))?,
            queen: Texture::new(ctx, folder.join("queen.png"))?,
            bishop: Texture::new(ctx, folder.join("bishop.png"))?,
            pawn: Texture::new(ctx, folder.join("pawn.png"))?,
            rook: Texture::new(ctx, folder.join("rook.png"))?,
        })
    }
}

pub struct DisplayableBoard {
    pub board: [Option<Piece>; 64],
}

pub struct TetraState {
    light_square: Texture,
    dark_square: Texture,
    white_assets: PiecesAsset,
    black_assets: PiecesAsset,
    pub valid_squares: Vec<usize>,
    pub selected_piece: Option<usize>,
    pub view: DisplayableBoard,
    pub board: Board,
}

impl TetraState {
    pub fn new(ctx: &mut Context, board: Board) -> tetra::Result<TetraState> {
        let mut state = TetraState {
            white_assets: PiecesAsset::load(ctx, Path::new("./resources/white"))?,
            black_assets: PiecesAsset::load(ctx, Path::new("./resources/black"))?,
            light_square: Texture::new(ctx, "./resources/square/gray_light.png")?,
            dark_square: Texture::new(ctx, "./resources/square/gray_dark.png")?,
            board,
            valid_squares: Vec::new(),
            selected_piece: None,
            view: DisplayableBoard {
                board: [(); 64].map(|_| None),
            },
        };
        state.view = state.board_to_displayable();
        Ok(state)
    }

    fn asset_from_color(&self, color: &piece::Color) -> &PiecesAsset {
        match color {
            piece::Color::WHITE => &self.white_assets,
            piece::Color::BLACK => &self.black_assets,
        }
    }

    fn x_position(i: usize) -> f32 {
        SQUARE_SIZE * (i % 8) as f32
    }
    fn y_position(i: usize) -> f32 {
        SQUARE_SIZE * (i / 8) as f32
    }

    fn handle_mouse_clicked(&mut self, button: MouseButton, x: f32, y: f32) {
        if button != MouseButton::Left {
            return;
        }
        let x = (x / SQUARE_SIZE) as usize;
        let y = (y / SQUARE_SIZE) as usize;

        if x >= 8 || y >= 8 {
            return;
        }
        let position = x + 8 * y;
        if let Some(piece) = self.board.piece_at_board_index(position) {
            self.selected_piece = Some(position);
            self.valid_squares = piece
                .valid_moves(TO_MAILBOX[position], piece.get_color(), &self.board)
                .iter()
                .map(Box::as_ref)
                .map(ChessAction::target_square)
                .map(|index| TO_BOARD[index] as usize)
                .collect();
        }
    }

    fn handle_mouse_released(&mut self, button: MouseButton, x: f32, y: f32) {
        if button != MouseButton::Left {
            return;
        }

        let x = (x / SQUARE_SIZE) as usize;
        let y = (y / SQUARE_SIZE) as usize;

        if x >= 8 || y >= 8 {
            return;
        }

        let position = x + 8 * y;
        if let Some(start) = self.selected_piece {

            if let Some(piece) = self.board.piece_at_board_index(start as usize) {
                let mut moves =
                    piece.valid_moves(TO_MAILBOX[start], piece.get_color(), &self.board);
                let mut selected : Vec<Box<dyn ChessAction>> = Vec::new();

                for i in (0..moves.len()).rev() {
                    if moves[i].start_square() == TO_MAILBOX[start]
                        && moves[i].target_square() == TO_MAILBOX[position] as usize {
                            selected.push(moves.remove(i))
                    }
                }

                if !selected.is_empty() {
                    self.board.do_move(selected.remove(0));
                }
            }
        }

        self.view = self.board_to_displayable();
        self.selected_piece = None;
        self.valid_squares = vec![];
    }


    fn handle_key_pressed(&mut self, key: Key) {
        if key == Key::Left {
            self.board.undo_last_move();
            self.view = self.board_to_displayable();
        }
    }
    fn board_to_displayable(&self) -> DisplayableBoard {
        let mut board = [(); 64].map(|_| {
            Some(Piece::Pawn {
                color: Color::WHITE,
            })
        });
        for index in 0..64 {
            board[index] = if let Square::Inside(option) = self.board.mailbox[TO_MAILBOX[index]] {
                match option {
                    Some(piece) => Some(piece.clone()),
                    None => None,
                }
            } else {
                None
            }
        }
        DisplayableBoard { board }
    }

    fn piece_to_texture(&self, piece: &Piece) -> (&Texture, f32) {
        match piece {
            Piece::Pawn { color } => (&self.asset_from_color(&color).pawn, PAWN_OFFSET),
            Piece::Bishop { color } => (&self.asset_from_color(&color).bishop, BISHOP_OFFSET),
            Piece::Knight { color } => (&self.asset_from_color(&color).knight, KNIGHT_OFFSET),
            Piece::Rook { color, .. } => (&self.asset_from_color(&color).rook, ROOK_OFFSET),
            Piece::Queen { color } => (&self.asset_from_color(&color).queen, QUEEN_OFFSET),
            Piece::King { color, .. } => (&self.asset_from_color(&color).king, KNIGHT_OFFSET),
        }
    }
}

impl State for TetraState {
    fn update(&mut self, ctx: &mut tetra::Context) -> Result<(), TetraError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut tetra::Context) -> Result<(), TetraError> {
        graphics::clear(ctx, graphics::Color::WHITE);
        for i in 0..self.view.board.len() {
            let square = if (i + i / 8) % 2 == 0 {
                &self.light_square
            } else {
                &self.dark_square
            };
            square.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        TetraState::x_position(i),
                        TetraState::y_position(i),
                    ))
                    .scale(Vec2::new(1.171875, 1.171875)),
            );
            match self.view.board[i] {
                Some(piece) => {
                    if self
                        .selected_piece
                        .map(|selected| selected == i)
                        .unwrap_or(false)
                    {
                        continue;
                    }
                    let (texture, offset) = self.piece_to_texture(&piece);
                    texture.draw(
                        ctx,
                        Vec2::new(
                            offset + TetraState::x_position(i),
                            11.0 + TetraState::y_position(i),
                        ),
                    )
                }
                None => (),
            }
        }
        for square in self.valid_squares.iter() {
            let rect = Rectangle::new(0.0, 0.0, SQUARE_SIZE, SQUARE_SIZE);
            Mesh::rectangle(ctx, graphics::mesh::ShapeStyle::Fill, rect)?.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        SQUARE_SIZE * (square % 8) as f32,
                        SQUARE_SIZE * (square / 8) as f32,
                    ))
                    .color(graphics::Color::rgba(0.0, 0.0, 0.0, 0.5)),
            )
        }
        if let Some(index) = self.selected_piece {
            match self.board.piece_at_board_index(index) {
                Some(piece) => {
                    let (texture, offset) = self.piece_to_texture(piece);
                    texture.draw(
                        ctx,
                        Vec2::new(
                            offset - 75.0 + input::get_mouse_x(ctx),
                            11.0 - 75.0 + input::get_mouse_y(ctx),
                        ),
                    );
                }
                None => (),
            };
        }
        Ok(())
    }

    fn event(&mut self, ctx: &mut tetra::Context, event: tetra::Event) -> Result<(), TetraError> {
        match event {
            tetra::Event::MouseButtonPressed { button } => {
                self.handle_mouse_clicked(button, input::get_mouse_x(ctx), input::get_mouse_y(ctx));
            }
            tetra::Event::MouseButtonReleased { button } => {
                self.handle_mouse_released(
                    button,
                    input::get_mouse_x(ctx),
                    input::get_mouse_y(ctx),
                );
            },
            tetra::Event::KeyPressed { key } => {
                self.handle_key_pressed(key)
            }
            _ => (),
        }
        Ok(())
    }
}
