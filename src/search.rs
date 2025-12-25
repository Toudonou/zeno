use crate::moves::{Move, MoveList};
use crate::moves_generator::generate_legal_moves;
use crate::position::Position;
use crate::utils::random_u64;

pub static MAX_PLY: i32 = 128;

pub struct Searcher {}

impl Searcher {
  pub fn new() -> Searcher {
    Searcher {}
  }

  pub fn search(&mut self, position: &mut Position) -> Option<Move> {
    let mut move_list = MoveList::new();
    generate_legal_moves(position, &mut move_list);

    if move_list.count != 0 { Some(move_list.moves[random_u64() as usize % move_list.count]) } else { None }
  }
}
