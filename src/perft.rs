use crate::history::History;
use crate::moves_generator::generate_pseudo_legal_moves;
use crate::position::Position;

pub fn perft(depth: i32, position: &Position) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut history: History = History::new();

    let mut number_of_move: u64 = 0;
    let turn = position.get_turn();
    let moves = generate_pseudo_legal_moves(position);
    for mov in &moves {
        let mut temp_position = position.clone();
        temp_position.make_move(&mov, &mut history);
        if !temp_position.is_check(&turn) {
            number_of_move += perft(depth - 1, &temp_position);
        }
    }

    number_of_move
}
