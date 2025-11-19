use crate::pos_eval::PosEval;

pub static ZENO_TRANSPOSITION_TABLE_SIZE: usize = 256 * 1024 * 1024; // 256MB


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone)]
pub struct TTEntry {
    pub flag: TTFlag,
    pub pos_eval: PosEval,
    pub hash: u64,
    pub depth: i32,
}

#[derive(Clone)]
pub struct TranspositionTable {
    table: Vec<Option<TTEntry>>,
    max_entries: usize,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        let max_entries = ZENO_TRANSPOSITION_TABLE_SIZE / size_of::<Option<TTEntry>>();
        let mut vec: Vec<Option<TTEntry>> = Vec::with_capacity(max_entries);
        for _ in 0..max_entries { vec.push(None); }

        TranspositionTable { table: vec, max_entries }
    }

    pub fn get_entry(&self, hash: u64) -> Option<TTEntry> {
        self.table[hash as usize % self.max_entries].clone()
    }

    pub fn add_entry(&mut self, entry: TTEntry) {
        self.table[entry.hash as usize % self.max_entries] = Some(entry.clone());
    }

    pub fn print_transposition_stats(&self) {
        let count = self.table.iter().filter(|x| x.is_some()).count();
        println!("Transposition utilization: {}/{} = {}%", count, self.max_entries, 100 * count / self.max_entries);
    }
}