use crate::{evaluation, moves_generator};
use crate::position::Position;
use crate::utils::{Move, PieceColor};
use crate::zobrist_hash::ZobristHash;

pub fn best_move(position: &Position, zobrist_hash:&mut ZobristHash) -> Move {
    let moves = moves_generator::generate_moves(position);
    let mut final_move: Move = moves[0].clone();
    let mut score: i32 = match position.get_turn() {
        PieceColor::None => 0,
        PieceColor::White => -i32::MAX,
        PieceColor::Black => i32::MAX,
    };
    let depth = 3 * 2;

    for m in moves {
        let mut temp_position = position.clone();
        match temp_position.get_turn() {
            PieceColor::None => {}
            PieceColor::White => {
                temp_position.make_move(&m);
                let result = alpha_beta(
                    &temp_position,
                    &depth,
                    &mut -i32::MAX,
                    &mut i32::MAX,
                    zobrist_hash
                );
                if score <= result {
                    score = result;
                    final_move = m;
                    println!(
                        "best-move {}{}{}{} => {}",
                        final_move.source.file,
                        final_move.source.rank,
                        final_move.destination.file,
                        final_move.destination.rank,
                        score
                    );
                }
            }
            PieceColor::Black => {
                temp_position.make_move(&m);
                let result = alpha_beta(
                    &temp_position,
                    &depth,
                    &mut -i32::MAX,
                    &mut i32::MAX,
                    zobrist_hash
                );
                if score >= result {
                    score = result;
                    final_move = m;
                    println!(
                        "best-move {}{}{}{} => {}",
                        final_move.source.file,
                        final_move.source.rank,
                        final_move.destination.file,
                        final_move.destination.rank,
                        score
                    );
                }
            }
        }
    }

    final_move
}

fn alpha_beta(position: &Position, depth: &i32, alpha: &mut i32, beta: &mut i32, zobrist_hash:&mut ZobristHash) -> i32 {
    if *depth == 0 {
        let score = evaluation::evaluate(position);
        // zobrist_hash.insert_position(position, &score); // Add the final score of the position (on the leaf)
        return score;
    }

    // If the position in already seen, no need to make any further analysis
    // if zobrist_hash.contains_position(position) {
    //     return zobrist_hash.get_position_evaluation(position);
    // }

    // To maximize the efficiency of the alpha-beta purring,
    // the array of all possibles moves is store ascending (or descending)
    // in order to hit the best score faster
    let moves = moves_generator::generate_moves(position);
    let mut temp_positions: Vec<Position> = Vec::new();
    for m in moves {
        let mut pos = position.clone();
        pos.make_move(&m);
        temp_positions.push(pos);
    }

    match position.get_turn() {
        PieceColor::None => 0,
        PieceColor::White => {
            let mut score = -i32::MAX;
            // temp_positions.sort_by(|a, b| evaluate(a).partial_cmp(&evaluate(b)).unwrap());
            for pos in temp_positions {
                score = score.max(alpha_beta(&pos, &(depth - 1), alpha, beta, zobrist_hash));
                if score >= *beta {
                    break;
                }
                *alpha = *alpha.max(&mut score);
            }
            // zobrist_hash.insert_position(position, &score); // Add the final score of the position
            score
        }
        PieceColor::Black => {
            let mut score = i32::MAX;
            // temp_positions.sort_by(|a, b| evaluate(b).partial_cmp(&evaluate(a)).unwrap());
            for pos in temp_positions {
                score = score.min(alpha_beta(&pos, &(depth - 1), alpha, beta, zobrist_hash));
                if score <= *alpha {
                    break;
                }
                *beta = *beta.min(&mut score);
            }
            // zobrist_hash.insert_position(position, &score); // Add the final score of the position
            score
        }
    }
}

// fn minmax(position: &Position, depth: &i32) -> f64 {
//     if *depth == 0 {
//         return evaluate(position);
//     }
//
//
//     let moves = moves_generator::generate_moves(position);
//     let mut current_score: f64 = match position.get_turn() {
//         PieceColor::None => 0.0,
//         PieceColor::White => f64::NEG_INFINITY,
//         PieceColor::Black => f64::INFINITY,
//     };
//
//     for m in moves {
//         let mut temp_position = position.clone();
//         match temp_position.get_turn() {
//             PieceColor::None => {}
//             PieceColor::White => {
//                 temp_position.make_move(&m);
//                 current_score = current_score.max(minmax(&temp_position, &(depth - 1)));
//             }
//             PieceColor::Black => {
//                 temp_position.make_move(&m);
//                 current_score = current_score.min(minmax(&temp_position, &(depth - 1)));
//             }
//         }
//     }
//
//     current_score
// }


