use std::fmt::{Display, Formatter};
use crate::moves::Move;

pub static MATE_SCORE: i32 = 1_000_000;


#[derive(Clone, Debug, Copy)]
pub enum Evaluation {
    Score(i32),
    MateIn(i32),
}

impl Evaluation {
    pub fn value(&self) -> i32 {
        match self {
            Evaluation::Score(score) => *score,
            Evaluation::MateIn(mate_in) => MATE_SCORE + 100 - (*mate_in).abs(),
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Evaluation::Score(score) => write!(f, "{}", score),
            Evaluation::MateIn(mate_in) => {
                if *mate_in > 0 { write!(f, "M{}", mate_in.abs() / 2) } else { write!(f, "-M{}", mate_in.abs() / 2) }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PosEval {
    pub best_move: Option<Move>,
    pub score: Evaluation,
    pub pv_line: Vec<Move>,
}