use crate::position::Position;

#[derive(Debug, Clone)]
pub struct History {
    pub history: Vec<u64>,
}

impl History {
    pub fn new() -> History {
        History { history: Vec::with_capacity(4096) }
    }

    pub fn save_position(&mut self, position: &Position) {
        self.history.push(position.get_hash());
    }

    pub fn pop_last_entry(&mut self) {
        self.history.pop();
    }

    pub fn get_position_occurrences_count(&self, position: &Position) -> usize {
        // A position cannot be repeated after an undoable moves.
        // So the repetition check should only concern the last n (half_move_clock) doable moves
        // https://www.freechess.org/Help/HelpFiles/fen.html
        let hash = position.get_hash();
        self.history.iter().rev().take(1 + position.get_half_move_clock() as usize).filter(|k| **k == hash).count()
    }

    pub fn clear(&mut self) { self.history.clear() }
}