use crate::moves_generator::{generate_legal_moves, generate_pseudo_legal_moves};
use crate::position::Position;

pub fn perft(depth: i32, position: &mut Position) -> u64 {
    let mut number_of_move: u64 = 0;
    if depth == 0 {
        number_of_move = 1;
    } else {
        let turn = position.get_turn();
        let moves = generate_legal_moves(position, &turn);
        for mov in &moves {
            match mov {
                None => break,
                Some(m) => {
                    position.make_move(&m, true);
                    number_of_move += perft(depth - 1, position);
                    position.undo_last_move();
                }
            }
        }
    }
    number_of_move
}

// pub fn pertf_divide(depth: i32, position: &Position) {
//     let moves: Vec<Move> = generate_legal_moves(position, &position.get_turn());
//     for m in moves {
//         let mut temp_position = position.clone();
//         temp_position.make_move(&m, true);
//         let start = Instant::now();
//         let number_of_move = perft(depth - 1, &temp_position);
//         let duration = start.elapsed();
//         println!(
//             "Move : {}{}{}{} => {} in {}s",
//             ('a' as u8 + (m.source % 8) as u8) as char,
//             1 + m.source / 8,
//             ('a' as u8 + (m.destination % 8) as u8) as char,
//             1 + m.destination / 8,
//             number_of_move,
//             duration.as_secs()
//         );
//     }
// }
