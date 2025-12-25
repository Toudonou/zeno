use crate::bitboard::BitBoard;
use rand::Rng;
use std::fmt::Display;

pub static START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub static RANK_2: BitBoard = 0x0000000000FF00;
pub static RANK_3: BitBoard = 0x00000000FF0000;
pub static RANK_4: BitBoard = 0x000000FF000000;
pub static RANK_5: BitBoard = 0x0000FF00000000;
pub static RANK_6: BitBoard = 0x00FF0000000000;
pub static RANK_7: BitBoard = 0xFF000000000000;
pub static NOT_FILE_A: BitBoard = !0x101010101010101;
pub static NOT_FILE_H: BitBoard = !0x8080808080808080;
pub static BITBOARD_FILL_WITH_ONE: BitBoard = !0;

pub static ZENO_INFINITY: i32 = 1_000_000_000;

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

pub fn print_array_as_board<T: Display>(array: [T; 64]) {
  println!();
  for rank in (0..=7).rev() {
    print!("{} ", rank + 1);
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
