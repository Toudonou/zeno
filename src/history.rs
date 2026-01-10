use crate::{position::Position, zobrist_hash::BoardHash};

pub static HISTORY_MAX_SIZE: usize = 1024;

#[derive(Debug, Clone)]
pub struct History {
  history: [BoardHash; HISTORY_MAX_SIZE],
  count: usize,
}

impl History {
  #[inline(always)]
  pub fn new() -> History {
    History { history: [0u64; HISTORY_MAX_SIZE], count: 0 }
  }

  #[inline(always)]
  pub fn save_hash(&mut self, hash: BoardHash) {
    self.history[self.count] = hash;
    self.count += 1;
  }

  #[inline(always)]
  pub fn pop_last_entry(&mut self) {
    self.count -= 1;
  }

  #[inline(always)]
  pub fn get_position_occurrences_count(&self, position: &Position) -> usize {
    // A position cannot be repeated after an undoable moves.
    // So the repetition check should only concern the last n (half_move_clock) reversible moves
    // https://www.freechess.org/Help/HelpFiles/fen.html
    let hash = position.get_zobrist_hash();
    self.history.iter().take(self.count).rev().take(1 + position.get_half_move_clock() as usize).filter(|k| **k == hash).count()
  }

  #[inline(always)]
  pub fn clear(&mut self) {
    self.count = 0;
  }
}
