use crate::model::{board::Board, actions};

pub fn count_actions(board: &mut Board, depth: usize) -> u32 {
    if depth == 0 {
        return 1;
    }
    let mut moves = actions::generate_moves(board);
    let mut count = 0;
    while !moves.is_empty(){
        board.do_move(moves.remove(0));
        count += count_actions(board, depth-1);
        board.undo_last_move()
    }
    return count;
}