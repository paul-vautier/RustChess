use std::{cell::RefCell, rc::Rc};

use tetra::{input::MouseButton, Context, State, TetraError};

use crate::{
    model::{
        board::{Board, Square, MAILBOX_INDICES},
        piece::{Color, Piece},
    },
    view::tetra_view::{MouseEventHandler, TetraView},
};

pub const SQUARE_SIZE: i32 = 150;
pub struct Controller {
    pub view: Rc<RefCell<TetraView>>,
    pub model: Board,
}

pub struct DisplayableBoard {
    pub board: [Option<Piece>; 64],
}
struct ControllerClickHandler {
    controller: Rc<RefCell<Controller>>,
}
struct ControllerReleaseHandler {
    controller: Rc<RefCell<Controller>>,
}
impl MouseEventHandler for ControllerClickHandler {
    fn handle(&mut self, button: MouseButton, x: f32, y: f32) {
        println!("clicked");

        if button != MouseButton::Left {
            return;
        }
        let x = x as usize / 8;
        let y = y as usize / 8;

        if x >= 8 || y >= 8 {
            return;
        }
        let controller = self.controller.borrow_mut();
        controller.view.borrow_mut().selected_piece = controller
            .model
            .piece_at_board_index(x + 8 * y)
            .map(|_| x + 8 * y);
    }
}
impl MouseEventHandler for ControllerReleaseHandler {
    fn handle(&mut self, button: MouseButton, x: f32, y: f32) {
        println!("released")
    }
}
impl Controller {
    pub fn new(view: &mut Rc<RefCell<TetraView>>, model: Board) -> Rc<RefCell<Self>> {
        let mut controller = Rc::new(RefCell::new(Controller {
            model,
            view: view.clone(),
        }));

        {
            let borrowed = controller.borrow_mut();
            borrowed.view.borrow_mut().mouse_clicked = Some(Box::new(ControllerClickHandler {
                controller: controller.clone(),
            }));

            borrowed.view.borrow_mut().mouse_released = Some(Box::new(ControllerReleaseHandler {
                controller: controller.clone(),
            }));
            borrowed.view.borrow_mut().board = borrowed.board_to_displayable();
        }

        controller
    }

    fn board_to_displayable(&self) -> DisplayableBoard {
        let mut board = [(); 64].map(|_| Some(Piece::Pawn(Color::WHITE)));
        for index in 0..64 {
            board[index] =
                if let Square::Inside(option) = self.model.mailbox[MAILBOX_INDICES[index]] {
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
}
