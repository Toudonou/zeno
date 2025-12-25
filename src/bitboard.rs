#[macro_export]
macro_rules! get_lsb {
  ($bitboard:expr) => {
    ($bitboard).trailing_zeros() as Square
  };
}

#[macro_export]
macro_rules! pop_lsb {
  ($bitboard:expr) => {
    $bitboard &= $bitboard - 1;
  };
}

#[macro_export]
macro_rules! shift {
  ($bitboard:expr, $amount:expr) => {
    if $amount > 0 {
      $bitboard << $amount as u8
    } else {
      $bitboard >> (-$amount) as u8
    }
  };
}

pub type BitBoard = u64;

pub trait BitBoardOps {
  fn print(&self);
}

impl BitBoardOps for BitBoard {
  fn print(&self) {
    println!();
    for rank in (0..=7).rev() {
      print!("{} ", rank + 1);
      for file in 0..=7 {
        let index = rank * 8 + file as u64;
        if (self & (1u64 << index)) != 0 {
          print!("x ");
        } else {
          print!(". ");
        }
      }
      println!();
    }
    println!("\n  a b c d e f g h\n");
  }
}
