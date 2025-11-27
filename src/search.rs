use std::time::{Instant};
use thousands::Separable;
use crate::evaluation::evaluate;
use crate::history::History;
use crate::moves::{Move, MoveType};
use crate::moves_generator::generate_pseudo_legal_moves;
use crate::moves_ordering::order_moves;
use crate::pos_eval::{Evaluation, PosEval, MATE_SCORE};
use crate::position::Position;
use crate::transposition_table::{TTEntry, TTFlag, TranspositionTable};
use crate::piece::PieceType;
use crate::polyglot_book::PolyglotBook;
use crate::utils::ZENO_INFINITY;

pub static MAX_PLY: i32 = 128;

pub enum GameState {
    InProgress,
    WhiteWin,
    BlackWin,
    Draw,
}

#[derive(Copy, Clone)]
struct SearchStats {
    pub number_of_nodes_visited: u32,
    pub number_of_alpha_beta_cut_off: u32,
    pub number_of_tt_cut_off: u32,
    pub number_of_nodes_evaluated: u32,
    pub search_time: u128,
    pub search_depth: u32,
}

pub struct Searcher {
    game_state: GameState,
    timer: Instant,
    thinking_time: u128,
    max_depth: u32,
    is_search_cancel_early: bool,
    search_stats: SearchStats,
    evaluation: Evaluation,
    current_pv_line: Vec<Vec<Move>>,
    killers: [(Move, Move); 1 + MAX_PLY as usize],
    pv_line_per_depth: Vec<Vec<Move>>,

    polyglot_book: PolyglotBook
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            game_state: GameState::InProgress,
            timer: Instant::now(),
            thinking_time: 3000,
            max_depth: MAX_PLY as u32,
            is_search_cancel_early: false,
            search_stats: SearchStats {
                number_of_nodes_visited: 0,
                number_of_alpha_beta_cut_off: 0,
                number_of_tt_cut_off: 0,
                number_of_nodes_evaluated: 0,
                search_time: 0,
                search_depth: 0,
            },
            evaluation: Evaluation::Score(0),
            current_pv_line: vec![],
            killers: [(Move::new(0, 0, MoveType::Normal), Move::new(0, 0, MoveType::Normal)); 1 + MAX_PLY as usize],
            pv_line_per_depth: vec![],

