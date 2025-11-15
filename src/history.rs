use crate::position::Position;

const MAX_PLY: usize = 1024;

#[derive(Clone)]
pub struct History {
    history: Vec<u64>
}

impl History {
    pub fn new() -> History {
        History { history: Vec::with_capacity(MAX_PLY) }
    }

    pub fn save_position(&mut self, position: &Position) {
        self.history.push(position.get_hash());
    }

    pub fn pop_last_entry(&mut self) {
        self.history.pop();
    }

    pub fn get_position_occurrences_count(&self, position: &Position) -> usize {
        let hash = position.get_hash();
        // history.iter().rev().take(halfmove_clock).filter(|&&k| k == zobrist).count();
        self.history.iter().rev().filter(|k| **k == hash).count()
    }

    pub fn clear(&mut self) {
        self.history.clear()
    }
}