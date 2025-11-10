use crate::position::Position;
use crate::psqt::PIECES_SQUARES_TABLES;

// Value from https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function
static PAWN_VALUE: i32 = 82;
static KNIGHT_VALUE: i32 = 337;
static BISHOP_VALUE: i32 = 365;
static ROOK_VALUE: i32 = 477;
static QUEEN_VALUE: i32 = 1_025;
static KING_VALUE: i32 = 10_000;

pub static PIECES_VALUES: [i32; 6] = [PAWN_VALUE, KNIGHT_VALUE, BISHOP_VALUE, ROOK_VALUE, QUEEN_VALUE, KING_VALUE];
pub static MATE_SCORE: i32 = 1000_000;

#[inline(always)]
pub fn evaluate(position: &Position) -> i32 {
    psqt_evaluation(position)
}

#[inline(always)]
fn psqt_evaluation(position: &Position) -> i32 {
    let mut score: i32 = 0;
    let boards: [u64; 6] = [position.get_pawns_board(), position.get_knights_board(), position.get_bishops_board(), position.get_rooks_board(), position.get_queens_board(), position.get_kings_board()];

    for i in 0..6 {
        let mut board = boards[i] & position.get_white_board();
        while board != 0 {
            let square = board.trailing_zeros();

            score += PIECES_VALUES[i] + PIECES_SQUARES_TABLES[i][square as usize];

            board &= board - 1;
        }

        let mut board = boards[i] & position.get_black_board();
        while board != 0 {
            let square = board.trailing_zeros();

            score -= PIECES_VALUES[i] + PIECES_SQUARES_TABLES[i][square as usize];

            board &= board - 1;
        }
    }

    score
}
