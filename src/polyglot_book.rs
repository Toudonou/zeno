use std::collections::HashMap;
use std::fs;
use std::path::Path;
use rand::Rng;
use crate::moves::{Move, MoveType};
use crate::moves::MoveType::Normal;
use crate::utils::random_u64;

#[derive(Debug)]
struct PolyglotEntry {
    zobrist_hash: u64,
    mov: u16,
    weight: u16,
    learn: u32,
}

pub struct PolyglotBook {
    entries: HashMap<u64, Vec<u16>>,
}

impl PolyglotBook {
    pub fn new(opening_book: &str) -> Self {
        let data = fs::read(opening_book);
        let mut entries: HashMap<u64, Vec<u16>> = HashMap::new();

        match data {
            Ok(data) => {
                entries = HashMap::with_capacity(data.len() / size_of::<PolyglotEntry>());

                for chunk in data.chunks_exact(16) {
                    // Should be big-endian
                    let zobrist_hash = u64::from_be_bytes(chunk[0..8].try_into().unwrap());
                    let mov = u16::from_be_bytes(chunk[8..10].try_into().unwrap());
                    let weight = u16::from_be_bytes(chunk[10..12].try_into().unwrap());
                    let learn = u32::from_be_bytes(chunk[12..16].try_into().unwrap());

                    if entries.contains_key(&zobrist_hash) {
                        entries.get_mut(&zobrist_hash).unwrap().push(mov);
                    } else {
                        entries.insert(zobrist_hash, vec![mov]);
                    }
                }

                println!("{} book move load successfully", Path::new(opening_book).file_name().unwrap().to_str().unwrap().to_owned())
            }
            Err(e) => {
                println!("No book move load");
            }
        }

        PolyglotBook { entries }
    }

    pub fn get_book_move(&self, zobrist_hash: &u64) -> Option<Move> {
        match self.entries.get(zobrist_hash) {
            None => {}
            Some(moves) => {
                let mut rng = rand::rng();
                let idx = rng.random::<u64>() as usize;

                let mov = moves[idx % moves.len()];

                let destination_file = mov & 0b111;
                let destination_rank = (mov >> 3) & 0b111;

                let source_file = (mov >> 6) & 0b111;
                let source_rank = (mov >> 9) & 0b111;

                let promotion = (mov >> 12) & 0b111;
                let move_type = match promotion {
                    0 => Normal,
                    1 => MoveType::PawnToKnight,
                    2 => MoveType::PawnToBishop,
                    3 => MoveType::PawnToRook,
                    4 => MoveType::PawnToQueen,
                    _ => panic!("Error reading promotion"),
                };

                let source = (source_rank * 8) as u8 + source_file as u8;
                let destination = (destination_rank * 8) as u8 + destination_file as u8;

                return Some(Move::new(source, destination, move_type));
            }
        }

        None
    }
}