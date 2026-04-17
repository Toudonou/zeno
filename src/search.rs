use std::time::Instant;

use crate::containers::ByColor;
use crate::eval_params::{EvalParams, EVAL_PARAMS_DEFAULT};
use crate::evaluator::{Evaluator, DRAW_VALUE};
use crate::history::History;
use crate::moves::{Move, MoveType};
use crate::moves_picker::MovePicker;
use crate::piece::{PieceColor, PieceType};
use crate::pos_eval::{Evaluation, MATE_SCORE};
use crate::position::Position;
use crate::transposition_table::{TTEntry, TTFlag, TranspositionTable};
use crate::utils::{MAX_PLY, ZENO_INFINITY};

static NMP_DEPTH_LIMIT: i32 = 2;
static NMP_DEPTH_REDUCTION: i32 = 2;
static MAX_EXTENSION: i32 = 16;
static LMR_DEPTH_LIMIT: i32 = 6;
static LMR_FULL_QUIET_MOVE_SEARCHED: i32 = 6;
static LMR_DEPTH_REDUCTION: i32 = 2;
static STATIC_NMP_MARGIN: i32 = 120;
static MAX_HISTORY_BONUS: i32 = MAX_PLY * MAX_PLY;

#[derive(Copy, Clone)]
struct SearchStats {
  pub number_of_nodes_visited: u32,
  pub search_time: u128,
  pub search_depth: u32,
}

#[derive(Copy, Clone)]
struct SearchTables {
  pub killers: [(Move, Move); 1 + MAX_PLY as usize],
  pub counters: ByColor<[[Move; 64]; 64]>,
  pub history_moves: ByColor<[[i32; 64]; 64]>,
}

pub struct Searcher<'a> {
  transposition_table: &'a mut TranspositionTable,
  timer: Instant,
  thinking_time: u128,
  max_depth: i32,
  stop_search: bool,
  search_stats: SearchStats,
  search_tables: SearchTables,
  pv_line: Vec<Move>,
}

