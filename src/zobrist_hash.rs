use crate::utils::{Coord, Piece, PieceColor, PieceType};
use std::collections::HashMap;
use rand::Rng;
use crate::position::Position;

pub struct ZobristHash {
    keys_tables: HashMap<Piece, Vec<u64>>,
    hashs: HashMap<u64, f64>
}

impl ZobristHash {
    pub fn new() -> ZobristHash{
        let mut keys_tables: HashMap<Piece, Vec<u64>> = HashMap::new();
        let hashs: HashMap<u64, f64> = HashMap::new();

        keys_tables.insert(Piece{color:PieceColor::None, piece_type:  PieceType::None}, Vec::new());

        keys_tables.insert(Piece{color:PieceColor::White, piece_type:  PieceType::  Pawn}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::White, piece_type:  PieceType::  Knight}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::White, piece_type:  PieceType::  Bishop}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::White, piece_type:  PieceType::  Rook}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::White, piece_type:  PieceType::  Queen}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::White, piece_type:  PieceType::  King}, Vec::new());

        keys_tables.insert(Piece{color:PieceColor::Black, piece_type:  PieceType::  Pawn}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::Black, piece_type:  PieceType::  Knight}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::Black, piece_type:  PieceType::  Bishop}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::Black, piece_type:  PieceType::  Rook}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::Black, piece_type:  PieceType::  Queen}, Vec::new());
        keys_tables.insert(Piece{color:PieceColor::Black, piece_type:  PieceType::  King}, Vec::new());

        let mut rng = rand::rng();
        for piece_board in &mut keys_tables {
            for _ in 0..64{
                piece_board.1.push(rng.random::<u64>());
            }
        }

        ZobristHash{
            keys_tables,
            hashs
        }
    }

    pub fn hash_position(&self, position:& Position) -> u64{
        let mut hash: u64 = 0;

        for rank in 1..=8{
            for file in 'a' ..='h'{
                let piece = position.get_piece_on_square(&Coord{rank, file});
                let i = 8 - rank;
                let j = file as u8 - 'a' as u8;
                let piece_hash = self.keys_tables.get(&piece).unwrap()[(8 * i as u8 + j) as usize];
                hash = hash ^ piece_hash;
            }
        }

        hash
    }

    pub fn contains_position(&self, position: &Position) -> bool{
        self.hashs.contains_key(&self.hash_position(position))
    }

    pub fn get_position_evaluation(&self, position: &Position) -> f64{
        let position_hash = self.hash_position(position);
        *self.hashs.get(&position_hash).unwrap()
    }

    pub fn insert_position(&mut self, position: &Position, evaluation: &f64){
        let position_hash = self.hash_position(position);
        if self.hashs.contains_key(&position_hash){
            return;
        }
        self.hashs.insert(position_hash, *evaluation);
    }
}
