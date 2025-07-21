use std::sync::LazyLock;

pub static RANK_1: LazyLock<u64> =
    LazyLock::new(|| 0b0000000000000000000000000000000000000000000000000000000011111111);
pub static FILE_A: LazyLock<u64> =
    LazyLock::new(|| 0b0000000100000001000000010000000100000001000000010000000100000001);
pub static ANTI_DIAGONAL: LazyLock<u64> =
    LazyLock::new(|| 0b1000000001000000001000000001000000001000000001000000001000000001);
pub static DIAGONAL: LazyLock<u64> =
    LazyLock::new(|| 0b0000000100000010000001000000100000010000001000000100000010000000);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(i16)]
pub enum PieceColor {
    None = 0,
    White = 1,
    Black = -1,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(i16)]
pub enum PieceType {
    None = 0,
    Pawn = 100,
    Knight = 320,
    Bishop = 330,
    Rook = 500,
    Queen = 900,
    King = 20000,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub rank: i8,
    pub file: char,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MoveType {
    Normal,
    ShortCastle,
    LongCastle,
    PawnToKnight,
    PawnToBishop,
    PawnToRook,
    PawnToQueen,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub source: Coord,
    pub destination: Coord,
    pub move_type: MoveType,
}
