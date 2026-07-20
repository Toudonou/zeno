use crate::containers::ByColor;
use crate::moves::Move;
use crate::pos_eval::Evaluation;
use crate::utils::MAX_PLY;

pub static DEFAULT_NUMBERS_OF_THREADS: u32 = 1;
pub static MIN_NUMBERS_OF_THREADS: u32 = 1;
pub static MAX_NUMBERS_OF_THREADS: u32 = 1024;

pub static BASE_ASPIRATION_WINDOW_DELTA: i32 = 30;
pub static NODES_BETWEEN_TIME_CHECKS: u32 = 4096;

pub static LMP_DEPTH_HORIZON: i32 = 5;
pub static LMP_MARGINS: [i32; 1 + LMP_DEPTH_HORIZON as usize] = [0, 9, 13, 17, 21, 25];

pub static LMR_DEPTH_LIMIT: i32 = 6;
pub static LMR_MOVE_SEARCHED: i32 = 6;
pub static LMR_REDUCTION: i32 = 2;

pub static MAX_EXTENSION: i32 = 16;
pub static MAX_HISTORY_BONUS: i32 = MAX_PLY * MAX_PLY;

pub static NMP_DEPTH_LIMIT: i32 = 2;
pub static NMP_DEPTH_REDUCTION: i32 = 2;

pub static RAZORING_BASE: i32 = 300;
pub static RAZORING_DEPTH_HORIZON: i32 = 3;
pub static RAZORING_MARGIN: i32 = 60;

pub static STATIC_NMP_DEPTH_HORIZON: i32 = 3;
pub static STATIC_NMP_MARGIN: i32 = 120;

#[derive(Copy, Clone)]
pub struct SearchStats {
  pub search_depth: i32,
  pub search_time: u128,
  pub number_of_nodes_visited: u32,
}

#[derive(Copy, Clone)]
pub struct SearchTables {
  pub counters: ByColor<[[Move; 64]; 64]>,
  pub history_moves: ByColor<[[i32; 64]; 64]>,
  pub killers: [(Move, Move); 1 + MAX_PLY as usize],
}

#[derive(Debug, Clone, Copy)]
pub enum SearchLimits {
  ThinkingTime(u128),
  MaxDepth(i32),
}

pub struct SearchResult {
  pub mov: Option<Move>,
  pub depth: i32,
  pub score: Evaluation,
  pub nodes: u32,
  pub search_time: u128,
  pub pv: Vec<Move>,
}

impl SearchResult {
  #[inline(always)]
  pub fn print_info(&self) {
    print!("info depth {} nodes {} time {} nps {} ", self.depth, self.nodes, self.search_time, 1000 * self.nodes as u128 / self.search_time);

    match self.score {
      Evaluation::Score(score) => print!("score cp {} ", score),
      Evaluation::MateIn(mate_in) => print!("score mate {} ", mate_in / 2),
    }

    print!("pv ");
    for mov in &self.pv {
      print!("{} ", mov)
    }
    println!();
  }
}