use std::time::{Instant};
use crate::evaluation::evaluate;
use crate::history::History;
use crate::moves::Move;
use crate::moves_generator::generate_pseudo_legal_moves;
use crate::moves_ordering::order_moves;
use crate::pos_eval::{Evaluation, PosEval};
use crate::position::Position;
use crate::piece::PieceType;
use crate::transposition_table::{TTEntry, TTFlag, TranspositionTable};

pub enum GameState {
    InProgress,
    WhiteWin,
    BlackWin,
    Draw,
}

pub struct Searcher {
    game_state: GameState,
    timer: Instant,
    thinking_time: u128,
    max_depth: i32,
    is_search_cancel_early: bool,
    number_of_nodes_visited: i32,
    evaluation: Evaluation,
    pv_line: Vec<Vec<Move>>,
}

impl Searcher {
    pub fn new() -> Searcher { Searcher { game_state: GameState::InProgress, timer: Instant::now(), thinking_time: 3000, max_depth: 40, is_search_cancel_early: false, number_of_nodes_visited: 0, evaluation: Evaluation::Score(0), pv_line: vec![] } }

    pub fn search(&mut self, position: &Position, mut history: Option<&mut History>, mut transposition_table: Option<&mut TranspositionTable>, thinking_time: u128) -> Option<Move> {
        self.thinking_time = thinking_time;
        self.timer = Instant::now();

        self.number_of_nodes_visited = 0;
        self.is_search_cancel_early = false;
        self.evaluation = Evaluation::Score(0);

        // The whole "last_pv_line" thing is to reuse the previous work done by the engine
        // It has to be removed  if I can achieve some speed improvement

        let mut last_pv_line = if let Some(last_line) = self.pv_line.last() {
            last_line.clone()
        } else { vec![] };

        self.pv_line = Vec::with_capacity(self.max_depth as usize);
        if last_pv_line.len() > 2 {
            last_pv_line.remove(1);
            last_pv_line.remove(0);

            for _ in 0..self.max_depth {
                self.pv_line.push(last_pv_line.clone());
            }
        } else {
            for _ in 0..self.max_depth {
                self.pv_line.push(vec![]);
            }
        }

        let mut best_move: Option<Move> = None;

        for depth in 1..=self.max_depth {
            if self.timer.elapsed().as_millis() > self.thinking_time { break; }

            let result = self.pv_search(position, history.as_deref_mut(), transposition_table.as_deref_mut(), 1, depth, -i32::MAX, i32::MAX, position.get_turn() as i32);

            if !self.is_search_cancel_early {
                self.pv_line[depth as usize] = result.pv_line.clone();
                self.evaluation = result.score * position.get_turn() as i32;
                best_move = result.best_move;
            } else {
                for i in (depth as usize..self.max_depth as usize).rev() {
                    self.pv_line.remove(i);
                }
            }
        }

        for i in 0..self.pv_line.len() {
            print!("Depth {}: ", i + 1);
            for mov in self.pv_line.get(i).unwrap() {
                print!("{} ", mov.to_uci_string());
            }
            println!();
        }
        println!();

        best_move
    }

