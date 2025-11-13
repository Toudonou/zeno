use crate::position::Position;
use crate::psqt::{EG_PIECES_SQUARES_TABLES, EG_PIECES_VALUES, MG_PIECES_SQUARES_TABLES, MG_PIECES_VALUES, PHASE_TABLE, TOTAL_PHASE};
use crate::utils::PieceColor;


static ARR_CENTER_MANHATTAN_DISTANCE: [i32; 64] = [
    6, 5, 4, 3, 3, 4, 5, 6,
    5, 4, 3, 2, 2, 3, 4, 5,
    4, 3, 2, 1, 1, 2, 3, 4,
    3, 2, 1, 0, 0, 1, 2, 3,
    3, 2, 1, 0, 0, 1, 2, 3,
    4, 3, 2, 1, 1, 2, 3, 4,
    5, 4, 3, 2, 2, 3, 4, 5,
    6, 5, 4, 3, 3, 4, 5, 6
];

#[inline(always)]
pub fn evaluate(position: &Position) -> i32 {
    tapered_evaluation(position)
}

#[inline(always)]
fn tapered_evaluation(position: &Position) -> i32 {
    let mut mg_evaluation: i32 = 0;
    let mut eg_evaluation: i32 = 0;
    let boards: [u64; 6] = [position.get_pawns_board(), position.get_knights_board(), position.get_bishops_board(), position.get_rooks_board(), position.get_queens_board(), position.get_kings_board()];
    let mut phase = TOTAL_PHASE;

    for i in 0..6 {
        let mut board = boards[i] & position.get_white_board();
        let material_weight = 70;
        while board != 0 {
            let square = board.trailing_zeros();

            let index = (8 * (7 - (square >> 3)) + (square & 7)) as usize;
            mg_evaluation += (MG_PIECES_VALUES[i] * material_weight + MG_PIECES_SQUARES_TABLES[i][index] * (100 - material_weight)) / 100;
            eg_evaluation += (EG_PIECES_VALUES[i] * material_weight + EG_PIECES_SQUARES_TABLES[i][index] * (100 - material_weight)) / 100;

            phase -= PHASE_TABLE[i];
            board &= board - 1;
        }

        let mut board = boards[i] & position.get_black_board();
        while board != 0 {
            let square = board.trailing_zeros() as usize;

            mg_evaluation -= (MG_PIECES_VALUES[i] * material_weight + MG_PIECES_SQUARES_TABLES[i][square] * (100 - material_weight)) / 100;
            eg_evaluation -= (EG_PIECES_VALUES[i] * material_weight + EG_PIECES_SQUARES_TABLES[i][square] * (100 - material_weight)) / 100;

            phase -= PHASE_TABLE[i];
            board &= board - 1;
        }
    }

    phase = phase.max(0); // If we have a custom setup with more pieces than a normal chess board start position
    phase = (phase * 256 + (TOTAL_PHASE / 2)) / TOTAL_PHASE; // phase from [0, 24] to [0, 256]

    eg_evaluation += king_cornering(&position.get_king_coord(&PieceColor::White), &position.get_king_coord(&PieceColor::Black));
    eg_evaluation -= king_cornering(&position.get_king_coord(&PieceColor::Black), &position.get_king_coord(&PieceColor::White));

    ((mg_evaluation * (256 - phase)) + (eg_evaluation * phase)) / 256
}

fn king_cornering(friendly_square: &u8, opponent_square: &u8) -> i32 {
    let mut evaluation: i32 = 0;
    let distance_between_kings: i32 = ((friendly_square % 8).abs_diff(opponent_square % 8) + (friendly_square / 8).abs_diff(opponent_square / 8)) as i32;

    evaluation += 47 * ARR_CENTER_MANHATTAN_DISTANCE[(*opponent_square) as usize];
    evaluation += 16 * (14 - distance_between_kings);

    evaluation
}
