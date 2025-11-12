use std::time::{Instant};
use crate::evaluation::evaluate;
use crate::moves::Move;
use crate::moves_generator::generate_pseudo_legal_moves;
use crate::moves_ordering::order_moves;
use crate::pos_eval::{Evaluation, PosEval};
use crate::position::Position;

pub enum GameState {
    InProgress,
    WhiteWin,
    BlackWin,
    Draw,
}

pub struct Searcher {
    game_state: GameState,
    timer: Instant,
    max_thinking_time: i32,
    max_depth: i32,
    number_of_nodes_evaluated: i32,
    evaluation: Evaluation,
    pv_line: Vec<Move>,
}

impl Searcher {
    pub fn new() -> Searcher { Searcher { game_state: GameState::InProgress, timer: Instant::now(), max_thinking_time: 1000, max_depth: 6, number_of_nodes_evaluated: 0, evaluation: Evaluation::Score(0), pv_line: vec![] } }

    pub fn search(&mut self, position: &Position) -> Option<Move> {
        self.number_of_nodes_evaluated = 0;
        self.timer = Instant::now();

        let result = self.negamax_alpha_beta(position, 1, self.max_depth, -i32::MAX, i32::MAX, position.get_turn() as i32);
        self.pv_line = result.pv_line.clone();
        self.evaluation = Evaluation::Score(result.score.value() * position.get_turn() as i32);

        result.best_move
    }


    fn negamax_alpha_beta(&mut self, position: &Position, current_ply: i32, max_ply: i32, mut alpha: i32, beta: i32, point_of_view: i32) -> PosEval {
        if current_ply > max_ply {
            self.number_of_nodes_evaluated += 1;
            return PosEval { best_move: None, score: Evaluation::Score(evaluate(position) * point_of_view), pv_line: vec![] };
        }

        let mut moves = generate_pseudo_legal_moves(position);
        order_moves(&mut moves, position);

        let turn = position.get_turn();
        let mut no_legal_moves = true;

        let mut best_eval = PosEval { best_move: None, score: Evaluation::Score(-i32::MAX), pv_line: vec![] };

        for mov in &moves {
            let mut temp_position = position.clone();
            temp_position.make_move(&mov);
            if !temp_position.is_check(&turn) {
                no_legal_moves = false;

                let mut eval = self.negamax_alpha_beta(&temp_position, current_ply + 1, max_ply, -beta, -alpha, -point_of_view);
                eval.score = Evaluation::Score(eval.score.value() * -1);

                if best_eval.score.value() < eval.score.value() {
                    best_eval.best_move = Some(mov.clone());
                    best_eval.score = eval.score;

                    best_eval.pv_line.clear();
                    best_eval.pv_line.push(mov.clone());
                    best_eval.pv_line.extend(eval.pv_line.clone());
                }

                alpha = alpha.max(eval.score.value());
                if alpha >= beta {
                    break;
                }
            }

            if self.timer.elapsed().as_millis() > self.max_thinking_time as u128 { break; }
        }

        if no_legal_moves {
            if position.is_check(&turn) {
                return PosEval { best_move: None, score: Evaluation::MateIn(-current_ply), pv_line: vec![] };
            }
            return PosEval { best_move: None, score: Evaluation::Score(0), pv_line: vec![] };
        }

        best_eval
    }

    pub fn get_pv_line(&self) -> Vec<Move> { self.pv_line.clone() }
    pub fn get_number_of_nodes_evaluated(&self) -> i32 { self.number_of_nodes_evaluated }
    pub fn get_evaluation(&self) -> Evaluation { self.evaluation }
}