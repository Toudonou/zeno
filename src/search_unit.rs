use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Instant;

use crate::containers::ByColor;
use crate::eval_params::{EVAL_PARAMS_DEFAULT, EvalParams};
use crate::evaluator::{DRAW_VALUE, Evaluator};
use crate::history::History;
use crate::moves::{Move, MoveType};
use crate::moves_picker::MovePicker;
use crate::piece::{PieceColor, PieceType};
use crate::pos_eval::{Evaluation, MATE_SCORE};
use crate::position::Position;
use crate::search_constants::{
  BASE_ASPIRATION_WINDOW_DELTA, LMP_DEPTH_HORIZON, LMP_MARGINS, LMR_DEPTH_LIMIT, LMR_MOVE_SEARCHED, LMR_REDUCTION, MAX_EXTENSION, MAX_HISTORY_BONUS, NMP_DEPTH_LIMIT, NMP_DEPTH_REDUCTION,
  NODES_BETWEEN_TIME_CHECKS, RAZORING_BASE, RAZORING_DEPTH_HORIZON, RAZORING_MARGIN, STATIC_NMP_DEPTH_HORIZON, STATIC_NMP_MARGIN, SearchLimits, SearchResult, SearchTables,
};
use crate::transposition_table::{TTFlag, TranspositionTable};
use crate::utils::{MAX_PLY, ZENO_INFINITY};

pub struct SearcherUnit {
  thread_id: u32,
  timer: Instant,
  stop_search: bool,
  nodes_visited: u32,
  search_limits: SearchLimits,
  search_tables: SearchTables,
  total_threads_nodes: Arc<AtomicU32>,
  external_stop: Option<Arc<AtomicBool>>,
  transposition_table: Arc<TranspositionTable>,
}

impl SearcherUnit {
  pub fn new(thread_id: u32, transposition_table: Arc<TranspositionTable>, external_stop: Option<Arc<AtomicBool>>, total_threads_nodes: Arc<AtomicU32>) -> SearcherUnit {
    SearcherUnit {
      thread_id,
      external_stop,
      nodes_visited: 0,
      stop_search: false,
      total_threads_nodes,
      transposition_table,
      timer: Instant::now(),
      search_limits: SearchLimits::ThinkingTime(3000),
      search_tables: SearchTables {
        killers: [(Move::default(), Move::default()); 1 + MAX_PLY as usize],
        counters: ByColor::new([[Move::default(); 64]; 64], [[Move::default(); 64]; 64]),
        history_moves: ByColor::new([[0; 64]; 64], [[0; 64]; 64]),
      },
    }
  }

