use crate::moves_generator::generate_legal_moves;
use crate::position::Position;
use crate::utils::Move;
use std::time::Instant;

pub fn perft(depth: i32, position: &Position) -> u64 {
    let mut number_of_move: u64 = 0;
    if depth == 0 {
        number_of_move = 1;
    } else {
        let turn = position.get_turn();
        let moves: Vec<Move> = generate_legal_moves(position, &turn);
        for m in &moves {
            let mut temp = position.clone();
            temp.make_move(&m, true);
            number_of_move += perft(depth - 1, &temp);
            // position.undo_last_move(history.pop());
        }
    }
    number_of_move
}

/*
pub fn perft(depth: i32, position: &Position) -> u64 {
    let mut number_of_move: u64 = 0;
    if depth == 0 {
        number_of_move = 1;
    } else {
        let turn = position.get_turn();
        let moves: Vec<Move> = generate_legal_moves(position, &turn);
        for m in &moves {
            history.push(UndoMove {
                source: m.source,
                destination: m.destination,
                piece_moved: position.get_piece_on_square(&m.source),
                piece_captured: if position.get_piece_on_square(&m.destination).piece_type
                    == PieceType::None
                {
                    None
                } else {
                    Option::from(position.get_piece_on_square(&m.destination))
                },
                move_type: m.move_type.clone(),
                en_passant: position.get_en_passant(),
            });
            position.make_move(&m, true);
            number_of_move += perft(depth - 1, position, history);
            position.undo_last_move(history.pop());
        }
    }
    number_of_move
}
*/


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