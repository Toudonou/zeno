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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
