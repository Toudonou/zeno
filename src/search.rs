use crate::position::Position;
use crate::utils::{Move, PieceColor};
use crate::{evaluation, moves_generator};

pub fn best_move(position: &Position) -> Move {
    let mut moves = moves_generator::generate_legal_moves(position, &position.get_turn());
    let mut best_move = moves[0].clone();
    let turn = position.get_turn();
    let mut score = -1_000_000 * turn.clone() as i32;
    let depth: usize = 5;
    for m in moves {
        let mut temp_position = position.clone();
        temp_position.make_move(&m, true);
        let eval = alpha_beta(&temp_position, depth - 1, -1_000_000, 1_000_000);
        match turn {
            PieceColor::White => {
                if eval > score {
                    score = eval;
                    best_move = m;
                }
            }
            PieceColor::Black => {
                if eval < score {
                    score = eval;
                    best_move = m;
                }
            }
            _ => {}
        }
    }
    best_move
}

fn alpha_beta(position: &Position, depth: usize, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return evaluation::evaluate(position);
    }

    let mut moves = moves_generator::generate_legal_moves(position, &position.get_turn());
    if moves.is_empty() {
        return if position.is_check(&position.get_turn()) {
            if position.get_turn() == PieceColor::White {
                -1_000_000
            } else {
                1_000_000
            }
        } else {
            0 // stalemate
        };
    }

    match position.get_turn() {
        PieceColor::None => 0,
        PieceColor::White => {
            let mut score = -100_000;
            for mov in moves {
                let mut temp_position = position.clone();
                temp_position.make_move(&mov, true);
                score = score.max(alpha_beta(&temp_position, depth - 1, alpha, beta));
                alpha = alpha.max(score);
                if beta <= alpha {
                    break;
                }
            }
            score
        }
        PieceColor::Black => {
            let mut score = 100_000;
            for mov in moves {
                let mut temp_position = position.clone();
                temp_position.make_move(&mov, true);
                score = score.min(alpha_beta(&temp_position, depth - 1, alpha, beta));
                beta = beta.min(score);
                if beta <= alpha {
                    break;
                }
            }
            score
        }
    }
}
