use crate::piece::Piece;

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

impl From<u32> for MoveType {
    #[inline(always)]
    fn from(value: u32) -> Self {
        match value {
            0 => MoveType::Normal,
            1 => MoveType::ShortCastle,
            2 => MoveType::LongCastle,
            3 => MoveType::PawnToKnight,
            4 => MoveType::PawnToBishop,
            5 => MoveType::PawnToRook,
            6 => MoveType::PawnToQueen,
            7 => MoveType::EnPassant,
            _ => panic!("Invalid MoveType value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move(u32);

impl Move {
    /// Pack source, destination, move_type, source piece and destination piece into a 32-bit value:
    /// [ destination_piece:4 | source_piece:4 | move_type:3 | destination:6 | source:6 ]
    #[inline(always)]
    pub fn new(source: u8, destination: u8, move_type: MoveType, source_piece: &Piece, destination_piece: &Piece) -> Self {
        Self((destination_piece.to_u32_into() << 19) | (source_piece.to_u32_into() << 15) | ((move_type as u32) << 12) | ((destination as u32) << 6) | (source as u32))
    }

    #[inline(always)]
    pub fn source(&self) -> u8 { (self.0 & 0b111111) as u8 }

    #[inline(always)]
    pub fn destination(&self) -> u8 { ((self.0 >> 6) & 0b111111) as u8 }

    #[inline(always)]
    pub fn move_type(&self) -> MoveType { MoveType::from((self.0 >> 12) & 0b111) }

    #[inline(always)]
    pub fn get_source_piece(&self) -> Piece { Piece::from_u32((self.0 >> 15) & 0b1111) }

    #[inline(always)]
    pub fn get_destination_piece(&self) -> Piece { Piece::from_u32((self.0 >> 19) & 0b1111) }

    #[inline(always)]
    pub fn to_uci_string(&self) -> String {
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let ranks = [1, 2, 3, 4, 5, 6, 7, 8];

        files[(self.source() % 8) as usize].to_string() + &ranks[(self.source() / 8) as usize].to_string() +
            &files[(self.destination() % 8) as usize].to_string() + &ranks[(self.destination() / 8) as usize].to_string() +
            match self.move_type() {
                MoveType::PawnToKnight => "n",
                MoveType::PawnToBishop => "b",
                MoveType::PawnToRook => "r",
                MoveType::PawnToQueen => "q",
                _ => ""
            }
    }
}