  pub fn search(&mut self, position: &Position, history: &History, search_limits: SearchLimits) -> SearchResult {
    let mut position = position.clone();
    let mut history = history.clone();
    let mut score = Evaluation::CentiPawns(-ZENO_INFINITY);

    self.stop_search = false;
    self.timer = Instant::now();
    self.search_limits = search_limits;

    let mut search_result = SearchResult { mov: None, depth: -1, score, nodes: 0, search_time: 1, pv: vec![] };

    // Iterative deepening
    for depth in 1..=MAX_PLY {
      self.nodes_visited = 0;
      if self.thread_id == 0 {
        self.total_threads_nodes.store(0, Ordering::Relaxed);
      }

      let search_time = Instant::now();
      let mut temp_pv_line: Vec<Move> = vec![];

      // Aspiration Window
      let mut aspiration_window_delta = BASE_ASPIRATION_WINDOW_DELTA;
      loop {
        if self.update_and_check_stop(depth) {
          break;
        }

        if depth == 1 {
          score = self.pv_search(&mut position, &mut history, 1, depth, -ZENO_INFINITY, ZENO_INFINITY, None, &mut temp_pv_line, 0);
          break;
        } else {
          let alpha = score.value() - aspiration_window_delta;
          let beta = score.value() + aspiration_window_delta;

          score = self.pv_search(&mut position, &mut history, 1, depth, alpha, beta, None, &mut temp_pv_line, 0);
          if !(alpha < score.value() && score.value() < beta) {
            aspiration_window_delta *= 2;
          } else {
            break;
          }
        }
      }

      self.total_threads_nodes.fetch_add(self.nodes_visited, Ordering::Relaxed);

      if !self.stop_search {
        search_result.mov = temp_pv_line.first().copied();
        search_result.depth = depth;
        search_result.score = score;
        search_result.nodes = self.total_threads_nodes.load(Ordering::Relaxed);
        search_result.search_time = search_time.elapsed().as_millis().max(1);
        search_result.pv = temp_pv_line.clone();

        if self.thread_id == 0 {
          search_result.print_info();

          // Stop the search if a mate was found
          if score.is_mate_score() {
            break;
          }

          // I try to predict the time needed to search the next depth.
          // If there is no enough time, the search is automatically canceled
          match self.search_limits {
            SearchLimits::ThinkingTime(thinking_time) => {
              let estimated_time_ms = Self::estimate_time_for_the_next_search(depth, self.nodes_visited, search_result.search_time);
              if estimated_time_ms > thinking_time - self.timer.elapsed().as_millis() {
                break;
              }
            }
            SearchLimits::MaxDepth(_) => {}
          }
        }
      }
    }

    search_result
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
    if self.update_and_check_stop(depth) {
      return Evaluation::CentiPawns(-ZENO_INFINITY);
    }

    self.nodes_visited += 1;

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
      return Evaluation::CentiPawns(DRAW_VALUE);
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
    if depth <= STATIC_NMP_DEPTH_HORIZON && !is_in_check && !is_pv && beta < MATE_SCORE {
      let static_score = Evaluator::static_evaluation(&position, &EVAL_PARAMS_DEFAULT);
      let score_margin = STATIC_NMP_MARGIN * depth;
      if static_score >= beta + score_margin {
        return self.quiescence_search(position, alpha, beta, &EVAL_PARAMS_DEFAULT);
      }
    }

    // Razoring
    if depth <= RAZORING_DEPTH_HORIZON && !is_in_check && !is_pv && alpha < MATE_SCORE {
      let static_score = Evaluator::static_evaluation(&position, &EVAL_PARAMS_DEFAULT);
      let razoring_margin = RAZORING_BASE + RAZORING_MARGIN * depth;
      if static_score < alpha - razoring_margin {
        let quiescence_eval = self.quiescence_search(position, alpha, beta, &EVAL_PARAMS_DEFAULT);
        if quiescence_eval.value() < alpha {
          return Evaluation::CentiPawns(alpha);
        }
      }
    }

    // Null move
    let can_do_null_move = !is_pv && !is_in_check && position.has_non_pawn_material();
    if can_do_null_move && depth >= NMP_DEPTH_LIMIT {
      let previous_en_passant_file = position.make_null_move();
      history.save_hash(position.get_zobrist_hash());

      let nmp_reduction = NMP_DEPTH_REDUCTION + depth / 6;
      let eval = self.pv_search(position, history, ply + 1, depth - 1 - nmp_reduction, -beta, -alpha, None, &mut child_pv_line, num_extensions) * -1;

      position.unmake_null_move(previous_en_passant_file);
      history.pop_last_entry();

      if !eval.is_mate_score() && eval.value() >= beta {
        return Evaluation::CentiPawns(beta);
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
      return Evaluation::CentiPawns(DRAW_VALUE);
    }

    let original_alpha = alpha;
    let mut best_eval = Evaluation::CentiPawns(-ZENO_INFINITY);
    let mut best_move = None;
    let mut quiets_moves: Vec<Move> = Vec::with_capacity(move_picker.get_moves_count());
    while let Some((mov, move_index)) = move_picker.pick_best_move() {
      let mut eval: Evaluation;
      let is_capture = position.get_piece_on_square(mov.destination()).piece_type != PieceType::None || mov.move_type() == MoveType::EnPassant;
      let is_quiet = !is_capture && !mov.is_promotion();

      let mut temp_position = position.clone();
      temp_position.make_move(mov);
      history.save_hash(temp_position.get_zobrist_hash());

      // Late move pruning
      if depth <= LMP_DEPTH_HORIZON && is_quiet && !is_pv && !is_in_check && move_index >= LMP_MARGINS[depth as usize] {
        let give_check = temp_position.is_check(side.opposite());
        if !give_check {
          history.pop_last_entry();
          continue;
        }
      }

      // Pv move or first move - Full Search
      if move_index == 0 {
        eval = self.pv_search(&mut temp_position, history, ply + 1, depth - 1, -beta, -alpha, Some(mov), &mut child_pv_line, num_extensions + extension) * -1;
      } else {
        // Late move reduction
        let can_lmr = ply > 1 && !is_in_check && is_quiet && !is_pv && depth >= LMR_DEPTH_LIMIT && quiets_moves.len() as i32 >= LMR_MOVE_SEARCHED;
        let lmr_r = if can_lmr { (LMR_REDUCTION + depth / (2 * LMR_DEPTH_LIMIT)).clamp(1, depth - 2) } else { 0 };

        eval = self.pv_search(&mut temp_position, history, ply + 1, depth - 1 - lmr_r, -alpha - 1, -alpha, Some(mov), &mut child_pv_line, num_extensions + extension) * -1;

        if eval.value() > alpha && lmr_r > 0 {
          eval = self.pv_search(&mut temp_position, history, ply + 1, depth - 1, -alpha - 1, -alpha, Some(mov), &mut child_pv_line, num_extensions + extension) * -1;
        }

        if alpha < eval.value() && eval.value() < beta {
          eval = self.pv_search(&mut temp_position, history, ply + 1, depth - 1, -beta, -alpha, Some(mov), &mut child_pv_line, num_extensions + extension) * -1;
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
    }

    if !self.stop_search {
      let tt_flag = if best_eval.value() <= original_alpha {
        TTFlag::UpperBound
      } else if best_eval.value() >= beta {
        TTFlag::LowerBound
      } else {
        TTFlag::Exact
      };
      self.transposition_table.save_entry(position.get_zobrist_hash(), best_move, depth as u32, tt_flag, best_eval, ply as u32);
    }

    best_eval
  }

  #[inline(always)]
  pub fn quiescence_search(&mut self, position: &mut Position, mut alpha: i32, beta: i32, eval_params: &EvalParams) -> Evaluation {
    self.nodes_visited += 1;
    let static_evaluation = Evaluation::CentiPawns(Evaluator::evaluate(position, eval_params));

    let mut best_eval = static_evaluation;
    if best_eval.value() >= beta {
      return best_eval;
    }
    if best_eval.value() > alpha {
      alpha = best_eval.value();
    }

    let mut move_picker: MovePicker = MovePicker::new(position, None, None, None, None, true);
    while let Some((mov, _)) = move_picker.pick_best_move() {
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

      if self.update_and_check_stop(0) {
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
  fn update_and_check_stop(&mut self, depth: i32) -> bool {
    self.stop_search |= self.nodes_visited & (NODES_BETWEEN_TIME_CHECKS - 1) == 0
      && if self.thread_id == 0 {
        match self.search_limits {
          SearchLimits::ThinkingTime(max_thinking_time) => self.timer.elapsed().as_millis() >= max_thinking_time,
          SearchLimits::MaxDepth(max_searching_depth) => depth > max_searching_depth,
        }
      } else {
        match &self.external_stop {
          None => false,
          Some(external_stop) => external_stop.load(Ordering::Relaxed),
        }
      };

    self.stop_search
  }

  #[inline(always)]
  fn estimate_time_for_the_next_search(depth: i32, nodes_visited: u32, search_time: u128) -> u128 {
    let future_depth = depth + 1;
    let speed = 1000 * nodes_visited as u128 / search_time;
    let branching_factor = (nodes_visited as f32).powf(1.0 / (depth as f32));

    // Geometric series because of the iterative deepening
    let nodes_prediction = (branching_factor.powf((future_depth + 1) as f32) - 1.0) / ((branching_factor - 1.0).max(f32::EPSILON));
    // I only take 80% of the time because the prediction is not that accurate
    (0.8 * (nodes_prediction / speed as f32) * 1000.0) as u128
  }
}
