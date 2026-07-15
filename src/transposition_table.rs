use std::sync::atomic::{AtomicU64, Ordering};
use thousands::Separable;

use crate::moves::Move;
use crate::pos_eval::Evaluation;
use crate::zobrist_hash::BoardHash;

pub static DEFAULT_TRANSPOSITION_SIZE: u32 = 1; // 1 MB
pub static MIN_TRANSPOSITION_SIZE: u32 = 1; // 1 MB
pub static MAX_TRANSPOSITION_SIZE: u32 = 256 * 1024; // 256 GB

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
pub struct TTEntry {
  hash: BoardHash,
  data: u64,
}
impl TTEntry {
  #[inline(always)]
  pub fn new(hash: BoardHash, data: u64) -> TTEntry {
    TTEntry { hash, data }
  }

  #[inline(always)]
  pub fn get_hash(&self) -> BoardHash {
    self.hash
  }

  #[inline(always)]
  pub fn get_evaluation(&self, ply: u32) -> Evaluation {
    let eval = Evaluation::from_u32(((self.data >> (4 * 8)) & 0xFFFFFFFF) as u32);

    match eval {
      Evaluation::CentiPawns(_) => eval,
      Evaluation::MateIn(mate_in) => Evaluation::MateIn(mate_in.signum() * (mate_in.abs() + ply as i32)),
    }
  }

  #[inline(always)]
  pub fn get_best_move(&self) -> Option<Move> {
    let move_code = ((self.data >> (2 * 8)) & 0xFFFF) as u16;
    if move_code != 0 { Some(Move::from_u16(move_code)) } else { None }
  }

  #[inline(always)]
  pub fn get_depth(&self) -> u32 {
    ((self.data >> (1 * 8)) & 0xFF) as u32
  }

  #[inline(always)]
  pub fn get_flag(&self) -> TTFlag {
    TTFlag::from_u32(((self.data) & 0xFF) as u32)
  }
}

pub struct AtomicTTEntry {
  hash: AtomicU64,
  data: AtomicU64,
}

impl AtomicTTEntry {
  #[inline(always)]
  pub fn new(hash: BoardHash, best_move: Option<Move>, depth: u32, flag: TTFlag, evaluation: Evaluation, ply: u32) -> AtomicTTEntry {
    AtomicTTEntry {
      hash: hash.into(),
      data: AtomicTTEntry::pack_data(best_move, depth, flag, evaluation, ply).into(),
    }
  }

  #[inline(always)]
  pub fn get_tt_entry(&self) -> TTEntry {
    TTEntry {
      hash: self.hash.load(Ordering::Relaxed),
      data: self.data.load(Ordering::Relaxed),
    }
  }

  #[inline(always)]
  pub fn pack_data(best_move: Option<Move>, depth: u32, flag: TTFlag, evaluation: Evaluation, ply: u32) -> u64 {
    let best_move = if let Some(m) = best_move { m.to_u16() } else { 0 } as u64;
    let flag = flag.to_u32() as u64;
    let depth = depth as u64;
    let evaluation = match evaluation {
      Evaluation::CentiPawns(_) => evaluation.to_u32() as u64,
      Evaluation::MateIn(mate_in) => Evaluation::MateIn(mate_in.signum() * (mate_in.abs() - ply as i32)).to_u32() as u64,
    };

    (evaluation << (4 * 8)) | (best_move << (2 * 8)) | (depth << (1 * 8)) | (flag)
  }
}

pub struct TranspositionTable {
  table: Vec<AtomicTTEntry>,
  max_entries: usize,
}

impl TranspositionTable {
  #[inline(always)]
  pub fn default() -> TranspositionTable {
    Self::with_capacity(DEFAULT_TRANSPOSITION_SIZE)
  }

  #[inline(always)]
  pub fn with_capacity(tt_size_mb: u32) -> TranspositionTable {
    let tt_size_mb = tt_size_mb.clamp(MIN_TRANSPOSITION_SIZE, MAX_TRANSPOSITION_SIZE);
    let max_entries = ((tt_size_mb * 1024 * 1024) as usize / size_of::<AtomicTTEntry>()).next_power_of_two();

    let mut table = Vec::with_capacity(max_entries);
    table.resize_with(max_entries, || AtomicTTEntry::new(0, None, 0, TTFlag::None, Evaluation::CentiPawns(0), 0));

    TranspositionTable { table, max_entries }
  }

  pub fn index(&self, hash: BoardHash) -> usize {
    hash as usize & (self.max_entries - 1) // max_entries is a power of 2, therefore (x % max_entries) == x & (max_entries)
  }

  #[inline(always)]
  pub fn get_entry(&self, hash: BoardHash) -> TTEntry {
    self.table[self.index(hash)].get_tt_entry()
  }

  #[inline(always)]
  pub fn save_entry(&self, hash: BoardHash, best_move: Option<Move>, depth: u32, flag: TTFlag, evaluation: Evaluation, ply: u32) {
    let entry = &self.table[self.index(hash)]; // max_entries is a power of 2, therefore (x % max_entries) == x & (max_entries)
    entry.hash.store(hash, Ordering::Relaxed);
    entry
      .data
      .store(AtomicTTEntry::pack_data(best_move, depth, flag, evaluation, ply).into(), Ordering::Relaxed);
  }

  pub fn print_transposition_stats(&self) {
    let count = self.table.iter().filter(|x| x.get_tt_entry().get_flag() != TTFlag::None).count();
    println!(
      "Transposition utilization: {}/{} = {:.3}%",
      count.separate_with_commas(),
      self.max_entries.separate_with_commas(),
      100f64 * count as f64 / self.max_entries as f64
    );
  }

  #[inline(always)]
  pub fn clear(&self) {
    for entry in &self.table {
      entry.hash.store(0, Ordering::Relaxed);
      entry.data.store(0, Ordering::Relaxed);
    }
  }
}