            polyglot_book: PolyglotBook::new("opening_books/Human.bin")
        }
    }

    pub fn search(&mut self, position: &Position, history: &mut History, transposition_table: &mut TranspositionTable, thinking_time: u128) -> Option<Move> {
        match self.polyglot_book.get_book_move(&position.get_hash()) {
            None => {}
            Some(mov) => {
                println!("Book move found: {}", mov);
                return Some(mov);
            }
        }

        self.thinking_time = thinking_time;
        self.timer = Instant::now();

        transposition_table.clear();

        self.is_search_cancel_early = false;
        self.evaluation = Evaluation::Score(0);

        self.pv_line_per_depth.clear();
        self.killers = [(Move::new(0, 0, MoveType::Normal), Move::new(0, 0, MoveType::Normal)); 1 + MAX_PLY as usize];

        let mut best_move: Option<Move> = None;

        let mut final_stats: SearchStats = SearchStats {
            number_of_nodes_visited: 0,
            number_of_alpha_beta_cut_off: 0,
            number_of_tt_cut_off: 0,
            number_of_nodes_evaluated: 0,
            search_time: 0,
            search_depth: 0,
        };

        for depth in 1..=self.max_depth as usize {
            let iterative_timer = Instant::now();

            self.search_stats = SearchStats {
                number_of_nodes_visited: 0,
                number_of_alpha_beta_cut_off: 0,
                number_of_tt_cut_off: 0,
                number_of_nodes_evaluated: 0,
                search_time: 0,
                search_depth: 0,
            };

            self.current_pv_line = vec![vec![]; depth + 1];

            if self.timer.elapsed().as_millis() > self.thinking_time { break; }

            let result = self.pv_search(position, history, transposition_table, 1, depth as u32, -ZENO_INFINITY, ZENO_INFINITY, position.get_turn() as i32);

            self.search_stats.search_depth = depth as u32;
            self.search_stats.search_time = iterative_timer.elapsed().as_millis().max(35);

            if !self.is_search_cancel_early {
                print!("Depth: {} ==> {}ms; ", depth, self.search_stats.search_time.separate_with_commas());

                final_stats = self.search_stats;

                self.pv_line_per_depth.push(self.current_pv_line[depth].clone());
                self.evaluation = result.score * position.get_turn() as i32;
                best_move = result.best_move;

                if self.evaluation.value().abs() >= MATE_SCORE { break; }

                // I try to predict the time need to search the next depth.
                // If there is no enough time, the search is automatically canceled

                let branching_factor = if final_stats.search_depth > 1 {
                    (final_stats.number_of_nodes_visited as f32).powf(1.0 / (final_stats.search_depth as f32))
                } else { 0.0 };

                let speed = 1000 * final_stats.number_of_nodes_visited as u128 / final_stats.search_time;

                // Geometric series because of the iterative deepening
                let future_depth = 1f32 + depth as f32;
                let nodes_needed = (branching_factor.powf(future_depth + 1.0) - 1.0) / (branching_factor - 1.0);

                // I only take 80% of the estimated time because, the predictions are not so good; maybe because of the move ordering
                let estimated_time_ms = ((0.8 * (nodes_needed / speed as f32) * 1000.0) as u128).max(35); // The search seems to take at least 33ms
                println!("Estimated time for depth {}: {}ms", depth + 1, estimated_time_ms);

                if estimated_time_ms > self.thinking_time - self.timer.elapsed().as_millis() { break; }
            }
        }

        println!();
        for depth in 0..self.pv_line_per_depth.len() {
            print!("Depth {}: ", depth + 1);
            for mov in self.pv_line_per_depth[depth].clone() {
                print!("{} ", mov.to_uci_string());
            }
            println!();
        }
        println!();

        let branching_factor = if final_stats.search_depth > 1 {
            (final_stats.number_of_nodes_visited as f64).powf(1.0 / (final_stats.search_depth as f64))
        } else { 0.0 };

        println!("Search stats for last depth searched");
        println!("Branching factor: {:.3}", branching_factor);
        if final_stats.search_time > 0 { println!("Speed {} NPS", (1000 * final_stats.number_of_nodes_visited as u128 / final_stats.search_time).separate_with_commas()) }
        println!("Number of nodes visited: {};", final_stats.number_of_nodes_visited.separate_with_commas());
        println!("Alpha cut off rate: {:.3}%", 100f64 * final_stats.number_of_alpha_beta_cut_off as f64 / final_stats.number_of_nodes_visited as f64);
        println!("TT cut of rate: {:.3}%", 100f64 * final_stats.number_of_tt_cut_off as f64 / final_stats.number_of_nodes_visited as f64);
        println!("Final node evaluation rate: {:.3}%", 100f64 * final_stats.number_of_nodes_evaluated as f64 / final_stats.number_of_nodes_visited as f64);

        println!();
        println!("Evaluation: {}", self.get_evaluation());
        print!("PV Line: ");
        for mov in self.get_pv_line() {
            print!("{} ", mov.to_uci_string());
        }
        println!();
        println!();

        best_move
    }

    fn pv_search(&mut self, position: &Position, history: &mut History, transposition_table: &mut TranspositionTable, current_ply: u32, max_ply: u32, mut alpha: i32, beta: i32, point_of_view: i32) -> PosEval {
        self.search_stats.number_of_nodes_visited += 1;

        let depth = max_ply - current_ply + 1;

        let mut is_full_search = true;

        // Check for threefold repetition
        if history.get_position_occurrences_count(position) >= 3 {
            self.search_stats.number_of_nodes_evaluated += 1;

            self.current_pv_line[depth as usize] = vec![];

            return PosEval { best_move: None, score: Evaluation::Score(0) };
        }

        let tt_entry = transposition_table.get_entry(position.get_hash());
        let mut tt_move: Option<Move> = None;

        if tt_entry.get_flag() != TTFlag::None {
            tt_move = tt_entry.get_best_move();
            let mut will_return_early = false;

            if tt_entry.get_hash() == position.get_hash() && tt_entry.get_depth() >= (max_ply - current_ply) {
                if tt_entry.get_flag() == TTFlag::Exact {
                    will_return_early = true;
                } else if tt_entry.get_flag() == TTFlag::LowerBound && tt_entry.get_evaluation().value() >= beta {
                    will_return_early = true;
                } else if tt_entry.get_flag() == TTFlag::UpperBound && tt_entry.get_evaluation().value() <= alpha {
                    will_return_early = true;
                }

                if will_return_early {
                    self.search_stats.number_of_tt_cut_off += 1;
                    match tt_move {
                        None => return PosEval { best_move: None, score: tt_entry.get_evaluation() },
                        Some(mov) => {
                            let mut temp_position = position.clone();
                            temp_position.make_move(&mov, Some(history));

                            if history.get_position_occurrences_count(&temp_position) < 3 {
                                history.pop_last_entry();

                                self.current_pv_line[depth as usize].clear();
                                self.current_pv_line[depth as usize].push(mov);

                                if depth > 1 {
                                    let temp = self.current_pv_line[(depth - 1) as usize].clone();
                                    self.current_pv_line[depth as usize].extend(temp);
                                }

                                return PosEval { best_move: tt_move, score: tt_entry.get_evaluation() };
                            }
                            history.pop_last_entry();
                        }
                    }
                }
            }
        }


        if current_ply > max_ply {
            self.search_stats.number_of_nodes_evaluated += 1;
            self.current_pv_line[0] = vec![];

            return self.quiescence_search(position, 100, alpha, beta, point_of_view);
        }

        let mut pv_move = None;
        if max_ply > 1 {
            pv_move = self.pv_line_per_depth.get((max_ply - 2) as usize).unwrap().get((current_ply - 1) as usize).copied();
        }


        let original_alpha = alpha;
        let turn = position.get_turn();
        let mut no_legal_moves = true;
        let mut moves = generate_pseudo_legal_moves(position);
        order_moves(&mut moves, position, &pv_move, &tt_move, &Some(self.killers[current_ply as usize]));


        let mut best_eval = PosEval { best_move: None, score: Evaluation::Score(alpha) };

        for mov in &moves {
            let mut temp_position = position.clone();
            temp_position.make_move(&mov, Some(history));
            if !temp_position.is_check(&turn) {
                let mut eval: PosEval;

                // In some sense, it is the first move
                if no_legal_moves {
                    no_legal_moves = false;

                    eval = self.pv_search(&temp_position, history, transposition_table, current_ply + 1, max_ply, -beta, -alpha, -point_of_view);
                    eval.score *= -1;
                } else {
                    eval = self.pv_search(&temp_position, history, transposition_table, current_ply + 1, max_ply, -alpha - 1, -alpha, -point_of_view);
                    eval.score *= -1;

                    if alpha < eval.score.value() && eval.score.value() < beta {
                        eval = self.pv_search(&temp_position, history, transposition_table, current_ply + 1, max_ply, -beta, -alpha, -point_of_view);
                        eval.score *= -1;
                    }
                }

                if eval.score.value() > alpha {
                    alpha = eval.score.value();

                    best_eval.best_move = Some(mov.clone());
                    best_eval.score = eval.score;

                    self.current_pv_line[depth as usize].clear();
                    self.current_pv_line[depth as usize].push(mov.clone());

                    let temp = self.current_pv_line[(depth - 1) as usize].clone();
                    self.current_pv_line[depth as usize].extend(temp);
                }

                if alpha >= beta {
                    if position.get_piece_on_square(&mov.destination()).piece_type != PieceType::None {
                        if *mov != self.killers[current_ply as usize].0 && *mov != self.killers[current_ply as usize].1 {
                            self.killers[current_ply as usize].1 = self.killers[current_ply as usize].0;
                            self.killers[current_ply as usize].0 = *mov;
                        }
                    }

                    history.pop_last_entry();

                    self.search_stats.number_of_alpha_beta_cut_off += 1;
                    is_full_search = false;
                    break;
                }
            }

            history.pop_last_entry();

            if self.timer.elapsed().as_millis() > self.thinking_time {
                self.is_search_cancel_early = true;
                break;
            }
        }

        if no_legal_moves {
            self.search_stats.number_of_nodes_evaluated += 1;

            self.current_pv_line[depth as usize] = vec![];

            if position.is_check(&turn) {
                return PosEval { best_move: None, score: Evaluation::MateIn(-1 * (current_ply as i32)) };
            }
            return PosEval { best_move: None, score: Evaluation::Score(0) };
        }


        let mut tt_entry = TTEntry::new(position.get_hash(), &best_eval.best_move, (max_ply - current_ply) as u32, &TTFlag::Exact, &best_eval.score);
        if best_eval.score.value() <= original_alpha {
            tt_entry = TTEntry::new(position.get_hash(), &best_eval.best_move, (max_ply - current_ply) as u32, &TTFlag::UpperBound, &best_eval.score);
        } else if best_eval.score.value() >= beta {
            tt_entry = TTEntry::new(position.get_hash(), &best_eval.best_move, (max_ply - current_ply) as u32, &TTFlag::LowerBound, &best_eval.score);
        }
        transposition_table.add_entry(tt_entry);

        if is_full_search { self.search_stats.number_of_nodes_evaluated += 1; }

        best_eval
    }

    fn quiescence_search(&mut self, position: &Position, depth: i32, mut alpha: i32, beta: i32, point_of_view: i32) -> PosEval {
        let static_evaluation = PosEval { best_move: None, score: Evaluation::Score(evaluate(position) * point_of_view) };
        if depth <= 0 { return static_evaluation; }

        let mut best_eval = static_evaluation;
        if best_eval.score.value() >= beta { return best_eval; }
        if best_eval.score.value() > alpha { alpha = best_eval.score.value(); }

        let mut moves = generate_pseudo_legal_moves(position);
        order_moves(&mut moves, position, &None, &None, &None);

        let turn = position.get_turn();

        for mov in &moves {
            if position.get_piece_on_square(&mov.destination()).piece_type != PieceType::None {
                let mut temp_position = position.clone();
                temp_position.make_move(&mov, None);
                if !temp_position.is_check(&turn) {
                    let mut eval = self.quiescence_search(&temp_position, depth - 1, -beta, -alpha, -point_of_view);
                    eval.score *= -1;

                    if eval.score.value() >= beta { return eval; }
                    if eval.score.value() > alpha { alpha = eval.score.value(); }

                    if eval.score.value() > best_eval.score.value() {
                        best_eval.best_move = Some(mov.clone());
                        best_eval.score = eval.score;
                    }
                }
            }

            if self.timer.elapsed().as_millis() > self.thinking_time { break; }
        }

        best_eval
    }

    pub fn get_pv_line(&self) -> Vec<Move> { self.pv_line_per_depth.last().unwrap_or(&vec![Move::new(0, 0, MoveType::Normal)]).clone() }
    pub fn get_number_of_nodes_visited(&self) -> u32 { self.search_stats.number_of_nodes_visited }
    pub fn get_evaluation(&self) -> Evaluation { self.evaluation }
}