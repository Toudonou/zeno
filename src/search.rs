use crate::evaluation::{evaluate, MATE_SCORE};
use crate::moves::Move;
use crate::moves_generator::generate_pseudo_legal_moves;
use crate::position::Position;

pub enum GameState {
    InProgress,
    WhiteWin,
    BlackWin,
    Draw,
}

#[derive(Debug)]
pub struct PosEval {
    pub best_move: Option<Move>,
    pub score: i32,
}


pub struct Searcher {
    game_state: GameState,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher { game_state: GameState::InProgress }
    }

    pub fn search(&self, position: &Position) -> Option<Move> {
        self.negamax_alpha_beta(position, 6, -i32::MAX, i32::MAX, position.get_turn() as i32).best_move
    }

    fn negamax_alpha_beta(&self, position: &Position, depth: i32, mut alpha: i32, beta: i32, point_of_view: i32) -> PosEval {
        if depth <= 0 {
            return PosEval { best_move: None, score: evaluate(position) * point_of_view };
        }

        let moves = generate_pseudo_legal_moves(position);
        let turn = position.get_turn();
        let mut no_legal_moves = true;

        let mut best_eval = PosEval { best_move: None, score: -i32::MAX };

        for mov in &moves {
            let mut temp_position = position.clone();
            temp_position.make_move(&mov);
            if !temp_position.is_check(&turn) {
                no_legal_moves = false;

                let eval = -self.negamax_alpha_beta(&temp_position, depth - 1, -beta, -alpha, -point_of_view).score;

                if best_eval.score < eval {
                    best_eval.best_move = Option::from(mov.clone());
                    best_eval.score = eval;
                }

                alpha = std::cmp::max(alpha, best_eval.score);
                if alpha >= beta {
                    break;
                }
            }
        }

        if no_legal_moves {
            if position.is_check(&turn) {
                return PosEval { best_move: None, score: -(MATE_SCORE + depth) };
            }
            return PosEval { best_move: None, score: 0 };
        }

        best_eval
    }
}