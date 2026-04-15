use crate::piece::PieceType;
use crate::square::{Square, SquareOps};

pub static PIECE_TYPES: [PieceType; 6] = [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King];
pub static SQUARES: [Square; Square::INVALID_SQUARE as usize] = {
  let mut squares = [0; Square::INVALID_SQUARE as usize];
  let mut i = 0;
  while i < Square::INVALID_SQUARE {
    squares[i as usize] = i;
    i += 1;
  }
  squares
};

static MAX_MATERIALS_FEATURES: usize = PIECE_TYPES.len();
static MAX_PSQT_FEATURES: usize = PIECE_TYPES.len() * SQUARES.len();
pub static MAX_FEATURES: usize = MAX_MATERIALS_FEATURES + MAX_PSQT_FEATURES + 1 /*(Isolated pawn penatly)*/;

pub static FEATURES_ALL: [Features; MAX_FEATURES] = {
  let mut features = [Features::Material(PieceType::Pawn); MAX_FEATURES];
  let mut index = 0;
  let mut p = 0;

  while p < PIECE_TYPES.len() {
    features[index] = Features::Material(PIECE_TYPES[p]);
    index += 1;
    p += 1;
  }

  let mut p = 0;
  while p < PIECE_TYPES.len() {
    let mut s = 0;
    while s < SQUARES.len() {
      features[index] = Features::Psqt(PIECE_TYPES[p], SQUARES[s]);
      index += 1;
      s += 1;
    }
    p += 1;
  }

  features
};

#[derive(Clone, Copy, Debug)]
pub enum Features {
  Material(PieceType),
  Psqt(PieceType, Square),
}

impl Features {
  #[inline(always)]
  pub fn to_index(self) -> usize {
    match self {
      Features::Material(piece_type) => piece_type as usize,
      Features::Psqt(piece_type, square) => PieceType::None as usize + Square::INVALID_SQUARE as usize * piece_type as usize + square as usize,
    }
  }
}
