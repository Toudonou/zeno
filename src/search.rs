use crate::evaluator::Evaluator;
use crate::moves::Move;
use crate::moves_picker::MovePicker;
use crate::pos_eval::Evaluation;
use crate::position::Position;
use crate::utils::ZENO_INFINITY;
use std::time::Instant;

pub static MAX_PLY: u32 = 6;

#[derive(Copy, Clone)]
struct SearchStats {
  pub number_of_nodes_visited: u32,
  pub search_time: u128,
  pub search_depth: u32,
}

pub struct Searcher {
  search_stats: SearchStats,
  pv_line: Vec<Vec<Move>>,
}

impl Searcher {
  pub fn new() -> Searcher {
    Searcher { search_stats: SearchStats { number_of_nodes_visited: 0, search_time: 0, search_depth: 0 }, pv_line: Vec::with_capacity(MAX_PLY as usize) }
  }

  pub fn search(&mut self, position: &mut Position) -> Option<Move> {
    let timer = Instant::now();
    self.search_stats.number_of_nodes_visited = 0;

    self.pv_line = vec![vec![]; (MAX_PLY + 1) as usize];
    let score = self.nega_max_alpha_beta(position, 1, MAX_PLY, -ZENO_INFINITY, ZENO_INFINITY);

    self.search_stats.search_time = timer.elapsed().as_millis().max(1);
    self.search_stats.search_depth = MAX_PLY;

    self.print_info(MAX_PLY as usize, self.search_stats, score);

    self.pv_line[MAX_PLY as usize].first().copied()
  }

  fn nega_max_alpha_beta(&mut self, position: &mut Position, current_ply: u32, max_ply: u32, mut alpha: i32, beta: i32) -> Evaluation {
    self.search_stats.number_of_nodes_visited += 1;

    let depth = max_ply - current_ply + 1;

    if current_ply > max_ply {
      self.pv_line[0] = vec![];
      return self.quiescence_search(position, alpha, beta);
    }

    let mut move_picker: MovePicker = MovePicker::new(position, false);
    if move_picker.get_moves_count() == 0 {
      if position.is_check(position.get_side()) {
        return Evaluation::MateIn(-1 * (current_ply as i32));
      }
      return Evaluation::Score(0);
    }

    let mut best_eval = Evaluation::Score(alpha);
    while let Some(mov) = move_picker.pick_best_move(position) {
      let mut temp_position = position.clone();
      temp_position.make_move(mov);

      let eval = self.nega_max_alpha_beta(&mut temp_position, current_ply + 1, max_ply, -beta, -alpha) * -1;

      if eval.value() > alpha {
        alpha = eval.value();
        best_eval = eval;

        self.pv_line[depth as usize].clear();
        self.pv_line[depth as usize].push(mov);
        let temp = self.pv_line[(depth - 1) as usize].clone();
        self.pv_line[depth as usize].extend(temp);
      }

      if alpha >= beta {
        break;
      }
    }
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

    let mut move_picker: MovePicker = MovePicker::new(position, true);
    while let Some(mov) = move_picker.pick_best_move(position) {
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
    }

    best_eval
  }

  fn print_info(&self, depth: usize, stats: SearchStats, nega_max_score: Evaluation) {
    print!("info depth {} nodes {} time {} nps {} ", depth, stats.number_of_nodes_visited, stats.search_time, (1000 * stats.number_of_nodes_visited as u128 / stats.search_time));

    match nega_max_score {
      Evaluation::Score(score) => print!("score cp {} ", score),
      Evaluation::MateIn(mate_in) => print!("score mate {} ", mate_in / 2),
    }

    print!("pv ");
    for mov in self.pv_line[depth].clone() {
      print!("{} ", mov)
    }
    println!();
  }
}
