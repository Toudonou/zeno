use crate::{evaluator::Score, zobrist_hash::BoardHash};

pub struct PawnHash {
  pub hash: BoardHash,
  pub score: Score,
}
