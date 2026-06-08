use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, mpsc};

use crate::history::History;
use crate::moves::Move;
use crate::position::Position;
use crate::search_constants::{SearchLimits, SearchResult};
use crate::search_unit::SearcherUnit;
use crate::transposition_table::TranspositionTable;

pub struct SearchPool {
  number_of_threads: i32,
  transposition_table: Arc<TranspositionTable>,
}

impl SearchPool {
  pub fn new(transposition_table: Arc<TranspositionTable>, number_of_threads: i32) -> Self {
    Self { number_of_threads: number_of_threads.max(1), transposition_table }
  }

  pub fn search(&self, position: &Position, history: &History, search_limits: SearchLimits) -> Option<Move> {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let total_nodes = Arc::new(AtomicU32::new(0));
    let (sender, receiver) = mpsc::channel::<SearchResult>();

    std::thread::scope(|scope| {
      for id in 0..self.number_of_threads {
        let sender = sender.clone();
        let stop_flag = stop_flag.clone();
        let total_nodes = total_nodes.clone();

        scope.spawn(move || {
          let mut searcher = SearcherUnit::new(id, self.transposition_table.clone(), Some(stop_flag.clone()), total_nodes);
          let search_result = searcher.search(&position, &history, search_limits);

          stop_flag.store(true, Ordering::Relaxed);
          sender.send(search_result).expect("Error sending search results");
        });
      }

      drop(sender);
    });

    let mut best_result: Option<SearchResult> = None;
    while let Ok(search_result) = receiver.recv() {
      best_result = Some(match best_result {
        None => search_result,
        Some(current_best) => {
          if search_result.depth > current_best.depth || (search_result.depth == current_best.depth && search_result.score.value() > current_best.score.value()) {
            search_result
          } else {
            current_best
          }
        }
      })
    }

    let best_result = best_result.expect("At least one unit must produce a result");
    best_result.mov
  }
}
