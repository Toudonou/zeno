use crate::moves::{Move, MoveType};
use crate::position::Position;
use crate::utils::PieceType;

static PROMOTION_SCORE: i32 = 100_000;

//https://open-chess.org/viewtopic.php?t=3058
static MVV_LVA: [[i32; 6]; 6] = [
    /*      P     N      B      R      Q      K  */
    /*P*/ [6002, 20225, 20250, 20400, 20800, 26900],
    /*N*/ [4775, 6004, 20025, 20175, 20575, 26675],
    /*B*/ [4750, 4975, 6006, 20150, 20550, 26650],
    /*R*/ [4600, 4825, 4850, 6008, 20400, 26500],
    /*Q*/ [4200, 4425, 4450, 4600, 6010, 26100],
    /*K*/ [3100, 3325, 3350, 3500, 3900, 26000],
];

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
        score = MVV_LVA[source_piece_type as usize - 1][destination_piece_type as usize - 1];
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