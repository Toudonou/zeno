use crate::position::Position;
use crate::utils::{Piece, PieceColor, PieceType};

pub fn evaluate(position: &Position) -> i32 {
    pst_evaluation(position)
}

fn pst_evaluation(position: &Position) -> i32 {
    let mut score= simple_evaluation(position);

    for rank in 0..=7 {
        for file in 0..=7 {
            let index = (rank * 8 + file as usize) as i8;
            let piece = position.get_piece_on_square(&index);
            score = score + get_pst_value(&piece, &index);
        }
    }

    score
}

fn get_pst_value(piece: &Piece, index: &i8) -> i32 {
    // #[Rustfmt::skip]
    let pawn_table = vec![
        0, 0, 0, 0, 0, 0, 0, 0,
        50, 50, 50, 50, 50, 50, 50, 50,
        10, 10, 20, 30, 30, 20, 10, 10,
        5, 5, 10, 25, 25, 10, 5, 5,
        0, 0, 0, 20, 20, 0, 0, 0,
        5, -5, -10, 0, 0, -10, -5, 5,
        5, 10, 10, -20, -20, 10, 10, 5,
        0, 0, 0, 0, 0, 0, 0, 0
    ];
    let knight_table = vec![
        -50, -40, -30, -30, -30, -30, -40, -50,
        -40, -20, 0, 0, 0, 0, -20, -40,
        -30, 0, 10, 15, 15, 10, 0, -30,
        -30, 5, 15, 20, 20, 15, 5, -30,
        -30, 0, 15, 20, 20, 15, 0, -30,
        -30, 5, 10, 15, 15, 10, 5, -30,
        -40, -20, 0, 5, 5, 0, -20, -40,
        -50, -40, -30, -30, -30, -30, -40, -50,
    ];
    let bishop_table = vec![
        -20, -10, -10, -10, -10, -10, -10, -20,
        -10, 0, 0, 0, 0, 0, 0, -10,
        -10, 0, 5, 10, 10, 5, 0, -10,
        -10, 5, 5, 10, 10, 5, 5, -10,
        -10, 0, 10, 10, 10, 10, 0, -10,
        -10, 10, 10, 10, 10, 10, 10, -10,
        -10, 5, 0, 0, 0, 0, 5, -10,
        -20, -10, -10, -10, -10, -10, -10, -20,
    ];
    let rook_table = vec![
        0, 0, 0, 0, 0, 0, 0, 0,
        5, 10, 10, 10, 10, 10, 10, 5,
        -5, 0, 0, 0, 0, 0, 0, -5,
        -5, 0, 0, 0, 0, 0, 0, -5,
        -5, 0, 0, 0, 0, 0, 0, -5,
        -5, 0, 0, 0, 0, 0, 0, -5,
        -5, 0, 0, 0, 0, 0, 0, -5,
        0, 0, 0, 5, 5, 0, 0, 0
    ];
    let queen_table = vec![
        -20, -10, -10, -5, -5, -10, -10, -20,
        -10, 0, 0, 0, 0, 0, 0, -10,
        -10, 0, 5, 5, 5, 5, 0, -10,
        -5, 0, 5, 5, 5, 5, 0, -5,
        0, 0, 5, 5, 5, 5, 0, -5,
        -10, 5, 5, 5, 5, 5, 0, -10,
        -10, 0, 5, 0, 0, 0, 0, -10,
        -20, -10, -10, -5, -5, -10, -10, -20
    ];
    let king_table = vec![
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        20, 20,  0,  0,  0,  0, 20, 20,
        20, 30, 10,  0,  0, 10, 30, 200
    ];

    let mut rank = index / 8;
    let file = index%8;
    
    let mut value = 0;

    if piece.color == PieceColor::Black {
        rank = 7 - rank + 1;
    }

    let i = 8 - rank;
    let index = (8 * i + file) as usize;

    match piece.piece_type {
        PieceType::None => {}
        PieceType::Pawn => value = pawn_table[index],
        PieceType::Knight => value = knight_table[index],
        PieceType::Bishop => value = bishop_table[index],
        PieceType::Rook => value = rook_table[index],
        PieceType::Queen => value = queen_table[index],
        PieceType::King => value = king_table[index],
    }

    value * piece.color.clone() as i32
}

fn simple_evaluation(position: &Position) -> i32 {
    let mut score: i32 = 0;

    for rank in 0..=7 {
        for file in 0..=7 {
            let index = (rank * 8 + file as usize) as i8;
            let piece = position.get_piece_on_square(&index);
            score = score + (piece.piece_type as i16 * piece.color as i16) as i32;
        }
    }
    score
}