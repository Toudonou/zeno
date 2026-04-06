use std::fmt::Display;
use rand::RngExt;

use crate::{bitboard::BitBoard, containers::ByColor};
use crate::containers::ByPieceType;
use crate::piece::{PieceColor, PieceType};

pub static START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub static RANK_1: BitBoard = 0x000000000000FF;
pub static RANK_2: BitBoard = 0x0000000000FF00;
pub static RANK_3: BitBoard = 0x00000000FF0000;
pub static RANK_4: BitBoard = 0x000000FF000000;
pub static RANK_5: BitBoard = 0x0000FF00000000;
pub static RANK_6: BitBoard = 0x00FF0000000000;
pub static RANK_7: BitBoard = 0xFF000000000000;
pub static FILE_A: BitBoard = 0x101010101010101;
pub static NOT_FILE_A: BitBoard = !FILE_A;
pub static NOT_FILE_H: BitBoard = !0x8080808080808080;
pub static BITBOARD_FILL_WITH_ONE: BitBoard = !0;

#[rustfmt::skip]
static WHITE_PAWNS_OCCUPANCY_OBLIGATION_FOR_EN_PASSANT: [BitBoard; 8] = [0x200000000, 0x500000000, 0xA00000000, 0x1400000000, 0x2800000000, 0x5000000000, 0xA000000000, 0x4000000000];
#[rustfmt::skip]
static BLACK_PAWNS_OCCUPANCY_OBLIGATION_FOR_EN_PASSANT: [BitBoard; 8] = [0x2000000, 0x5000000, 0xA000000, 0x14000000, 0x28000000, 0x50000000, 0xA0000000, 0x40000000];
#[rustfmt::skip]
pub static PAWNS_OCCUPANCY_OBLIGATION_FOR_EN_PASSANT: ByColor<[BitBoard; 8]> = ByColor::new(WHITE_PAWNS_OCCUPANCY_OBLIGATION_FOR_EN_PASSANT, BLACK_PAWNS_OCCUPANCY_OBLIGATION_FOR_EN_PASSANT);

pub static ZENO_INFINITY: i32 = 1_000_000_000;
pub static MAX_PLY: i32 = 128;
pub static MAX_TUNER_BUFFER_SIZE: usize = 10 * 1024 * 1024;

static PAWN_PHASE: i8 = 0;
static KNIGHT_PHASE: i8 = 1;
static BISHOP_PHASE: i8 = 1;
static ROOK_PHASE: i8 = 2;
static QUEEN_PHASE: i8 = 4;
static PHASE_TABLE: ByPieceType<i8> = ByPieceType::new(PAWN_PHASE, KNIGHT_PHASE, BISHOP_PHASE, ROOK_PHASE, QUEEN_PHASE, 0);
pub static TOTAL_PHASE: i8 = PAWN_PHASE * 16 + KNIGHT_PHASE * 4 + BISHOP_PHASE * 4 + ROOK_PHASE * 4 + QUEEN_PHASE * 2;

pub fn random_u64() -> u64 {
  // https://www.chessprogramming.org/index.php?title=Looking_for_Magics&oldid=2272
  let mut rng = rand::rng();
  let u1 = rng.random::<u64>() & 0xFFFF;
  let u2 = rng.random::<u64>() & 0xFFFF;
  let u3 = rng.random::<u64>() & 0xFFFF;
  let u4 = rng.random::<u64>() & 0xFFFF;
  u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
}


pub fn random_u64_few_bits() -> u64 {
  random_u64() & random_u64() & random_u64()
}

#[inline(always)]
pub fn get_phase(piece_type: PieceType) -> i8 {
  PHASE_TABLE[piece_type]
}

#[inline(always)]
pub fn get_psqt_index(side: PieceColor, square: u8) -> usize {
  match side {
    PieceColor::White => (square ^ 56) as usize,
    _ => square as usize,
  }
}

#[inline(always)]
pub fn sigmoid(k: f32, x: f32) -> f32 {
  1.0 / (1.0 + (-(k * x) / 400.0).exp())
}

#[inline(always)]
pub fn mse(a: f32, b: f32) -> f32 {
  (a - b) * (a - b)
}


pub fn print_array_as_board<T: Display>(array: [T; 64]) {
  for rank in 0..=7 {
    print!("{} ", 8 - rank);
    for file in 0..=7 {
      let index = rank * 8 + file;
      print!("{:+04} ", array[index]);
    }
    println!();
  }
  for i in [' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
    print!("{:+04} ", i);
  }
  println!();
}
