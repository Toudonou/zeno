use rand::Rng;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
#[repr(i16)]
pub enum PieceColor {
    None = 0,
    White = 1,
    Black = -1,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MoveType {
    Normal,
    ShortCastle,
    LongCastle,
    PawnToKnight,
    PawnToBishop,
    PawnToRook,
    PawnToQueen,
    EnPassant,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Move {
    pub source: i8,
    pub destination: i8,
    pub move_type: MoveType,
    pub move_score: i32,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct UndoMove {
    pub source: i8,
    pub destination: i8,
    pub move_type: MoveType,
    pub piece_moved: PieceType,
    pub piece_captured: PieceType,
    pub castling_rights: u8,
    pub turn: PieceColor,
    pub en_passant: Option<i8>,
}

pub fn count_set_bit(value: u64) -> u8 {
    let mut count = 0;
    let mut value = value;
    while value != 0 {
        count += 1;
        value &= value - 1;
    }
    count
}

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

