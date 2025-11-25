use std::cmp::max;
use crate::moves::{Move, MoveType};
use crate::piece::{Piece, PieceColor, PieceType};
use crate::position::Position;
use crate::psqt::{EG_PIECES_SQUARES_TABLES, MG_PIECES_SQUARES_TABLES, MG_PIECES_VALUES, TOTAL_PHASE};

static PV_MOVE_SCORE: i32 = 600_000;
static TT_MOVE_SCORE: i32 = 500_000;
static PROMOTION_MOVE_SCORE: i32 = 400_000;
static CASTLE_MOVE_SCORE: i32 = 100_000;
static CAPTURE_MOVE_SCORE: i32 = 50_000;
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

#[derive(Debug, Clone, Copy)]
struct MoveScore {
    pub primary_score: i32,
    pub secondary_score: i32,
}

#[inline(always)]
pub fn order_moves(moves: &mut Vec<Move>, position: &Position, pv_move: &Option<Move>, tt_move: &Option<Move>, killers: &Option<(Move, Move)>) {
    let pv_move = pv_move.unwrap_or(Move::new(0, 0, MoveType::Normal));
    let tt_move = tt_move.unwrap_or(Move::new(0, 0, MoveType::Normal));

    let killer = killers.unwrap_or((Move::new(0, 0, MoveType::Normal), Move::new(0, 0, MoveType::Normal)));

    let flip = match position.get_turn() {
        PieceColor::White => 0,
        PieceColor::Black => 1,
        PieceColor::None => panic!("Invalid color"),
    };

    let mut phase = position.get_phase().max(0);
    phase = (phase * 256 + (TOTAL_PHASE / 2)) / TOTAL_PHASE; // phase from [0, 24] to [0, 256]

    let mut moves_scores: Vec<(Move, MoveScore)> = vec![(Move::new(0, 0, MoveType::Normal), MoveScore { primary_score: 0, secondary_score: 0 }); moves.len()];

    for i in 0..moves.len() {
        moves_scores[i] = (moves[i], evaluate_move(&moves[i], position, &pv_move, &tt_move, &killer, flip, phase));
    }

    moves_scores.sort_by(|a, b| b.1.primary_score.cmp(&a.1.primary_score).then(a.1.secondary_score.cmp(&b.1.secondary_score)));

    for i in 0..moves.len() {
        moves[i] = moves_scores[i].0;
    }
}

// Between two captures, the one that win materials should be searched first no matter what;
// In case of a tie, the one that involves the less valuable piece will be searched first
// Between two quiets, the one that improves the evaluation of the position (by the PSQT) should be searched first no matter what;
// In case of a tie, the one that involves the less valuable piece will be searched first

#[inline(always)]
fn evaluate_move(mov: &Move, position: &Position, pv_move: &Move, tt_move: &Move, killers: &(Move, Move), flip: i32, phase: i32) -> MoveScore {
    let mut primary_score = 0;
    let mut secondary_score = 0;

    let source_piece = position.get_piece_on_square(&mov.source());
    let destination_piece = position.get_piece_on_square(&mov.destination());

    if mov == pv_move {
        primary_score = PV_MOVE_SCORE;
    } else if mov == tt_move {
        primary_score = TT_MOVE_SCORE;
    }


    // Promotion bonus
    primary_score += match mov.move_type() {
        MoveType::Normal => 0,
        MoveType::ShortCastle => CASTLE_MOVE_SCORE,
        MoveType::LongCastle => CASTLE_MOVE_SCORE,
        MoveType::EnPassant => EN_PASSANT_MOVE_SCORE,
        MoveType::PawnToKnight => PROMOTION_MOVE_SCORE + 300,
        MoveType::PawnToBishop => PROMOTION_MOVE_SCORE + 400,
        MoveType::PawnToRook => PROMOTION_MOVE_SCORE + 500,
        MoveType::PawnToQueen => PROMOTION_MOVE_SCORE + 600,
    };


    // MVV_LVA:
    if destination_piece.piece_type != PieceType::None {
        let see_capture = see_capture(position, &source_piece, &destination_piece, &mov.source(), &mov.destination());

        if see_capture >= 0 {
            primary_score += see_capture + CAPTURE_MOVE_SCORE;
        } else {
            primary_score -= see_capture + CAPTURE_MOVE_SCORE;
        }

        secondary_score += MVV_LVA[source_piece.piece_type.to_usize()][destination_piece.piece_type.to_usize()];
    } else {
        if *mov == (*killers).0 {
            primary_score += KILLER_MOVE_SCORE + 500;
        } else if *mov == (*killers).1 {
            primary_score += KILLER_MOVE_SCORE;
        } else {
            let destination_square = mov.destination() as i32;
            let destination_sq_index = (1 - flip) * (8 * (7 - (destination_square >> 3)) + (destination_square & 7)) + flip * destination_square;

            primary_score += MG_PIECES_SQUARES_TABLES[source_piece.piece_type.to_usize()][destination_sq_index as usize] * (256 - phase) + EG_PIECES_SQUARES_TABLES[source_piece.piece_type.to_usize()][destination_sq_index as usize] * phase;
            secondary_score = MG_PIECES_VALUES[PieceType::King.to_usize()] - MG_PIECES_VALUES[source_piece.piece_type.to_usize()];

            // If the destination square happens to be attacked by an opponent piece, the move could be worse than a bad capture
            // The more valuable is the hanged piece, the more punish the move will be (using the primary score); Even in case of a promotion, a delay of this one can be better
            let attacker = position.get_smallest_attacker(&mov.destination(), &source_piece.color.opposite());
            if attacker.0 != PieceType::None {
                // The primary score is fully reset to ensure that, in case of a promotion, an idea of a delay will be taken into account
                // And a bad quiet is worse than a bad capture
                primary_score = -100 * (MG_PIECES_VALUES[source_piece.piece_type.to_usize()] + CAPTURE_MOVE_SCORE);
            }
        }
    }

    MoveScore { primary_score, secondary_score }
}


// https://www.chessprogramming.org/Static_Exchange_Evaluation#Implementation
#[inline(always)]
pub fn see_capture(position: &Position, attacker: &Piece, victim: &Piece, source: &u8, destination: &u8) -> i32 {
    let mut temp_position = position.clone();

    temp_position.make_see_move(&attacker, victim, source, destination);

    let value = MG_PIECES_VALUES[victim.piece_type.to_usize()] - see(&temp_position, destination, &attacker);

    value * 10
}

#[inline(always)]
fn see(position: &Position, destination: &u8, victim: &Piece) -> i32 {
    let mut value: i32 = 0;
    let attacker_type_and_square = position.get_smallest_attacker(destination, &victim.color.opposite());
    let attacker = Piece { color: victim.color.opposite(), piece_type: attacker_type_and_square.0 };

    if attacker.piece_type != PieceType::None {
        let mut temp_position = position.clone();

        temp_position.make_see_move(&attacker, victim, &attacker_type_and_square.1, destination);

        // should be good, all captures are not forced
        value = max(0, MG_PIECES_VALUES[victim.piece_type.to_usize()] - see(&temp_position, destination, &attacker));
    }

    value
}