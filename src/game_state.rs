use crate::history::History;
use crate::moves::Move;
use crate::position::Position;

pub struct GameState {
  position: Position,
  history: History,
}

impl GameState {
  #[inline(always)]
  pub fn new(fen: &str) -> GameState {
    GameState { position: Position::from_fen(fen), history: History::new() }
  }

  #[inline(always)]
  pub fn set_fen(&mut self, fen: &str) {
    self.position = Position::from_fen(fen);
    self.history.clear();
    self.history.save_hash(self.position.get_zobrist_hash())
  }

  #[inline(always)]
  pub fn make_move(&mut self, mov: Move) {
    self.position.make_move(mov);
    self.history.save_hash(self.position.get_zobrist_hash());
  }

  #[inline(always)]
  pub fn unmake_move(&mut self, mov: Move) {
    todo!()
  }

  #[inline(always)]
  pub fn make_null_move(&mut self) -> u8 {
    let ancient_en_passant_file = self.position.make_null_move();
    self.history.save_hash(self.position.get_zobrist_hash());
    ancient_en_passant_file
  }

  #[inline(always)]
  pub fn unmake_null_move(&mut self, ancient_en_passant_file: u8) {
    self.position.unmake_null_move(ancient_en_passant_file);
    self.history.pop_last_entry();
  }
}
