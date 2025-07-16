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
