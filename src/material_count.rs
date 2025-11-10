use crate::utils::{PieceColor, PieceType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialCount(u64);

impl MaterialCount {
    /// Pack the number of each type of piece into a 64-bit value:
    /// Each type of pieces can be at most 62 on a chess board
    /// Number of kings is irrelevant
    /// [ black_queen:6 | black_rook:6 | black_bishop:6 | black_knight:6 | black_pawn:6 | white_queen:6 | white_rook:6 | white_bishop:6 | white_knight:6 | white_pawn:6 ]

    #[inline(always)]
    pub fn new() -> Self { Self(0) }

    #[inline(always)]
    pub fn add_piece(&mut self, piece_color: &PieceColor, piece_type: &PieceType) {
        match (piece_color, piece_type) {
            (PieceColor::White, PieceType::Pawn) => self.0 += 1,
            (PieceColor::White, PieceType::Knight) => self.0 += 1 << 6,
            (PieceColor::White, PieceType::Bishop) => self.0 += 1 << 12,
            (PieceColor::White, PieceType::Rook) => self.0 += 1 << 18,
            (PieceColor::White, PieceType::Queen) => self.0 += 1 << 24,

            (PieceColor::Black, PieceType::Pawn) => self.0 += 1 << 30,
            (PieceColor::Black, PieceType::Knight) => self.0 += 1 << 36,
            (PieceColor::Black, PieceType::Bishop) => self.0 += 1 << 42,
            (PieceColor::Black, PieceType::Rook) => self.0 += 1 << 48,
            (PieceColor::Black, PieceType::Queen) => self.0 += 1 << 54,

            _ => {}
        }
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, piece_color: &PieceColor, piece_type: &PieceType) {
        match (piece_color, piece_type) {
            (PieceColor::White, PieceType::Pawn) => self.0 -= 1,
            (PieceColor::White, PieceType::Knight) => self.0 -= 1 << 6,
            (PieceColor::White, PieceType::Bishop) => self.0 -= 1 << 12,
            (PieceColor::White, PieceType::Rook) => self.0 -= 1 << 18,
            (PieceColor::White, PieceType::Queen) => self.0 -= 1 << 24,

            (PieceColor::Black, PieceType::Pawn) => self.0 -= 1 << 30,
            (PieceColor::Black, PieceType::Knight) => self.0 -= 1 << 36,
            (PieceColor::Black, PieceType::Bishop) => self.0 -= 1 << 42,
            (PieceColor::Black, PieceType::Rook) => self.0 -= 1 << 48,
            (PieceColor::Black, PieceType::Queen) => self.0 -= 1 << 54,

            _ => {}
        }
    }

    #[inline(always)]
    pub fn get_piece_count(&self, piece_color: &PieceColor, piece_type: &PieceType) -> i32 {
        match (piece_color, piece_type) {
            (PieceColor::White, PieceType::Pawn) => (self.0 & 0b111111) as i32,
            (PieceColor::White, PieceType::Knight) => ((self.0 >> 6) & 0b111111) as i32,
            (PieceColor::White, PieceType::Bishop) => ((self.0 >> 12) & 0b111111) as i32,
            (PieceColor::White, PieceType::Rook) => ((self.0 >> 18) & 0b111111) as i32,
            (PieceColor::White, PieceType::Queen) => ((self.0 >> 24) & 0b111111) as i32,

            (PieceColor::Black, PieceType::Pawn) => ((self.0 >> 30) & 0b111111) as i32,
            (PieceColor::Black, PieceType::Knight) => ((self.0 >> 36) & 0b111111) as i32,
            (PieceColor::Black, PieceType::Bishop) => ((self.0 >> 42) & 0b111111) as i32,
            (PieceColor::Black, PieceType::Rook) => ((self.0 >> 48) & 0b111111) as i32,
            (PieceColor::Black, PieceType::Queen) => ((self.0 >> 54) & 0b111111) as i32,

            _ => 0
        }
    }

    #[inline(always)]
    pub fn print(&self) {
        println!("Number of whites pawns: {}", self.0 & 0b111111);
        println!("Number of whites knights: {}", (self.0 >> 6) & 0b111111);
        println!("Number of whites bishops: {}", (self.0 >> 12) & 0b111111);
        println!("Number of whites rooks: {}", (self.0 >> 18) & 0b111111);
        println!("Number of whites queens: {}", (self.0 >> 24) & 0b111111);
        println!();

        println!("Number of blacks pawns: {}", (self.0 >> 30) & 0b111111);
        println!("Number of blacks knights: {}", (self.0 >> 36) & 0b111111);
        println!("Number of blacks bishops: {}", (self.0 >> 42) & 0b111111);
        println!("Number of blacks rooks: {}", (self.0 >> 48) & 0b111111);
        println!("Number of blacks queens: {}", (self.0 >> 54) & 0b111111);
        println!();
    }
}