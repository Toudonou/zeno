use crate::position::Position;
use crate::utils::{Piece, PieceColor, PieceType, count_set_bit};

#[inline(always)]
pub fn evaluate(position: &Position) -> i32 {
    pst_evaluation(position)
}

#[inline(always)]
fn pst_evaluation(position: &Position) -> i32 {
    let mut score = 0;
    let white_board = position.get_white_board();
    let black_board = position.get_black_board();
    let pawns_board = position.get_pawns_board();
    let knights_board = position.get_knight_board();
    let bishops_board = position.get_bishops_board();
    let rooks_board = position.get_rook_board();
    let queens_board = position.get_queens_board();

    score += (PieceType::Pawn as i16) as i32
        * (count_set_bit(white_board & pawns_board) as i32
            - count_set_bit(black_board & pawns_board) as i32);
    score += (PieceType::Knight as i16) as i32
        * (count_set_bit(white_board & knights_board) as i32
            - count_set_bit(black_board & knights_board) as i32);
    score += (PieceType::Bishop as i16) as i32
        * (count_set_bit(white_board & bishops_board) as i32
            - count_set_bit(black_board & bishops_board) as i32);
    score += (PieceType::Rook as i16) as i32
        * (count_set_bit(white_board & rooks_board) as i32
            - count_set_bit(black_board & rooks_board) as i32);
    score += (PieceType::Queen as i16) as i32
        * (count_set_bit(white_board & queens_board) as i32
            - count_set_bit(black_board & queens_board) as i32);

    // score += 10 * (generate_legal_moves(position, &PieceColor::White).len() as i32 - generate_legal_moves(position, &PieceColor::Black).len() as i32);

    let mut board = position.get_board();
    while board != 0 {
        let index = board.trailing_zeros() as i8;
        let piece = position.get_piece_on_square(&index);
        score += get_pst_value(&piece, &index);
        board &= board - 1;
    }
    score
}

#[inline(always)]
pub fn get_pst_value(piece: &Piece, index: &i8) -> i32 {
    #[rustfmt::skip]
    let pawn_table = vec![
        00,  00,  00,  00,  00,  00,  00,  00,
        50,  50,  50,  50,  50,  50,  50,  50,
        10,  10,  20,  30,  30,  20,  10,  10,
        05,  05,  10,  25,  25,  10,  05,  05,
        00,  00,  00,  20,  20,  00,  00,  00,
        05, -05, -10,  00,  00, -10, -05,  05,
        05,  10,  10, -20, -20,  10,  10,  05,
        00,  00,  00,  00,  00,  00,  00,  00
    ];

    #[rustfmt::skip]
    let knight_table = vec![
        -50, -40, -30, -30, -30, -30, -40, -50,
        -40, -20,  00,  00,  00,  00, -20, -40,
        -30,  00,  10,  15,  15,  10,  00, -30,
        -30,  05,  15,  20,  20,  15,  05, -30,
        -30,  00,  15,  20,  20,  15,  00, -30,
        -30,  05,  10,  15,  15,  10,  05, -30,
        -40, -20,  00,  05,  05,  00, -20, -40,
        -50, -40, -30, -30, -30, -30, -40, -50,
    ];

    #[rustfmt::skip]
    let bishop_table = vec![
        -20, -10, -10, -10, -10, -10, -10, -20,
        -10,  00,  00,  00,  00,  00,  00, -10,
        -10,  00,  05,  10,  10,  05,  00, -10,
        -10,  05,  05,  10,  10,  05,  05, -10,
        -10,  00,  10,  10,  10,  10,  00, -10,
        -10,  10,  10,  10,  10,  10,  10, -10,
        -10,  05,  00,  00,  00,  00,  05, -10,
        -20, -10, -10, -10, -10, -10, -10, -20,
    ];

    #[rustfmt::skip]
    let rook_table = vec![
         00,  00,  00,  00,  00,  00,  00,  00,
         05,  10,  10,  10,  10,  10,  10,  05,
        -05,  00,  00,  00,  00,  00,  00, -05,
        -05,  00,  00,  00,  00,  00,  00, -05,
        -05,  00,  00,  00,  00,  00,  00, -05,
        -05,  00,  00,  00,  00,  00,  00, -05,
        -05,  00,  00,  00,  00,  00,  00, -05,
         00,  00,  00,  50,  50,  00,  00,  00
    ];

    #[rustfmt::skip]
    let queen_table = vec![
        -20, -10, -10, -05, -05, -10, -10, -20,
        -10,  00,  00,  00,  00,  00,  00, -10,
        -10,  00,  05,  05,  05,  05,  00, -10,
        -05,  00,  05,  05,  05,  05,  00, -05,
         00,  00,  05,  05,  05,  05,  00, -05,
        -10,  05,  05,  05,  05,  05,  00, -10,
        -10,  00,  05,  00,  00,  00,  00, -10,
        -20, -10, -10, -05, -05, -10, -10, -20
    ];

    #[rustfmt::skip]
    let king_table = vec![
        -30, -40, -40, -50, -50, -40, -40, -30,
        -30, -40, -40, -50, -50, -40, -40, -30,
        -30, -40, -40, -50, -50, -40, -40, -30,
        -30, -40, -40, -50, -50, -40, -40, -30,
        -20, -30, -30, -40, -40, -30, -30, -20,
        -10, -20, -20, -20, -20, -20, -20, -10,
         20,  20,  00,  00,  00,  00,  20,  20,
         20,  30,  10,  00,  00,  10,  30,  20
    ];

    let mut rank = index / 8;
    let file = index % 8;
    if piece.color == PieceColor::White {
        rank = 7 - rank;
    }
    let index: usize = (rank * 8 + file) as usize;
    let value = match piece.piece_type {
        PieceType::None => 0,
        PieceType::Pawn => pawn_table[index],
        PieceType::Knight => knight_table[index],
        PieceType::Bishop => bishop_table[index],
        PieceType::Rook => rook_table[index],
        PieceType::Queen => queen_table[index],
        PieceType::King => king_table[index],
    };
    value as i32 * (piece.color as i16) as i32
}

#[inline(always)]
pub fn evaluate_move(position: &Position, source: &i8, destination: &i8) -> i32 {
    let source_piece = position.get_piece_on_square(source);
    let destination_piece = position.get_piece_on_square(&destination);
    let mut move_score: i32 = 0;

    move_score += match source_piece.piece_type {
        PieceType::None => 0,
        PieceType::Pawn => -10,
        PieceType::Knight => -30,
        PieceType::Bishop => -30,
        PieceType::Rook => -50,
        PieceType::Queen => -90,
        PieceType::King => -100,
    };

    move_score += match destination_piece.piece_type {
        PieceType::None => -50,
        PieceType::Pawn => 10,
        PieceType::Knight => 30,
        PieceType::Bishop => 30,
        PieceType::Rook => 50,
        PieceType::Queen => 90,
        PieceType::King => 100,
    };

    move_score
}

#[inline(always)]
pub fn simple_evaluation(position: &Position) -> i32 {
    let mut score: i32 = 0;
    let mut board = position.get_board();
    while board != 0 {
        let piece = position.get_piece_on_square(&(board.trailing_zeros() as i8));
        score = score + (piece.piece_type as i16 * piece.color as i16) as i32;
        board &= board - 1;
    }
    score
}
