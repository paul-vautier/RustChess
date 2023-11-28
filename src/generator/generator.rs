use crate::model::{actions, board::Board};

pub fn count_actions(board: &mut Board, depth: u32, is_start: bool) -> u32 {
    if depth == 0 {
        return 1;
    }
    let mut moves = actions::generate_moves(board);
    let mut count = 0;
    let mut target = String::from("_");
    let mut start = String::from("_");
    while let Some(mv) = moves.pop() {
        if is_start {
            start = String::from_iter(vec![
                Board::get_file(mv.start_square()),
                Board::get_column(mv.start_square()),
            ]);
            target = String::from_iter(vec![
                Board::get_file(mv.target_square()),
                Board::get_column(mv.target_square()),
            ]);
        }
        board.do_move(mv);
        let actions = count_actions(board, depth - 1, false);
        if is_start {
            println!("{}{} : {}", start, target, actions)
        }
        count += actions;
        board.undo_last_move()
    }
    return count;
}
