use std::time::Instant;

use crate::containers::ByColor;
use crate::evaluator::Evaluator;
use crate::history::History;
use crate::moves::{Move, MoveType};
use crate::moves_picker::MovePicker;
use crate::piece::PieceType;
use crate::pos_eval::{Evaluation, MATE_SCORE};
use crate::position::Position;
use crate::transposition_table::{TTEntry, TTFlag, TranspositionTable};
use crate::utils::{MAX_PLY, ZENO_INFINITY};

static NMP_DEPTH_LIMIT: u32 = 2;
static NMP_DEPTH_REDUCTION: u32 = 2;

#[derive(Copy, Clone)]
struct SearchStats {
  pub number_of_nodes_visited: u32,
  pub search_time: u128,
  pub search_depth: u32,
}

pub struct Searcher {
  timer: Instant,
  thinking_time: u128,
  max_depth: u32,
  stop_search: bool,
  search_stats: SearchStats,
  killers: [(Move, Move); 1 + MAX_PLY as usize],
  counters: ByColor<[[Move; 64]; 64]>,
  history_moves: ByColor<[[i32; 64]; 64]>,
  pv_line: Vec<Move>,
}

impl Searcher {
  pub fn new() -> Searcher {
    Searcher {
      timer: Instant::now(),
      thinking_time: 3000,
      max_depth: MAX_PLY as u32,
      stop_search: false,
      search_stats: SearchStats { number_of_nodes_visited: 0, search_time: 0, search_depth: 0 },
      killers: [(Move::default(), Move::default()); 1 + MAX_PLY as usize],
      counters: ByColor::new([[Move::default(); 64]; 64], [[Move::default(); 64]; 64]),
      history_moves: ByColor::new([[0; 64]; 64], [[0; 64]; 64]),
      pv_line: Vec::with_capacity(MAX_PLY as usize),
    }
  }