    fn pv_search(&mut self, position: &Position, mut history: Option<&mut History>, mut transposition_table: Option<&mut TranspositionTable>, current_ply: i32, max_ply: i32, mut alpha: i32, beta: i32, point_of_view: i32) -> PosEval {
        self.number_of_nodes_visited += 1;

        // Check for threefold repetition
        if history.as_deref_mut().unwrap().get_position_occurrences_count(position) >= 3 {
            return PosEval { best_move: None, score: Evaluation::Score(0), pv_line: vec![] };
        }

        let tt_entry = transposition_table.as_deref_mut().unwrap().get_entry(position.get_hash());
        let mut tt_move: Option<Move> = None;

        match tt_entry {
            None => {}
            Some(entry) => {
                tt_move = entry.pos_eval.best_move;
                let mut will_return_early = false;

                if entry.hash == position.get_hash() && entry.depth >= (max_ply - current_ply) {
                    // https://talkchess.com/posting.php?mode=quote&p=179317&sid=1d8df817122d148f7c7c83bedc9c23e8
                    if entry.flag == TTFlag::Exact {
                        will_return_early = false;
                    } else if entry.flag == TTFlag::LowerBound && entry.pos_eval.score.value() >= beta {
                        will_return_early = true;
                    } else if entry.flag == TTFlag::UpperBound && entry.pos_eval.score.value() <= alpha {
                        will_return_early = true;
                    }

                    if will_return_early {
                        match entry.pos_eval.best_move {
                            None => return entry.pos_eval,
                            Some(mov) => {
                                let mut temp_position = position.clone();
                                temp_position.make_move(&mov, history.as_deref_mut());

                                if history.as_deref_mut().unwrap().get_position_occurrences_count(&temp_position) < 3 {
                                    history.as_deref_mut().unwrap().pop_last_entry();
                                    return entry.pos_eval;
                                }
                                history.as_deref_mut().unwrap().pop_last_entry();
                            }
                        }
                    }
                }
            }
        }


        if current_ply > max_ply {
            return self.quiescence_search(position, Some(&mut History::new()), 100, alpha, beta, point_of_view);
        }

        let pv_move = if let Some(current_pv_line) = self.pv_line.get((current_ply - 1) as usize) {
            current_pv_line.first().copied()
        } else { None };

        let original_alpha = alpha;
        let mut moves = generate_pseudo_legal_moves(position);
        order_moves(&mut moves, position, &pv_move, &tt_move);

        let turn = position.get_turn();
        let mut no_legal_moves = true;

        let mut best_eval = PosEval { best_move: None, score: Evaluation::Score(alpha), pv_line: vec![] };

        for mov in &moves {
            let mut temp_position = position.clone();
            temp_position.make_move(&mov, history.as_deref_mut());
            if !temp_position.is_check(&turn) {
                let mut eval: PosEval;

                // In some sense, it is the first move
                if no_legal_moves {
                    no_legal_moves = false;

                    eval = self.pv_search(&temp_position, history.as_deref_mut(), transposition_table.as_deref_mut(), current_ply + 1, max_ply, -beta, -alpha, -point_of_view);
                    eval.score *= -1;
                } else {
                    eval = self.pv_search(&temp_position, history.as_deref_mut(), transposition_table.as_deref_mut(), current_ply + 1, max_ply, -alpha - 1, -alpha, -point_of_view);
                    eval.score *= -1;

                    if alpha < eval.score.value() && eval.score.value() < beta {
                        eval = self.pv_search(&temp_position, history.as_deref_mut(), transposition_table.as_deref_mut(), current_ply + 1, max_ply, -beta, -alpha, -point_of_view);
                        eval.score *= -1;
                    }
                }

                if eval.score.value() > alpha {
                    alpha = eval.score.value();

                    best_eval.best_move = Some(mov.clone());
                    best_eval.score = eval.score;

                    best_eval.pv_line = Vec::with_capacity(eval.pv_line.len() + 1);
                    best_eval.pv_line.push(mov.clone());
                    best_eval.pv_line.extend(eval.pv_line.clone());
                }

                if alpha >= beta {
                    history.as_deref_mut().unwrap().pop_last_entry();
                    break;
                }
            }

            history.as_deref_mut().unwrap().pop_last_entry();

            if self.timer.elapsed().as_millis() > self.thinking_time {
                self.is_search_cancel_early = true;
                break;
            }
        }

        if no_legal_moves {
            if position.is_check(&turn) {
                return PosEval { best_move: None, score: Evaluation::MateIn(-current_ply), pv_line: vec![] };
            }
            return PosEval { best_move: None, score: Evaluation::Score(0), pv_line: vec![] };
        }


        let mut tt_entry = TTEntry { flag: TTFlag::Exact, pos_eval: best_eval.clone(), hash: position.get_hash(), depth: max_ply - current_ply };
        if tt_entry.pos_eval.score.value() <= original_alpha {
            tt_entry.flag = TTFlag::UpperBound;
        } else if tt_entry.pos_eval.score.value() >= beta {
            tt_entry.flag = TTFlag::LowerBound;
        }
        transposition_table.as_deref_mut().unwrap().add_entry(tt_entry);

        best_eval
    }

    fn quiescence_search(&mut self, position: &Position, mut history: Option<&mut History>, depth: i32, mut alpha: i32, beta: i32, point_of_view: i32) -> PosEval {
        self.number_of_nodes_visited += 1;

        let static_evaluation = PosEval { best_move: None, score: Evaluation::Score(evaluate(position) * point_of_view), pv_line: vec![] };
        if depth <= 0 { return static_evaluation; }

        let mut best_eval = static_evaluation;
        if best_eval.score.value() >= beta { return best_eval; }
        if best_eval.score.value() > alpha { alpha = best_eval.score.value(); }

        let mut moves = generate_pseudo_legal_moves(position);
        order_moves(&mut moves, position, &None, &None);

        let turn = position.get_turn();

        for mov in &moves {
            if position.get_piece_on_square(&mov.destination()).piece_type != PieceType::None {
                let mut temp_position = position.clone();
                temp_position.make_move(&mov, history.as_deref_mut());
                if !temp_position.is_check(&turn) {
                    let mut eval = self.quiescence_search(&temp_position, history.as_deref_mut(), depth - 1, -beta, -alpha, -point_of_view);
                    eval.score *= -1;

                    if eval.score.value() >= beta { return eval; }
                    if eval.score.value() > alpha { alpha = eval.score.value(); }

                    if eval.score.value() > best_eval.score.value() {
                        best_eval.best_move = Some(mov.clone());
                        best_eval.score = eval.score;

                        best_eval.pv_line.clear();
                        best_eval.pv_line.push(mov.clone());
                        best_eval.pv_line.extend(eval.pv_line.clone());
                    }
                }
            }

            if self.timer.elapsed().as_millis() > self.thinking_time { break; }
        }

        best_eval
    }

    pub fn get_pv_line(&self) -> Vec<Move> { self.pv_line.last().unwrap().clone() }
    pub fn get_number_of_nodes_visited(&self) -> i32 { self.number_of_nodes_visited }
    pub fn get_evaluation(&self) -> Evaluation { self.evaluation }
}