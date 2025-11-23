use thousands::Separable;
use crate::moves::Move;
use crate::pos_eval::Evaluation;

pub static ZENO_TRANSPOSITION_TABLE_SIZE: usize = 64 * 1024 * 1024; // 64MB

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

// The informations about the Entry will be stored in a 64-bits value (8 bytes); the TTEntry will be 16-bytes overall
// - 1 byte: for the flag
// - 1 byte for the depth
// - 2 bytes: for the move
// - 4 bytes for the evaluation
#[derive(Clone, Copy)]
pub struct TTEntry {
    hash: u64,
    others_informations: u64,
}

impl TTEntry {
    #[inline(always)]
    pub fn new(hash: u64, best_move: &Option<Move>, depth: u32, flag: &TTFlag, evaluation: &Evaluation) -> TTEntry {
        let best_move = if let Some(m) = best_move { m.to_u16() } else { 0 } as u64;
        let flag = flag.to_u32() as u64;
        let evaluation = evaluation.to_u32() as u64;
        let depth = depth as u64;

        TTEntry { hash, others_informations: (evaluation << (4 * 8)) | (best_move << (2 * 8)) | (depth << (1 * 8)) | (flag) }
    }

    #[inline(always)]
    pub fn get_hash(&self) -> u64 { self.hash }

    #[inline(always)]
    pub fn get_evaluation(&self) -> Evaluation { Evaluation::from_u32(((self.others_informations >> (4 * 8)) & 0xFFFFFFFF) as u32) }

    #[inline(always)]
    pub fn get_best_move(&self) -> Option<Move> {
        let move_code = ((self.others_informations >> (2 * 8)) & 0xFFFF) as u16;
        if move_code != 0 { Some(Move::from_u16(move_code)) } else { None }
    }

    #[inline(always)]
    pub fn get_depth(&self) -> u32 { ((self.others_informations >> (1 * 8)) & 0xFF) as u32 }

    #[inline(always)]
    pub fn get_flag(&self) -> TTFlag { TTFlag::from_u32(((self.others_informations) & 0xFF) as u32) }
}

#[derive(Clone)]
pub struct TranspositionTable {
    table: Vec<TTEntry>,
    max_entries: usize,
}

impl TranspositionTable {
    #[inline(always)]
    pub fn new() -> TranspositionTable {
        let max_entries = ZENO_TRANSPOSITION_TABLE_SIZE / size_of::<TTEntry>();
        TranspositionTable { table: vec![TTEntry::new(0, &None, 0, &TTFlag::None, &Evaluation::Score(0)); ZENO_TRANSPOSITION_TABLE_SIZE / size_of::<TTEntry>()], max_entries }
    }

    #[inline(always)]
    pub fn get_entry(&self, hash: u64) -> TTEntry { self.table[hash as usize % self.max_entries] }

    #[inline(always)]
    pub fn add_entry(&mut self, entry: TTEntry) {
        let hash = entry.hash;
        self.table[hash as usize % self.max_entries] = entry;
    }

    pub fn print_transposition_stats(&self) {
        let count = self.table.iter().filter(|x| x.get_flag() != TTFlag::None).count();
        println!("Transposition utilization: {}/{} = {:.3}%", count.separate_with_commas(), self.max_entries.separate_with_commas(), 100f64 * count as f64 / self.max_entries as f64);
    }

    #[inline(always)]
    pub fn clear(&mut self) { self.table = vec![TTEntry::new(0, &None, 0, &TTFlag::None, &Evaluation::Score(0)); ZENO_TRANSPOSITION_TABLE_SIZE / size_of::<TTEntry>()] }
}
