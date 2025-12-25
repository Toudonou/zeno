use crate::containers::ByColor;

static OPPOSITE_COLOR_TABLE: ByColor<PieceColor> = ByColor::new(PieceColor::Black, PieceColor::White);
static SIGN_TABLE: ByColor<i32> = ByColor::new(1, -1);

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub enum PieceColor {
  Black = 0,
  White = 1,
  None = 2,
}

impl PieceColor {
  #[inline(always)]
  pub fn opposite(self) -> Self {
    // PieceColor::None.opposite() should never be called
    OPPOSITE_COLOR_TABLE[self]
  }

  #[inline(always)]
  pub fn to_i32(self) -> i32 {
    // PieceColor::None.to_i32() should never be called
    SIGN_TABLE[self]
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub enum PieceType {
  Pawn = 0,
  Knight = 1,
  Bishop = 2,
  Rook = 3,
  Queen = 4,
  King = 5,
  None = 6,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub struct Piece {
  pub color: PieceColor,
  pub piece_type: PieceType,
}

impl Piece {
  #[inline(always)]
  pub fn to_usize(&self) -> usize {
    match (self.color, self.piece_type) {
      (PieceColor::White, PieceType::Pawn) => 0,
      (PieceColor::White, PieceType::Knight) => 1,
      (PieceColor::White, PieceType::Bishop) => 2,
      (PieceColor::White, PieceType::Rook) => 3,
      (PieceColor::White, PieceType::Queen) => 4,
      (PieceColor::White, PieceType::King) => 5,

      (PieceColor::Black, PieceType::Pawn) => 6,
      (PieceColor::Black, PieceType::Knight) => 7,
      (PieceColor::Black, PieceType::Bishop) => 8,
      (PieceColor::Black, PieceType::Rook) => 9,
      (PieceColor::Black, PieceType::Queen) => 10,
      (PieceColor::Black, PieceType::King) => 11,

      (PieceColor::None, PieceType::None) => 12,
      _ => panic!("Invalid piece"),
    }
  }
}
