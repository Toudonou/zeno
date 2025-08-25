use crate::evaluation;
use crate::moves_generator::generate_pseudo_legal_moves;
use crate::position::Position;
use crate::utils::{Move, PieceColor};

pub fn best_move(position: &Position) -> Option<Move> {
    let moves = generate_pseudo_legal_moves(position, &position.get_turn());
    let mut best_move = None;
    let turn = position.get_turn();
    let mut best_score = -1_000_000 * turn as i32;
    let depth: usize = 6;

    for mov in moves {
        match mov {
            None => break,
            Some(m) => {
                let mut temp_position = position.clone();
                temp_position.make_move(&m, true);
                if !temp_position.is_check(&turn) {
                    let score = alpha_beta(&temp_position, depth - 1, -1_000_000, 1_000_000);

                    if score * turn as i32 == 1_000_000 {
                        best_move = mov;
                        break;
                    }

                    match turn {
                        PieceColor::White => {
                            if score > best_score {
                                best_score = score;
                                best_move = mov;
                            }
                        }
                        PieceColor::Black => {
                            if score < best_score {
                                best_score = score;
                                best_move = mov;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    best_move
}

fn alpha_beta(position: &Position, depth: usize, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return evaluation::evaluate(position);
    }
    let turn = position.get_turn();
    let moves = generate_pseudo_legal_moves(position, &turn);
    let mut no_legal_moves = true;

    let mut score = -100_000 * turn as i32;
    for mov in moves {
        match mov {
            None => break,
            Some(m) => {
                let mut temp_position = position.clone();
                temp_position.make_move(&m, true);
                if !temp_position.is_check(&turn) {
                    no_legal_moves = false;
                    let eval = alpha_beta(&temp_position, depth - 1, alpha, beta);
                    match turn {
                        PieceColor::White => {
                            score = score.max(eval);
                            alpha = alpha.max(score);
                            if beta <= alpha {
                                break;
                            }
                        }
                        PieceColor::Black => {
                            score = score.min(eval);
                            beta = beta.min(score);
                            if beta <= alpha {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if no_legal_moves {
        return if position.is_check(&turn) {
            -1_000_000 * turn as i32 // Checkmate
        } else {
            0 // stalemate
        };
    }

    score
}