  pub fn search(&mut self, position: &mut Position, transposition_table: &mut TranspositionTable, history: &mut History, thinking_time: u128) -> Option<Move> {
    transposition_table.clear();

    let mut score = Evaluation::Score(0);
    self.timer = Instant::now();
    self.thinking_time = thinking_time;
    self.search_stats.number_of_nodes_visited = 0;
    self.stop_search = false;
    self.killers = [(Move::default(), Move::default()); 1 + MAX_PLY as usize];
    self.counters = ByColor::new([[Move::default(); 64]; 64], [[Move::default(); 64]; 64]);
    self.history_moves = ByColor::new([[0; 64]; 64], [[0; 64]; 64]);
    self.pv_line.clear();

    // Iterative deepening
    for depth in 1..=self.max_depth as usize {
      self.search_stats = SearchStats { number_of_nodes_visited: 0, search_time: 0, search_depth: 0 };

      let iterative_timer = Instant::now();
      let mut triangular_pv: Vec<Vec<Move>> = vec![Vec::with_capacity(depth); depth + 1];

      // Aspiration Window
      let mut aspiration_window_delta = 30;
      loop {
        if self.timer.elapsed().as_millis() > self.thinking_time {
          break;
        }

        if depth == 1 {
          score = self.pv_search(position, transposition_table, history, &mut triangular_pv, 1, depth as u32, -ZENO_INFINITY, ZENO_INFINITY, None);
          break;
        } else {
          let alpha = score.value() - aspiration_window_delta;
          let beta = score.value() + aspiration_window_delta;

          score = self.pv_search(position, transposition_table, history, &mut triangular_pv, 1, depth as u32, alpha, beta, None);
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
        self.pv_line = triangular_pv.last().unwrap().clone();
        self.print_info(depth, self.search_stats, score);

        // Stop the search if a mate was found
        if score.value().abs() >= MATE_SCORE {
          break;
        }

        let estimated_time_ms = self.estimate_time_for_the_next_search(self.search_stats, depth + 1);
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
    transposition_table: &mut TranspositionTable,
    history: &mut History,
    triangular_pv: &mut Vec<Vec<Move>>,
    current_ply: u32,
    max_ply: u32,
    mut alpha: i32,
    beta: i32,
    previous_move: Option<Move>,
  ) -> Evaluation {
    self.search_stats.number_of_nodes_visited += 1;

    let depth = max_ply - current_ply + 1;

    // Check for threefold repetition and fifty-move rule (partially)
    if history.get_position_occurrences_count(position) >= 3 || position.get_half_move_clock() >= 100 {
      triangular_pv[depth as usize] = vec![];
      return Evaluation::Score(0);
    }

    if depth <= 0 {
      triangular_pv[0] = vec![];
      return self.quiescence_search(position, alpha, beta);
    }

    let mut tt_move: Option<Move> = None;
    let tt_entry = transposition_table.get_entry(position.get_zobrist_hash());
    if tt_entry.get_flag() != TTFlag::None {
      tt_move = tt_entry.get_best_move();
      if tt_entry.get_hash() == position.get_zobrist_hash() && tt_entry.get_depth() >= depth {
        let will_return_early = tt_entry.get_flag() == TTFlag::Exact
          || (tt_entry.get_flag() == TTFlag::LowerBound && tt_entry.get_evaluation().value() >= beta)
          || (tt_entry.get_flag() == TTFlag::UpperBound && tt_entry.get_evaluation().value() <= alpha);

        if will_return_early {
          triangular_pv[depth as usize] = vec![];
          return tt_entry.get_evaluation();
        }
      }
    }

    let side = position.get_side();
    let is_pv = beta - alpha != 1;
    let is_in_check = position.is_check(side);

    // Null move
    let can_do_null_move = !is_pv && !is_in_check && position.has_non_pawn_material();
    if can_do_null_move && depth > NMP_DEPTH_LIMIT {
      let ancient_en_passant_file = position.make_null_move();
      history.save_hash(position.get_zobrist_hash());

      let nmp_reduction = NMP_DEPTH_REDUCTION + (depth as f32 / 6f32) as u32;
      let eval = self.pv_search(position, transposition_table, history, triangular_pv, current_ply + 1, max_ply - nmp_reduction, -beta, -alpha, None) * -1;

      position.unmake_null_move(ancient_en_passant_file);
      history.pop_last_entry();

      if eval.value().abs() < MATE_SCORE && eval.value() >= beta {
        return Evaluation::Score(beta);
      }
    }

    let previous_move = previous_move.unwrap_or_default();
    let mut move_picker: MovePicker = MovePicker::new(
      position,
      tt_move,
      Some(self.killers[current_ply as usize]),
      Some(self.counters[side][previous_move.source() as usize][previous_move.destination() as usize]),
      Some(&self.history_moves[side]),
      false,
    );
    if move_picker.get_moves_count() == 0 {
      triangular_pv[depth as usize] = vec![];
      if is_in_check {
        return Evaluation::MateIn(-(current_ply as i32));
      }
      return Evaluation::Score(0);
    }

    let original_alpha = alpha;
    let mut best_eval = Evaluation::Score(-ZENO_INFINITY);
    let mut best_move = None;
    let mut first_move = true;
    while let Some(mov) = move_picker.pick_best_move() {
      let mut eval: Evaluation;
      let is_capture = position.get_piece_on_square(mov.destination()).piece_type != PieceType::None || mov.move_type() == MoveType::EnPassant;
      let is_quiet = !is_capture && !mov.is_promotion();

      let mut temp_position = position.clone();
      temp_position.make_move(mov);
      history.save_hash(temp_position.get_zobrist_hash());

      // Pv move or first move - Full Search
      match first_move {
        true => {
          first_move = false;
          eval = self.pv_search(&mut temp_position, transposition_table, history, triangular_pv, current_ply + 1, max_ply, -beta, -alpha, Some(mov)) * -1;
        }
        false => {
          eval = self.pv_search(&mut temp_position, transposition_table, history, triangular_pv, current_ply + 1, max_ply, -alpha - 1, -alpha, Some(mov)) * -1;
          // If all search that fail-high (score > alpha) do full research in the full windows and at the max depth
          if alpha < eval.value() && eval.value() < beta {
            eval = self.pv_search(&mut temp_position, transposition_table, history, triangular_pv, current_ply + 1, max_ply, -beta, -alpha, Some(mov)) * -1;
          }
        }
      }

      history.pop_last_entry();

      if eval.value() > best_eval.value() {
        best_eval = eval;
        best_move = Some(mov);

        triangular_pv[depth as usize].clear();
        triangular_pv[depth as usize].push(mov);
        let temp = triangular_pv[(depth - 1) as usize].clone();
        triangular_pv[depth as usize].extend(temp);
      }

      alpha = alpha.max(best_eval.value());

      if alpha >= beta {
        if is_quiet {
          if mov != self.killers[current_ply as usize].0 {
            self.killers[current_ply as usize].1 = self.killers[current_ply as usize].0;
            self.killers[current_ply as usize].0 = mov;
          }

          if previous_move != Move::default() {
            self.counters[side][previous_move.source() as usize][previous_move.destination() as usize] = mov;
          }

          let bonus = (depth * depth) as i32;
          let clamped_bonus = bonus.clamp(-16384, 16384);
          self.history_moves[side][mov.source() as usize][mov.destination() as usize] += clamped_bonus - self.history_moves[side][mov.source() as usize][mov.destination() as usize] * clamped_bonus.abs() / 16384;
        }
        break;
      }

      if self.timer.elapsed().as_millis() > self.thinking_time {
        self.stop_search = true;
        break;
      }
    }

    let mut tt_entry = TTEntry::new(position.get_zobrist_hash(), best_move, depth, TTFlag::Exact, best_eval);
    if best_eval.value() <= original_alpha {
      tt_entry = TTEntry::new(position.get_zobrist_hash(), best_move, depth, TTFlag::UpperBound, best_eval);
    } else if best_eval.value() >= beta {
      tt_entry = TTEntry::new(position.get_zobrist_hash(), best_move, depth, TTFlag::LowerBound, best_eval);
    }
    transposition_table.add_entry(tt_entry);

    best_eval
  }

  fn quiescence_search(&mut self, position: &mut Position, mut alpha: i32, beta: i32) -> Evaluation {
    let static_evaluation = Evaluation::Score(Evaluator::evaluate(position) * position.get_side().to_i32());

    let mut best_eval = static_evaluation;
    if best_eval.value() >= beta {
      return best_eval;
    }
    if best_eval.value() > alpha {
      alpha = best_eval.value();
    }

    let mut move_picker: MovePicker = MovePicker::new(position, None, None, None, None, true);
    while let Some(mov) = move_picker.pick_best_move() {
      let mut temp_position = position.clone();
      temp_position.make_move(mov);

      let eval = self.quiescence_search(&mut temp_position, -beta, -alpha) * -1;

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

  fn print_info(&self, depth: usize, stats: SearchStats, score: Evaluation) {
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

  fn estimate_time_for_the_next_search(&self, search_stats: SearchStats, future_depth: usize) -> u128 {
    // I try to predict the time need to search the next depth.
    // If there is no enough time, the search is automatically canceled

    let speed = 1000 * search_stats.number_of_nodes_visited as u128 / search_stats.search_time;
    let branching_factor = if search_stats.search_depth > 1 {
      (search_stats.number_of_nodes_visited as f32).powf(1.0 / (search_stats.search_depth as f32))
    } else {
      0.0
    };

    // Geometric series because of the iterative deepening
    let nodes_needed = (branching_factor.powf((future_depth + 1) as f32) - 1.0) / (branching_factor - 1.0);
    // I only take 80% of the time because the prediction is not that accurate
    ((0.8 * (nodes_needed / speed as f32) * 1000.0) as u128).max(1)
  }
}
