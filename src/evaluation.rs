use crate::position::Position;
use crate::utils::{PieceColor, PieceType};

static PAWN_VALUE: i32 = 100;
static KNIGHT_VALUE: i32 = 320;
static BISHOP_VALUE: i32 = 350;
static ROOK_VALUE: i32 = 500;
static QUEEN_VALUE: i32 = 900;
static KING_VALUE: i32 = 10000;
pub static MATE_SCORE: i32 = 1000_000;

#[inline(always)]
pub fn evaluate(position: &Position) -> i32 {
    material_evaluation(position)
}

#[inline(always)]
fn material_evaluation(position: &Position) -> i32 {
    PAWN_VALUE * (position.get_piece_count(&PieceColor::White, &PieceType::Pawn) - position.get_piece_count(&PieceColor::Black, &PieceType::Pawn)) +
        KNIGHT_VALUE * (position.get_piece_count(&PieceColor::White, &PieceType::Knight) - position.get_piece_count(&PieceColor::Black, &PieceType::Knight)) +
        BISHOP_VALUE * (position.get_piece_count(&PieceColor::White, &PieceType::Bishop) - position.get_piece_count(&PieceColor::Black, &PieceType::Bishop)) +
        ROOK_VALUE * (position.get_piece_count(&PieceColor::White, &PieceType::Rook) - position.get_piece_count(&PieceColor::Black, &PieceType::Rook)) +
        QUEEN_VALUE * (position.get_piece_count(&PieceColor::White, &PieceType::Queen) - position.get_piece_count(&PieceColor::Black, &PieceType::Queen))
}
