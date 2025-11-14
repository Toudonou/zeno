#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub enum PieceColor {
    White = 1,
    Black = -1,
    None = 0,
}

impl PieceColor {
    #[inline(always)]
    pub fn opposite(&self) -> Self {
        match self {
            PieceColor::None => PieceColor::None,
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }

    #[inline(always)]

    pub fn to_u8(&self) -> u8 {
        match self {
            PieceColor::White => 0,
            PieceColor::Black => 1,
            PieceColor::None => 2,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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