use crate::position::Position;
use crate::psqt::{EG_PIECES_SQUARES_TABLES, EG_PIECES_VALUES, MG_PIECES_SQUARES_TABLES, MG_PIECES_VALUES, PHASE_TABLE, TOTAL_PHASE};
use crate::piece::PieceColor;


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

#[rustfmt::skip]
static LIGHT_SQUARES_MAP: [i32; 64] = [
//  Index 0
//  ↓
    0, 1, 0, 1, 0, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0,
];

#[rustfmt::skip]
static DARK_SQUARES_MAP: [i32; 64] = [
//  Index 0
//  ↓
    1, 0, 1, 0, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 1, 0, 1,
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

    let mut number_of_pieces: [i32; 12] = [0; 12];

    for i in 0..6 {
        let mut board = boards[i] & position.get_white_board();
        let material_weight = 70;
        while board != 0 {
            number_of_pieces[i] += 1;

            let square = board.trailing_zeros();

            let index = (8 * (7 - (square >> 3)) + (square & 7)) as usize;
            mg_evaluation += (MG_PIECES_VALUES[i] * material_weight + MG_PIECES_SQUARES_TABLES[i][index] * (100 - material_weight)) / 100;
            eg_evaluation += (EG_PIECES_VALUES[i] * material_weight + EG_PIECES_SQUARES_TABLES[i][index] * (100 - material_weight)) / 100;

            phase -= PHASE_TABLE[i];
            board &= board - 1;
        }

        board = boards[i] & position.get_black_board();
        while board != 0 {
            number_of_pieces[6 + i] += 1;

            let square = board.trailing_zeros() as usize;

            mg_evaluation -= (MG_PIECES_VALUES[i] * material_weight + MG_PIECES_SQUARES_TABLES[i][square] * (100 - material_weight)) / 100;
            eg_evaluation -= (EG_PIECES_VALUES[i] * material_weight + EG_PIECES_SQUARES_TABLES[i][square] * (100 - material_weight)) / 100;

            phase -= PHASE_TABLE[i];
            board &= board - 1;
        }
    }

    if is_draw_by_insufficient_material(position, number_of_pieces) { return 0; }

    phase = phase.max(0); // If we have a custom setup with more pieces than a normal chess board start position
    phase = (phase * 256 + (TOTAL_PHASE / 2)) / TOTAL_PHASE; // phase from [0, 24] to [0, 256]

    eg_evaluation += king_cornering(&position.get_white_king_square(), &position.get_black_king_square());
    eg_evaluation -= king_cornering(&position.get_black_king_square(), &position.get_white_king_square());

    ((mg_evaluation * (256 - phase)) + (eg_evaluation * phase)) / 256
}

fn king_cornering(friendly_square: &u8, opponent_square: &u8) -> i32 {
    let mut evaluation: i32 = 0;
    let distance_between_kings: i32 = ((friendly_square % 8).abs_diff(opponent_square % 8) + (friendly_square / 8).abs_diff(opponent_square / 8)) as i32;

    evaluation += 47 * ARR_CENTER_MANHATTAN_DISTANCE[(*opponent_square) as usize];
    evaluation += 16 * (14 - distance_between_kings);

    evaluation
}

// number_of_pieces = [WHITE_PAWN, WHITE_KNIGHT, WHITE_BISHOP, WHITE_ROOK, WHITE_QUEEN, WHITE_KING, BLACK_PAWN, BLACK_KNIGHT, BLACK_BISHOP, BLACK_ROOK, BLACK_QUEEN, BLACK_KING]
fn is_draw_by_insufficient_material(position: &Position, number_of_pieces: [i32; 12]) -> bool {
    // king versus king
    if number_of_pieces[0] + number_of_pieces[1] + number_of_pieces[2] + number_of_pieces[3] + number_of_pieces[4] +
        number_of_pieces[6] + number_of_pieces[7] + number_of_pieces[8] + number_of_pieces[9] + number_of_pieces[10] == 0 {
        return true;
    }

    // king and knight versus king
    if (number_of_pieces[0] + number_of_pieces[2] + number_of_pieces[3] + number_of_pieces[4] +
        number_of_pieces[6] + number_of_pieces[8] + number_of_pieces[9] + number_of_pieces[10]) == 0
        && (number_of_pieces[1] - number_of_pieces[7]).abs() == 1 {
        return true;
    }

    // king and bishop versus king
    if (number_of_pieces[0] + number_of_pieces[1] + number_of_pieces[3] + number_of_pieces[4] +
        number_of_pieces[6] + number_of_pieces[7] + number_of_pieces[9] + number_of_pieces[10]) == 0
        && (number_of_pieces[2] - number_of_pieces[8]).abs() == 1 {
        return true;
    }

    // king and bishop versus king and bishop with the bishops on the same color.
    if (number_of_pieces[0] + number_of_pieces[1] + number_of_pieces[3] + number_of_pieces[4] +
        number_of_pieces[6] + number_of_pieces[7] + number_of_pieces[9] + number_of_pieces[10]) == 0 {
        let mut white_bishops_board = position.get_white_board() & position.get_bishops_board();
        let mut black_bishops_board = position.get_black_board() & position.get_bishops_board();

        let mut whites_have_light_square_bishop = false;
        let mut whites_have_dark_square_bishop = false;


        let mut blacks_have_light_square_bishop = false;
        let mut blacks_have_dark_square_bishop = false;

        while white_bishops_board != 0 && !(whites_have_light_square_bishop || whites_have_dark_square_bishop) {
            let square = white_bishops_board.trailing_zeros() as usize;
            whites_have_light_square_bishop = LIGHT_SQUARES_MAP[square] == 1;
            whites_have_dark_square_bishop = DARK_SQUARES_MAP[square] == 1;

            white_bishops_board &= white_bishops_board - 1;
        }


        while black_bishops_board != 0 && !(blacks_have_light_square_bishop || blacks_have_dark_square_bishop) {
            let square = black_bishops_board.trailing_zeros() as usize;
            blacks_have_light_square_bishop = LIGHT_SQUARES_MAP[square] == 1;
            blacks_have_dark_square_bishop = DARK_SQUARES_MAP[square] == 1;

            black_bishops_board &= black_bishops_board - 1;
        }

        if whites_have_light_square_bishop && blacks_have_light_square_bishop && !(whites_have_dark_square_bishop || blacks_have_dark_square_bishop) {
            return true;
        }

        if !(whites_have_light_square_bishop || blacks_have_light_square_bishop) && whites_have_dark_square_bishop && blacks_have_dark_square_bishop {
            return true;
        }
    }

    false
}