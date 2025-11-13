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
    pub fn to_u32_into(&self) -> u32 {
        match self {
            Piece { color: PieceColor::None, piece_type: PieceType::None } => 0,
            Piece { color: PieceColor::White, piece_type: PieceType::Pawn } => 1,
            Piece { color: PieceColor::White, piece_type: PieceType::Knight } => 2,
            Piece { color: PieceColor::White, piece_type: PieceType::Bishop } => 3,
            Piece { color: PieceColor::White, piece_type: PieceType::Rook } => 4,
            Piece { color: PieceColor::White, piece_type: PieceType::Queen } => 5,
            Piece { color: PieceColor::White, piece_type: PieceType::King } => 6,

            Piece { color: PieceColor::Black, piece_type: PieceType::Pawn } => 7,
            Piece { color: PieceColor::Black, piece_type: PieceType::Knight } => 8,
            Piece { color: PieceColor::Black, piece_type: PieceType::Bishop } => 9,
            Piece { color: PieceColor::Black, piece_type: PieceType::Rook } => 10,
            Piece { color: PieceColor::Black, piece_type: PieceType::Queen } => 11,
            Piece { color: PieceColor::Black, piece_type: PieceType::King } => 12,

            _ => panic!("Invalid piece")
        }
    }
    
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Piece { color: PieceColor::None, piece_type: PieceType::None },
            1 => Piece { color: PieceColor::White, piece_type: PieceType::Pawn },
            2 => Piece { color: PieceColor::White, piece_type: PieceType::Knight },
            3 => Piece { color: PieceColor::White, piece_type: PieceType::Bishop },
            4 => Piece { color: PieceColor::White, piece_type: PieceType::Rook },
            5 => Piece { color: PieceColor::White, piece_type: PieceType::Queen },
            6 => Piece { color: PieceColor::White, piece_type: PieceType::King },

            7 => Piece { color: PieceColor::Black, piece_type: PieceType::Pawn },
            8 => Piece { color: PieceColor::Black, piece_type: PieceType::Knight },
            9 => Piece { color: PieceColor::Black, piece_type: PieceType::Bishop },
            10 => Piece { color: PieceColor::Black, piece_type: PieceType::Rook },
            11 => Piece { color: PieceColor::Black, piece_type: PieceType::Queen },
            12 => Piece { color: PieceColor::Black, piece_type: PieceType::King },

            _ => panic!("Invalid piece")
        }
    }
}