use std::sync::atomic::{AtomicU64, Ordering};

use thousands::Separable;

use crate::evaluator::Score;
use crate::params::{EG_PAWN_VALUE, MG_PAWN_VALUE};
use crate::zobrist_hash::BoardHash;

/// We sure that abs(MG_PAWN_VALUE) and abs(EG_PAWN_VALUE) are less or equal than 500; so the pawn evaluation is always in `[`-32000, 32000`]`
const _: () = assert!(MG_PAWN_VALUE.abs() <= 500, "MG_PAWN_VALUE should be greater or equal to -500cp and less or equal to 500cp");
const _: () = assert!(EG_PAWN_VALUE.abs() <= 500, "EG_PAWN_VALUE should be greater or equal to -500cp and less or equal to 500cp");

/// PTEntry will store the evaluation of all the pawns for a given pawn structure identify that the pawn hash
///
/// Only a half key will be stored for the pawn hash.
///     - the first 4-byte will be use to calculate this index of the entry
///     - the last 4-byte will be stored in the table
///
/// For each phase the evaluation will be stored on 2-bytes
///     - As long as the value of a pawn in MG and EG is `[`-500cp, 500cp`]`, the evaluation of 64 pawns could fit in an i16; so 2-bytes should be enough
///     - The 2-bytes evaluation of each phase will be packed in a 4-bytes u32
///         - The first 2-bytes for the mg evaluation
///         - The last 2-bytes for the eg evaluation
#[derive(Debug, Clone, Copy)]
pub struct PTEntry {
  half_hash: u64,
  data: u64,
}

impl PTEntry {
  #[inline(always)]
  pub fn new(half_hash: u64, data: u64) -> PTEntry {
    PTEntry { half_hash, data }
  }

  #[inline(always)]
  pub fn get_half_hash(&self) -> u64 {
    self.half_hash
  }

  #[inline(always)]
  pub fn get_score(&self) -> Score {
    Score { mg: ((self.data >> 16) as u16) as i16 as i32, eg: self.data as u16 as i16 as i32 }
  }

  #[inline(always)]
  pub fn pack_score(score: Score) -> u64 {
    // Moving from i32 to i16 will not cause an error due to "owerflow"
    ((((score.mg as i16) as u16) as u64) << 16) | (((score.eg as i16) as u16) as u64)
  }
}

/// All the data regarding the PTEntry will be packed in an AtomicU64
///   - The first 4-bytes will store the u32 half hash
///   - The last 4-bytes will store u32 that contains the packed evaluations for the MG and the EG
///   - It is large enoungh and should be efficient
///
pub struct AtomicPTEntry {
  data: AtomicU64,
}

impl AtomicPTEntry {
  #[inline(always)]
  pub fn new(half_hash: u32, score: Score) -> AtomicPTEntry {
    AtomicPTEntry { data: AtomicPTEntry::pack_data(half_hash, score).into() }
  }

  #[inline(always)]
  pub fn get_pt_entry(&self) -> PTEntry {
    let data = self.data.load(Ordering::Relaxed);
    PTEntry::new((data >> 32) as u64, data as u64)
  }

  #[inline(always)]
  pub fn pack_data(half_hash: u32, score: Score) -> u64 {
    (half_hash as u64) << 32 | PTEntry::pack_score(score) as u64
  }
}

pub struct PawnTable {
  table: Vec<PTEntry>,
  max_entries: usize,
  // lookup: usize,
  // hit: usize,
}

impl PawnTable {
  #[inline(always)]
  pub fn default() -> PawnTable {
    Self::with_capacity(128)
  }

  #[inline(always)]
  pub fn with_capacity(size_kb: usize) -> PawnTable {
    let max_entries = ((size_kb * 1024) / size_of::<PTEntry>()).next_power_of_two().max(1);

    // let mut table = Vec::with_capacity(max_entries);
    // table.resize_with(max_entries, || AtomicPTEntry::new(0, Score { mg: 0, eg: 0 }));

    PawnTable { table: vec![PTEntry::new(0, 0); max_entries], max_entries }
  }

  #[inline(always)]
  pub fn get_entry(&mut self, pawn_hash: BoardHash) -> Option<Score> {
    let (index_hash, half_hash) = Self::slice_full_hash(pawn_hash);
    let entry = self.table[self.index(pawn_hash)]; //.get_pt_entry();

    // self.lookup.fetch_add(1, Ordering::Relaxed);
    // self.lookup += 1;
    if pawn_hash == entry.half_hash {
      // self.hit.fetch_add(1, Ordering::Relaxed);
      // self.hit += 1;
      Some(entry.get_score())
    } else {
      None
    }
  }

  #[inline(always)]
  pub fn save_entry(&mut self, pawn_hash: BoardHash, score: Score) {
    // let (index_hash, half_hash) = Self::slice_full_hash(pawn_hash);

    // let entry = &self.table[self.index(index_hash)];
    // entry.data.store(AtomicPTEntry::pack_data(half_hash, score), Ordering::Relaxed);

    let index = self.index(pawn_hash);
    self.table[index] = PTEntry::new(pawn_hash, PTEntry::pack_score(score))
  }

  #[inline(always)]
  pub fn print_stats(&self) {
    // println!();
    // println!("Max entries = {}", self.max_entries.separate_with_commas());
    // println!("Lookup = {}", self.lookup.separate_with_commas());
    // println!("Hit = {}", self.hit.separate_with_commas());
    // println!("Hit Rate = {:.2}%", 100.0 * self.hit as f32 / self.lookup as f32);
    // println!();
  }

  #[inline(always)]
  fn index(&self, index_hash: u64) -> usize {
    index_hash as usize & (self.max_entries - 1) // max_entries is a power of 2, therefore (x % max_entries) == x & (max_entries - 1)
  }

  /// Slice the full hash into two piece
  ///   - The first one will be used as index in the table
  ///   - The second one will be stored in the table as a key
  #[inline(always)]
  fn slice_full_hash(full_hash: BoardHash) -> (u32, u32) {
    ((full_hash >> 32) as u32, full_hash as u32)
  }
}
