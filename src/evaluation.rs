use crate::position::Position;
use crate::psqt::{EG_PIECES_SQUARES_TABLES, EG_PIECES_VALUES, MG_PIECES_SQUARES_TABLES, MG_PIECES_VALUES, PHASE_TABLE, TOTAL_PHASE};

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
        while board != 0 {
            let square = board.trailing_zeros();

            mg_evaluation += MG_PIECES_VALUES[i] + MG_PIECES_SQUARES_TABLES[i][square as usize];
            eg_evaluation += EG_PIECES_VALUES[i] + EG_PIECES_SQUARES_TABLES[i][square as usize];

            phase -= PHASE_TABLE[i];
            board &= board - 1;
        }

        let mut board = boards[i] & position.get_black_board();
        while board != 0 {
            let square = board.trailing_zeros();

            mg_evaluation -= MG_PIECES_VALUES[i] + MG_PIECES_SQUARES_TABLES[i][square as usize];
            eg_evaluation -= EG_PIECES_VALUES[i] + EG_PIECES_SQUARES_TABLES[i][square as usize];

            phase -= PHASE_TABLE[i];
            board &= board - 1;
        }
    }


    phase = (phase * 256 + (TOTAL_PHASE / 2)) / TOTAL_PHASE; // phase from [0, 24] to [0, 256]

    ((mg_evaluation * (256 - phase)) + (eg_evaluation * phase)) / 256
}