impl<'a> Searcher<'a> {
  pub fn new(transposition_table: &'a mut TranspositionTable) -> Searcher<'a> {
    Searcher {
      transposition_table,
      timer: Instant::now(),
      thinking_time: 3000,
      max_depth: MAX_PLY,
      stop_search: false,
      search_stats: SearchStats { number_of_nodes_visited: 0, search_time: 0, search_depth: 0 },
      search_tables: SearchTables {
        killers: [(Move::default(), Move::default()); 1 + MAX_PLY as usize],
        counters: ByColor::new([[Move::default(); 64]; 64], [[Move::default(); 64]; 64]),
        history_moves: ByColor::new([[0; 64]; 64], [[0; 64]; 64]),
      },
      pv_line: Vec::with_capacity(MAX_PLY as usize),
    }
  }

  pub fn search(&mut self, position: &mut Position, history: &mut History, thinking_time: u128) -> Option<Move> {
    let mut score = Evaluation::Score(0);
    self.timer = Instant::now();
    self.thinking_time = thinking_time;
    self.stop_search = false;
    self.pv_line.clear();

    // Iterative deepening
    for depth in 1..=self.max_depth {
      self.search_stats = SearchStats { number_of_nodes_visited: 0, search_time: 0, search_depth: 0 };

      let iterative_timer = Instant::now();
      let mut temp_pv_line: Vec<Move> = vec![];

      // Aspiration Window
      let mut aspiration_window_delta = 30;
      loop {
        if self.timer.elapsed().as_millis() > self.thinking_time {
          break;
        }

        if depth == 1 {
          score = self.pv_search(position, history, 1, depth, -ZENO_INFINITY, ZENO_INFINITY, None, &mut temp_pv_line, 0);
          break;
        } else {
          let alpha = score.value() - aspiration_window_delta;
          let beta = score.value() + aspiration_window_delta;

          score = self.pv_search(position, history, 1, depth, alpha, beta, None, &mut temp_pv_line, 0);
          if !(alpha < score.value() && score.value() < beta) {
            aspiration_window_delta *= 2;
          } else {
            break;
          }
        }
      }

      self.search_stats.search_depth = depth as u32;
      self.search_stats.search_time = iterative_timer.elapsed().as_millis().max(1);
      if !self.stop_search {
        self.pv_line = temp_pv_line;
        self.print_info(depth, self.search_stats, score);

        // Stop the search if a mate was found
        if score.is_mate_score() {
          break;
        }

        // I try to predict the time need to search the next depth.
        // If there is no enough time, the search is automatically canceled
        let estimated_time_ms = self.estimate_time_for_the_next_search(self.search_stats);
        if estimated_time_ms > self.thinking_time - self.timer.elapsed().as_millis() {
          break;
        }
      }
    }

    self.pv_line.first().copied()
  }

  fn pv_search(
    &mut self,
    position: &mut Position,
    history: &mut History,
    ply: i32,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    previous_move: Option<Move>,
    pv_line: &mut Vec<Move>,
    num_extensions: i32,
  ) -> Evaluation {
    self.search_stats.number_of_nodes_visited += 1;

    let side = position.get_side();
    let is_pv = beta - alpha != 1;
    let is_in_check = position.is_check(side);
    let mut child_pv_line: Vec<Move> = Vec::with_capacity(depth.max(1) as usize);

    // Check extension
    let extension = i32::from(num_extensions < MAX_EXTENSION && is_in_check);
    let mut depth = depth + extension;
    depth = depth.clamp(0, MAX_PLY);

    // Check for threefold repetition and fifty-move rule (partially)
    if history.is_repetition(position) || position.get_half_move_clock() >= 100 {
      pv_line.clear();
      return Evaluation::Score(DRAW_VALUE);
    }

    if depth <= 0 || ply > MAX_PLY {
      pv_line.clear();
      return self.quiescence_search(position, alpha, beta, &EVAL_PARAMS_DEFAULT);
    }

    let mut tt_move: Option<Move> = None;
    let tt_entry = self.transposition_table.get_entry(position.get_zobrist_hash());
    if tt_entry.get_flag() != TTFlag::None && tt_entry.get_hash() == position.get_zobrist_hash() {
      let tt_eval = tt_entry.get_evaluation(ply as u32);
      tt_move = tt_entry.get_best_move();
      if ply > 1 && tt_entry.get_depth() as i32 >= depth {
        // In the case of TTFlag::Exact flag, it is best to avoid returning the evaluation as it can result in the drawing of a winning endgame.
        // https://talkchess.com/viewtopic.php?t=20080
        if (tt_entry.get_flag() == TTFlag::LowerBound && tt_eval.value() >= beta) || (tt_entry.get_flag() == TTFlag::UpperBound && tt_eval.value() <= alpha) {
          pv_line.clear();
          pv_line.push(tt_move.unwrap_or_default());
          return tt_eval;
        }
      }
    }

    // Static null move pruning
    if depth <= 3 && !is_in_check && !is_pv && beta < MATE_SCORE {
      let static_score = Evaluator::evaluate(&position, &EVAL_PARAMS_DEFAULT);
      let score_margin = STATIC_NMP_MARGIN * depth;
      if static_score - score_margin >= beta {
        return self.quiescence_search(position, alpha, beta, &EVAL_PARAMS_DEFAULT);
      }
    }

    // Null move
    let can_do_null_move = !is_pv && !is_in_check && position.has_non_pawn_material();
    if can_do_null_move && depth >= NMP_DEPTH_LIMIT {
      let ancient_en_passant_file = position.make_null_move();
      history.save_hash(position.get_zobrist_hash());

      let nmp_reduction = NMP_DEPTH_REDUCTION + (depth as f32 / 6f32) as i32;
      let eval = self.pv_search(position, history, ply + 1, depth - 1 - nmp_reduction, -beta, -alpha, None, &mut child_pv_line, num_extensions) * -1;

      position.unmake_null_move(ancient_en_passant_file);
      history.pop_last_entry();

      if !eval.is_mate_score() && eval.value() >= beta {
        return Evaluation::Score(beta);
      }
    }

    let previous_move = previous_move.unwrap_or_default();
    let mut move_picker: MovePicker = MovePicker::new(
      position,
      tt_move,
      Some(self.search_tables.killers[ply as usize]),
      Some(self.search_tables.counters[side][previous_move.source() as usize][previous_move.destination() as usize]),
      Some(&self.search_tables.history_moves[side]),
      false,
    );
    if move_picker.get_moves_count() == 0 {
      pv_line.clear();
      if is_in_check {
        return Evaluation::MateIn(-ply);
      }
      return Evaluation::Score(DRAW_VALUE);
    }

    let original_alpha = alpha;
    let mut best_eval = Evaluation::Score(-ZENO_INFINITY);
    let mut best_move = None;
    let mut first_move = true;
    let mut quiets_moves: Vec<Move> = Vec::with_capacity(move_picker.get_moves_count());
    while let Some(mov) = move_picker.pick_best_move() {
      let mut eval: Evaluation;
      let is_capture = position.get_piece_on_square(mov.destination()).piece_type != PieceType::None || mov.move_type() == MoveType::EnPassant;
      let is_quiet = !is_capture && !mov.is_promotion();

      let mut temp_position = position.clone();
      temp_position.make_move(mov);
      history.save_hash(temp_position.get_zobrist_hash());

      // Pv move or first move - Full Search
      if first_move == true {
        first_move = false;
        eval = self.pv_search(&mut temp_position, history, ply + 1, depth - 1, -beta, -alpha, Some(mov), &mut child_pv_line, num_extensions + extension) * -1;
      } else {
        let can_lmr = ply > 1 && !is_in_check && is_quiet && !is_pv && depth >= LMR_DEPTH_LIMIT && quiets_moves.len() as i32 >= LMR_FULL_QUIET_MOVE_SEARCHED;
        eval = if can_lmr {
          let lmr_r = (LMR_DEPTH_REDUCTION + depth / (2 * LMR_DEPTH_LIMIT)).clamp(1, depth - 2);
          self.pv_search(&mut temp_position, history, ply + 1, depth - 1 - lmr_r, -alpha - 1, -alpha, Some(mov), &mut child_pv_line, num_extensions + extension) * -1
        } else {
          Evaluation::Score(alpha + 1)
        };

        if eval.value() > alpha {
          eval = self.pv_search(&mut temp_position, history, ply + 1, depth - 1, -alpha - 1, -alpha, Some(mov), &mut child_pv_line, num_extensions + extension) * -1;
          if alpha < eval.value() && eval.value() < beta {
            eval = self.pv_search(&mut temp_position, history, ply + 1, depth - 1, -beta, -alpha, Some(mov), &mut child_pv_line, num_extensions + extension) * -1;
          }
        }
      }

      history.pop_last_entry();

      if eval.value() > best_eval.value() {
        best_eval = eval;
        best_move = Some(mov);

        pv_line.clear();
        pv_line.push(mov);
        pv_line.extend_from_slice(&child_pv_line);
      }

      alpha = alpha.max(best_eval.value());

      if alpha >= beta {
        if is_quiet {
          if mov != self.search_tables.killers[ply as usize].0 {
            self.search_tables.killers[ply as usize].1 = self.search_tables.killers[ply as usize].0;
            self.search_tables.killers[ply as usize].0 = mov;
          }

          if previous_move != Move::default() {
            self.search_tables.counters[side][previous_move.source() as usize][previous_move.destination() as usize] = mov;
          }

          self.update_history_score(depth * depth, side, mov);
          quiets_moves.iter().for_each(|&quiet_move| {
            self.update_history_score(-depth, side, quiet_move);
          })
        }
        break;
      }

      if is_quiet {
        quiets_moves.push(mov);
      }

      if self.timer.elapsed().as_millis() > self.thinking_time {
        self.stop_search = true;
        break;
      }
    }

    if !self.stop_search {
      let mut tt_entry = TTEntry::new(position.get_zobrist_hash(), best_move, depth as u32, TTFlag::Exact, best_eval, ply as u32);
      if best_eval.value() <= original_alpha {
        tt_entry = TTEntry::new(position.get_zobrist_hash(), best_move, depth as u32, TTFlag::UpperBound, best_eval, ply as u32);
      } else if best_eval.value() >= beta {
        tt_entry = TTEntry::new(position.get_zobrist_hash(), best_move, depth as u32, TTFlag::LowerBound, best_eval, ply as u32);
      }
      self.transposition_table.save_entry(tt_entry);
    }

    best_eval
  }

  #[inline(always)]
  pub fn quiescence_search(&mut self, position: &mut Position, mut alpha: i32, beta: i32, eval_params: &EvalParams) -> Evaluation {
    let static_evaluation = Evaluation::Score(Evaluator::evaluate(position, eval_params));

    let mut best_eval = static_evaluation;
    if best_eval.value() >= beta {
      return best_eval;
    }
    if best_eval.value() > alpha {
      alpha = best_eval.value();
    }

    let mut move_picker: MovePicker = MovePicker::new(position, None, None, None, None, true);
    while let Some(mov) = move_picker.pick_best_move() {
      if MovePicker::see_capture(position, mov) < 0 {
        continue;
      }

      let mut temp_position = position.clone();
      temp_position.make_move(mov);

      let eval = self.quiescence_search(&mut temp_position, -beta, -alpha, eval_params) * -1;

      if eval.value() >= beta {
        return eval;
      }
      if eval.value() > alpha {
        alpha = eval.value();
      }
      if eval.value() > best_eval.value() {
        best_eval = eval;
      }

      if self.timer.elapsed().as_millis() > self.thinking_time {
        self.stop_search = true;
        break;
      }
    }

    best_eval
  }

  #[inline(always)]
  fn update_history_score(&mut self, bonus: i32, side: PieceColor, mov: Move) {
    let clamped_bonus = bonus.clamp(-MAX_HISTORY_BONUS, MAX_HISTORY_BONUS);
    self.search_tables.history_moves[side][mov.source() as usize][mov.destination() as usize] +=
      clamped_bonus - self.search_tables.history_moves[side][mov.source() as usize][mov.destination() as usize] * clamped_bonus.abs() / MAX_HISTORY_BONUS;
  }

  #[inline(always)]
  pub fn reset(&mut self) {
    self.transposition_table.clear();
    self.search_tables.killers = [(Move::default(), Move::default()); 1 + MAX_PLY as usize];
    self.search_tables.counters = ByColor::new([[Move::default(); 64]; 64], [[Move::default(); 64]; 64]);
    self.search_tables.history_moves = ByColor::new([[0; 64]; 64], [[0; 64]; 64]);
  }

  #[inline(always)]
  fn print_info(&self, depth: i32, stats: SearchStats, score: Evaluation) {
    print!("info depth {depth} nodes {} time {} nps {} ", stats.number_of_nodes_visited, stats.search_time, (1000 * stats.number_of_nodes_visited as u128 / stats.search_time));

    match score {
      Evaluation::Score(score) => print!("score cp {} ", score),
      Evaluation::MateIn(mate_in) => print!("score mate {} ", mate_in / 2),
    }

    print!("pv ");
    for mov in self.pv_line.clone() {
      print!("{} ", mov)
    }
    println!();
  }

  #[inline(always)]
  fn estimate_time_for_the_next_search(&self, search_stats: SearchStats) -> u128 {
    let speed = 1000 * search_stats.number_of_nodes_visited as u128 / search_stats.search_time;
    let branching_factor = (search_stats.number_of_nodes_visited as f32).powf(1.0 / (search_stats.search_depth as f32));
    let future_depth = search_stats.search_depth + 1;

    // Geometric series because of the iterative deepening
    let nodes_prediction = (branching_factor.powf((future_depth + 1) as f32) - 1.0) / ((branching_factor - 1.0).max(1.0));
    // I only take 80% of the time because the prediction is not that accurate
    (0.8 * (nodes_prediction / speed as f32) * 1000.0) as u128
  }
}
