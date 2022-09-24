use std::{option, path::Path};

use tetra::{
    graphics::{self, DrawParams, Texture},
    input::{self, Key, MouseButton},
    math::Vec2,
    Context, State, TetraError,
};

use crate::{
    controller::board_controller::{DisplayableBoard, SQUARE_SIZE},
    model::piece::{self, Piece},
};
const PAWN_OFFSET: f32 = 22.0;
const QUEEN_OFFSET: f32 = 2.0;
const KNIGHT_OFFSET: f32 = 17.0;
const BISHOP_OFFSET: f32 = 11.0;
const ROOK_OFFSET: f32 = 17.0;
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
pub trait MouseEventHandler {
    fn handle(&mut self, button: MouseButton, x: f32, y: f32);
}
pub struct TetraView {
    pub key_down_handler: Option<fn(Key)>,
    pub mouse_clicked: Option<Box<dyn MouseEventHandler>>,
    pub mouse_released: Option<Box<dyn MouseEventHandler>>,

    light_square: Texture,
    dark_square: Texture,
    white_assets: PiecesAsset,
    black_assets: PiecesAsset,
    pub selected_piece: Option<usize>,
    pub board: DisplayableBoard,
}

impl TetraView {
    pub fn new(ctx: &mut Context) -> tetra::Result<TetraView> {
        Ok(TetraView {
            key_down_handler: None,
            mouse_clicked: None,
            mouse_released: None,
            white_assets: PiecesAsset::load(ctx, Path::new("./resources/white"))?,
            black_assets: PiecesAsset::load(ctx, Path::new("./resources/black"))?,
            light_square: Texture::new(ctx, "./resources/square/gray_light.png")?,
            dark_square: Texture::new(ctx, "./resources/square/gray_dark.png")?,
            board: DisplayableBoard {
                board: [(); 64].map(|_| None),
            },
            selected_piece: None,
        })
    }

    fn asset_from_color(&self, color: &piece::Color) -> &PiecesAsset {
        match color {
            piece::Color::WHITE => &self.white_assets,
            piece::Color::BLACK => &self.black_assets,
        }
    }

    fn x_position(i: usize) -> f32 {
        (SQUARE_SIZE * (i as i32 % 8)) as f32
    }
    fn y_position(i: usize) -> f32 {
        (SQUARE_SIZE * (i as i32 / 8)) as f32
    }
}
impl State for TetraView {
    fn update(&mut self, ctx: &mut tetra::Context) -> Result<(), TetraError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut tetra::Context) -> Result<(), TetraError> {
        graphics::clear(ctx, graphics::Color::WHITE);
        for i in 0..self.board.board.len() {
            let square = if (i + i / 8) % 2 == 0 {
                &self.light_square
            } else {
                &self.dark_square
            };
            square.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        TetraView::x_position(i),
                        TetraView::y_position(i),
                    ))
                    .scale(Vec2::new(1.171875, 1.171875)),
            );
            match self.board.board[i] {
                Some(option) => {
                    let (texture, offset) = match option {
                        Piece::Pawn(c) => (&self.asset_from_color(&c).pawn, PAWN_OFFSET),
                        Piece::Bishop(c) => (&self.asset_from_color(&c).bishop, BISHOP_OFFSET),
                        Piece::Knight(c) => (&self.asset_from_color(&c).knight, KNIGHT_OFFSET),
                        Piece::Rook(c, _) => (&self.asset_from_color(&c).rook, ROOK_OFFSET),
                        Piece::Queen(c) => (&self.asset_from_color(&c).queen, QUEEN_OFFSET),
                        Piece::King(c, _) => (&self.asset_from_color(&c).king, KNIGHT_OFFSET),
                    };
                    if let Some(index) = self.selected_piece {
                        if index == i {
                            texture.draw(
                                ctx,
                                Vec2::new(
                                    offset - 75.0 + input::get_mouse_x(ctx),
                                    11.0 - 75.0 + input::get_mouse_y(ctx),
                                ),
                            );
                            continue;
                        }
                    }
                    texture.draw(
                        ctx,
                        Vec2::new(
                            offset + TetraView::x_position(i),
                            11.0 + TetraView::y_position(i),
                        ),
                    )
                }
                None => (),
            }
        }
        Ok(())
    }

    fn event(&mut self, ctx: &mut tetra::Context, event: tetra::Event) -> Result<(), TetraError> {
        match event {
            tetra::Event::KeyPressed { key } => {
                if let Some(event) = self.key_down_handler {
                    event(key);
                }
            }
            tetra::Event::MouseButtonPressed { button } => {
                if let Some(event) = &mut self.mouse_clicked {
                    event.handle(button, input::get_mouse_x(ctx), input::get_mouse_y(ctx));
                }
            }
            tetra::Event::MouseButtonReleased { button } => {
                if let Some(event) = &mut self.mouse_released {
                    event.handle(button, input::get_mouse_x(ctx), input::get_mouse_y(ctx));
                }
            }
            _ => (),
        }
        Ok(())
    }
}
