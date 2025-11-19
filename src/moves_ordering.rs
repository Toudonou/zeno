use crate::moves::{Move, MoveType};
use crate::piece::PieceType;
use crate::position::Position;

static PV_MOVE_SCORE: i32 = 600_000;
static TT_MOVE_SCORE: i32 = 500_000;
static PROMOTION_MOVE_SCORE: i32 = 400_000;
static CASTLE_MOVE_SCORE: i32 = 100_000;
static KILLER_MOVE_SCORE: i32 = 2000;
static EN_PASSANT_MOVE_SCORE: i32 = 6002;

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
pub fn order_moves(moves: &mut Vec<Move>, position: &Position, pv_move: &Option<Move>, tt_move: &Option<Move>, killers: &Option<(Move, Move)>) {
    let pv_move = pv_move.unwrap_or(Move::new(0, 0, MoveType::Normal));
    let tt_move = tt_move.unwrap_or(Move::new(0, 0, MoveType::Normal));

    let killer = killers.unwrap_or((Move::new(0, 0, MoveType::Normal), Move::new(0, 0, MoveType::Normal)));

    moves.sort_by(|a, b| evaluate_move(b, position, &pv_move, &tt_move, &killer).cmp(&evaluate_move(a, position, &pv_move, &tt_move, &killer)));
}

#[inline(always)]
fn evaluate_move(mov: &Move, position: &Position, pv_move: &Move, tt_move: &Move, killers: &(Move, Move)) -> i32 {
    let mut score = 0;
    let source_piece_type = position.get_piece_on_square(&mov.source()).piece_type;
    let destination_piece_type = position.get_piece_on_square(&mov.destination()).piece_type;

    if mov == pv_move {
        score = PV_MOVE_SCORE;
    } else if mov == tt_move {
        score = TT_MOVE_SCORE;
    }

    // MVV_LVA:
    if destination_piece_type != PieceType::None {
        score += MVV_LVA[source_piece_type.to_usize()][destination_piece_type.to_usize()];
    } else {
        if *mov == (*killers).0 {
            score += KILLER_MOVE_SCORE + 500;
        } else if *mov == (*killers).1 {
            score += KILLER_MOVE_SCORE;
        } else {
            score += match source_piece_type {
                PieceType::Pawn => 100,
                PieceType::Knight => 90,
                PieceType::Bishop => 80,
                PieceType::Rook => 70,
                PieceType::Queen => 50,
                PieceType::King => 0,
                PieceType::None => 0,
            }
        }
    }

    // Promotion bonus
    score += match mov.move_type() {
        MoveType::Normal => 0,
        MoveType::ShortCastle => CASTLE_MOVE_SCORE,
        MoveType::LongCastle => CASTLE_MOVE_SCORE,
        MoveType::EnPassant => EN_PASSANT_MOVE_SCORE,
        MoveType::PawnToKnight => PROMOTION_MOVE_SCORE + 300,
        MoveType::PawnToBishop => PROMOTION_MOVE_SCORE + 400,
        MoveType::PawnToRook => PROMOTION_MOVE_SCORE + 500,
        MoveType::PawnToQueen => PROMOTION_MOVE_SCORE + 600,
    };

    score
}