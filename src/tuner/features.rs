use std::collections::HashMap;
use std::sync::LazyLock;

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

pub const MAX_FEATURES: usize = /* Material */
  PIECE_TYPES.len() + /* PSQT */ PIECE_TYPES.len() * SQUARES.len() + /* Bishop Pair */ 1 + /* Doubled Pawns */ 8 + /* Passed Pawn */ 8 + /* Isolated Pawn */ 8 + /* Backward Pawns */ 8 +  /* Connected Pawns */ 8;

pub static FEATURES_ALL: LazyLock<Vec<Features>> = LazyLock::new(|| {
  let mut features = Vec::with_capacity(MAX_FEATURES);

  for piece in PIECE_TYPES {
    features.push(Features::Material(piece));
  }

  for piece in PIECE_TYPES {
    for square in SQUARES {
      features.push(Features::Psqt(piece, square));
    }
  }

  features.push(Features::BishopPair);

  for index in 0..8 {
    features.push(Features::DoubledPawns(index as u8));
    features.push(Features::PassedPawns(index as u8));
    features.push(Features::IsolatedPawns(index as u8));
    features.push(Features::BackwardPawns(index as u8));
    features.push(Features::ConnectedPawns(index as u8));
  }

  features
});

static FEATURES_TO_INDEX: LazyLock<HashMap<Features, usize>> = LazyLock::new(|| {
  let mut map = HashMap::new();

  FEATURES_ALL.iter().enumerate().for_each(|(i, feature)| {
    map.insert(*feature, i);
  });

  map
});

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Features {
  Material(PieceType),
  Psqt(PieceType, Square),
  BishopPair,
  DoubledPawns(u8),
  PassedPawns(u8),
  IsolatedPawns(u8),
  BackwardPawns(u8),
  ConnectedPawns(u8),
}

impl Features {
  #[inline(always)]
  pub fn to_index(self) -> usize {
    FEATURES_TO_INDEX[&self]
  }
}
