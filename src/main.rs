use std::{cell::RefCell, rc::Rc};

use controller::board_controller::{Controller, SQUARE_SIZE};
use tetra::{ContextBuilder, State, TetraError};
use view::tetra_view::TetraView;

use crate::model::board::Board;
mod controller;
mod model;
mod util;
mod view;
struct ViewWrapper(Rc<RefCell<TetraView>>);

impl State for ViewWrapper {
    fn update(&mut self, ctx: &mut tetra::Context) -> Result<(), TetraError> {
        self.0.borrow_mut().update(ctx)
    }

    fn draw(&mut self, ctx: &mut tetra::Context) -> Result<(), TetraError> {
        self.0.borrow_mut().draw(ctx)
    }

    fn event(&mut self, ctx: &mut tetra::Context, event: tetra::Event) -> Result<(), TetraError> {
        self.0.borrow_mut().event(ctx, event)
    }
}
fn main() -> tetra::Result {
    let board = {
        let this = Board::from_fen("8/pppppppp/8/8/8/8/PPPPPPPP/8".to_string());
        match this {
            Ok(t) => t,
            Err(e) => panic!("Invalid board {}", e.err),
        }
    };

    let mut context = ContextBuilder::new("Hello, world!", 8 * SQUARE_SIZE, 8 * SQUARE_SIZE)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?;

    let mut view = Rc::new(RefCell::new(TetraView::new(&mut context)?));
    Controller::new(&mut view, board);
    context.run(|ctx| Ok(ViewWrapper(view)))
}
