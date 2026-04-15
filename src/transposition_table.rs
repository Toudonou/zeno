use thousands::Separable;

use crate::moves::Move;
use crate::pos_eval::Evaluation;
use crate::zobrist_hash::BoardHash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TTFlag {
  None,
  Exact,
  LowerBound,
  UpperBound,
}

impl TTFlag {
  #[inline(always)]
  pub fn to_u32(&self) -> u32 {
    match self {
      TTFlag::None => 0,
      TTFlag::Exact => 1,
      TTFlag::LowerBound => 2,
      TTFlag::UpperBound => 3,
    }
  }

  #[inline(always)]
  pub fn from_u32(value: u32) -> TTFlag {
    match value {
      0 => TTFlag::None,
      1 => TTFlag::Exact,
      2 => TTFlag::LowerBound,
      3 => TTFlag::UpperBound,
      _ => panic!("Invalid value: {}", value),
    }
  }
}

/// The information about the Entry will be stored in a 64-bits value (8 bytes); the TTEntry will be 16-bytes overall
/// - 1 byte: for the flag
/// - 1 byte for the depth
/// - 2 bytes: for the move
/// - 4 bytes for the evaluation
#[derive(Clone, Copy)]
pub struct TTEntry {
  hash: BoardHash,
  others_information: u64,
}

impl TTEntry {
  #[inline(always)]
  pub fn new(hash: BoardHash, best_move: Option<Move>, depth: u32, flag: TTFlag, evaluation: Evaluation, ply: u32) -> TTEntry {
    let best_move = if let Some(m) = best_move { m.to_u16() } else { 0 } as u64;
    let flag = flag.to_u32() as u64;
    let depth = depth as u64;

    let evaluation = match evaluation {
      Evaluation::Score(_) => evaluation.to_u32() as u64,
      Evaluation::MateIn(mate_in) => Evaluation::MateIn(mate_in.signum() * (mate_in.abs() - ply as i32)).to_u32() as u64,
    };

    TTEntry { hash, others_information: (evaluation << (4 * 8)) | (best_move << (2 * 8)) | (depth << (1 * 8)) | (flag) }
  }

  #[inline(always)]
  pub fn get_hash(&self) -> BoardHash {
    self.hash
  }

  #[inline(always)]
  pub fn get_evaluation(&self, ply: u32) -> Evaluation {
    let eval = Evaluation::from_u32(((self.others_information >> (4 * 8)) & 0xFFFFFFFF) as u32);

    match eval {
      Evaluation::Score(_) => eval,
      Evaluation::MateIn(mate_in) => Evaluation::MateIn(mate_in.signum() * (mate_in.abs() + ply as i32)),
    }
  }

  #[inline(always)]
  pub fn get_best_move(&self) -> Option<Move> {
    let move_code = ((self.others_information >> (2 * 8)) & 0xFFFF) as u16;
    if move_code != 0 { Some(Move::from_u16(move_code)) } else { None }
  }

  #[inline(always)]
  pub fn get_depth(&self) -> u32 {
    ((self.others_information >> (1 * 8)) & 0xFF) as u32
  }

  #[inline(always)]
  pub fn get_flag(&self) -> TTFlag {
    TTFlag::from_u32(((self.others_information) & 0xFF) as u32)
  }
}

#[derive(Clone)]
pub struct TranspositionTable {
  table: Vec<TTEntry>,
  max_entries: usize,
}

impl TranspositionTable {
  /// Default size: 16MB
  #[inline(always)]
  pub fn default() -> TranspositionTable {
    Self::with_capacity(16)
  }

  /// TT size between 16MB and 1024MB
  #[inline(always)]
  pub fn with_capacity(tt_size_mb: u32) -> TranspositionTable {
    let tt_size_mb = tt_size_mb.clamp(16, 1024);

    let max_entries = ((tt_size_mb * 1024 * 1024) as usize / size_of::<TTEntry>()) as f32;
    let max_entries = 2usize.pow(max_entries.log2().ceil() as u32);

    TranspositionTable { table: vec![TTEntry::new(0, None, 0, TTFlag::None, Evaluation::Score(0), 0); max_entries], max_entries }
  }

  #[inline(always)]
  pub fn get_entry(&self, hash: BoardHash) -> TTEntry {
    self.table[hash as usize & (self.max_entries - 1)] // max_entries is a power of 2, therefore (x % max_entries) == x & (max_entries)
  }

  #[inline(always)]
  pub fn save_entry(&mut self, entry: TTEntry) {
    self.table[entry.get_hash() as usize & (self.max_entries - 1)] = entry; // max_entries is a power of 2, therefore (x % max_entries) == x & (max_entries)
  }

  pub fn print_transposition_stats(&self) {
    let count = self.table.iter().filter(|x| x.get_flag() != TTFlag::None).count();
    println!("Transposition utilization: {}/{} = {:.3}%", count.separate_with_commas(), self.max_entries.separate_with_commas(), 100f64 * count as f64 / self.max_entries as f64);
  }

  #[inline(always)]
  pub fn clear(&mut self) {
    self.table = vec![TTEntry::new(0, None, 0, TTFlag::None, Evaluation::Score(0), 0); self.max_entries]
  }
}
