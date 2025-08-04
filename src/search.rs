use crate::position::Position;
use crate::utils::{Move, PieceColor};
use crate::{evaluation, moves_generator};

pub fn best_move(position: &Position) -> Move {
    let moves = moves_generator::generate_legal_moves(position, &position.get_turn());
    let mut final_move = moves[0].clone();
    let mut score: f64 = match position.get_turn() {
        PieceColor::None => 0.0,
        PieceColor::White => f64::NEG_INFINITY,
        PieceColor::Black => f64::INFINITY,
    };
    let depth: u16 = 3 * 2;

    for m in moves {
        let mut temp_position = position.clone();
        match temp_position.get_turn() {
            PieceColor::None => {}
            PieceColor::White => {
                temp_position.make_move(&m, true);
                let result = alpha_beta(
                    &temp_position,
                    &depth,
                    &mut f64::NEG_INFINITY,
                    &mut f64::INFINITY,
                );
                if score <= result {
                    score = result;
                    final_move = m;
                    println!(
                        "best-move {}{}{}{} => {}",
                        ('a' as u8 + (final_move.source % 8) as u8) as char,
                        1 + final_move.source / 8,
                        ('a' as u8 + (final_move.destination % 8) as u8) as char,
                        1 + final_move.destination / 8,
                        score
                    );
                }
            }
            PieceColor::Black => {
                temp_position.make_move(&m, true);
                let result = alpha_beta(
                    &temp_position,
                    &depth,
                    &mut f64::NEG_INFINITY,
                    &mut f64::INFINITY,
                );
                if score >= result {
                    score = result;
                    final_move = m;
                    println!(
                        "best-move {}{}{}{} => {}",
                        ('a' as u8 + (final_move.source % 8) as u8) as char,
                        1 + final_move.source / 8,
                        ('a' as u8 + (final_move.destination % 8) as u8) as char,
                        1 + final_move.destination / 8,
                        score
                    );
                }
            }
        }
    }

    final_move
}

fn alpha_beta(position: &Position, depth: &u16, alpha: &mut f64, beta: &mut f64) -> f64 {
    if *depth == 0 {
        return evaluation::evaluate(position);
    }

    let moves = moves_generator::generate_legal_moves(position, &position.get_turn());
    match position.get_turn() {
        PieceColor::None => 0.0,
        PieceColor::White => {
            // To prioritize checkmate
            let mut score = if moves.len() == 0 {
                f64::NEG_INFINITY
            } else {
                -500_000.0
            };
            for mov in moves {
                let mut temp_position = position.clone();
                temp_position.make_move(&mov, true);
                score = score.max(alpha_beta(&temp_position, &(depth - 1), alpha, beta));
                if score >= *beta {
                    break;
                }
                *alpha = alpha.max(score);
            }
            score
        }
        PieceColor::Black => {
            // To prioritize checkmate
            let mut score = if moves.len() == 0 {
                f64::INFINITY
            } else {
                500_000.0
            };
            for mov in moves {
                let mut temp_position = position.clone();
                temp_position.make_move(&mov, true);
                score = score.min(alpha_beta(&temp_position, &(depth - 1), alpha, beta));
                if score <= *alpha {
                    break;
                }
                *beta = beta.min(score);
            }
            score
        }
    }
}
