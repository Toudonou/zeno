use crate::moves::Move;
use crate::moves_generator::generate_pseudo_legal_moves;
use crate::position::Position;
use crate::utils::random_u64;

pub fn best_move(position: &Position) -> Option<Move> {
    let moves = generate_pseudo_legal_moves(position);
    let mut legal_moves: Vec<Move> = Vec::with_capacity(moves.len());

    loop {
        if !moves.is_empty() {
            for mov in moves {
                let mut temp_position = position.clone();
                temp_position.make_move(&mov);
                if !temp_position.is_check(&position.get_turn()) {
                    legal_moves.push(mov);
                }
            }
        }
        break;
    }

    Some(legal_moves[random_u64() as usize % legal_moves.len()])
}