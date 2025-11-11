use crate::evaluation::PIECES_VALUES;
use crate::moves::{Move, MoveType};
use crate::position::Position;
use crate::utils::PieceType;

static PROMOTION_SCORE: i32 = 100_000;

#[inline(always)]
pub fn order_moves(moves: &mut Vec<Move>, position: &Position) {
    moves.sort_by(|a, b| evaluate_move(b, position).cmp(&evaluate_move(a, position)));
}

#[inline(always)]
fn evaluate_move(mov: &Move, position: &Position) -> i32 {
    let mut score = 0;
    let source_piece_type = position.get_piece_on_square(&mov.source()).piece_type;
    let destination_piece_type = position.get_piece_on_square(&mov.destination()).piece_type;

    // MVV_LVA:
    if destination_piece_type != PieceType::None {
        score = 10 * PIECES_VALUES[source_piece_type as usize - 1] - PIECES_VALUES[destination_piece_type as usize - 1];
    }

    // Promotion bonus
    match mov.move_type() {
        MoveType::PawnToKnight => score += PROMOTION_SCORE + 300,
        MoveType::PawnToBishop => score += PROMOTION_SCORE + 400,
        MoveType::PawnToRook => score += PROMOTION_SCORE + 500,
        MoveType::PawnToQueen => score += PROMOTION_SCORE + 600,
        _ => {}
    }

    score
}