use crate::search::MAX_PLY;
use std::fmt::{Display, Formatter};
use std::ops::{Mul, MulAssign};

pub static MATE_SCORE: i32 = 1_000_000;

// The value of the Score(score) should be between [-MATE_SCORE / 2, MATE_SCORE / 2]
// The Mate(mate_in) value should be inside [-MAX_PLY, MAX_PLY]
// No verifications are done during the runtime to avoid performance issue due to many if statements
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Evaluation {
  Score(i32),
  MateIn(i32),
}

impl Evaluation {
  #[inline(always)]
  pub fn value(&self) -> i32 {
    match self {
      Evaluation::Score(score) => *score,

      // M(4): my side wins in four moves; M(-4): the opponent wins in four moves
      Evaluation::MateIn(mate_in) => (if *mate_in > 0 { 1 } else { -1 }) * (MATE_SCORE + MAX_PLY as i32 - (*mate_in).abs()),
    }
  }
}

impl Mul<i32> for Evaluation {
  type Output = Evaluation;

  fn mul(self, rhs: i32) -> Self::Output {
    match self {
      Evaluation::Score(score) => Evaluation::Score(score * rhs),
      Evaluation::MateIn(mate_in) => Evaluation::MateIn(mate_in * rhs),
    }
  }
}

impl MulAssign<i32> for Evaluation {
  fn mul_assign(&mut self, rhs: i32) {
    match self {
      Evaluation::Score(score) => *self = Evaluation::Score(*score * rhs),
      Evaluation::MateIn(mate_in) => *self = Evaluation::MateIn(*mate_in * rhs),
    }
  }
}

impl Display for Evaluation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Evaluation::Score(score) => write!(f, "{}", (*score as f32) / 100f32),
      Evaluation::MateIn(mate_in) => {
        if *mate_in > 0 {
          write!(f, "+M{}", mate_in.abs() / 2)
        } else {
          write!(f, "-M{}", mate_in.abs() / 2)
        }
      }
    }
  }
}
