use crate::lookup_tables::LOOK_UP_TABLE;
use crate::moves::{Move, MoveType};
use crate::piece::{PieceColor, PieceType};
use crate::position::Position;
use crate::utils::{KING_ATTACKS, KNIGHT_ATTACKS, NOT_FILE_A, NOT_FILE_H, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7};

#[inline(always)]
pub fn generate_pseudo_legal_moves(position: &Position) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(64);
    let en_passant = position.get_en_passant();

    let non_own_pieces: u64;
    let pieces_types: [PieceType; 5] = [
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ];
    let mut pieces_board: [u64; 5] = [0; 5];

    match position.get_turn() {
        PieceColor::White => {
            let white_board = position.get_white_board();
            pieces_board[0] = white_board & position.get_knights_board();
            pieces_board[1] = white_board & position.get_bishops_board();
            pieces_board[2] = white_board & position.get_rooks_board();
            pieces_board[3] = white_board & position.get_queens_board();
            pieces_board[4] = white_board & position.get_kings_board();

            non_own_pieces = !position.get_white_board();
            generate_moves_pawn(position, &PieceColor::White, &en_passant, &mut moves);

            let king_square = position.get_king_coord(&PieceColor::White);
            if position.can_white_short_castle() {
                moves.push(Move::new(
                    king_square,
                    king_square + 2,
                    MoveType::ShortCastle,
                ));
            }
            if position.can_white_long_castle() {
                moves.push(Move::new(
                    king_square,
                    king_square - 2,
                    MoveType::LongCastle,
                ));
            }
        }
        PieceColor::Black => {
            let black_board = position.get_black_board();
            pieces_board[0] = black_board & position.get_knights_board();
            pieces_board[1] = black_board & position.get_bishops_board();
            pieces_board[2] = black_board & position.get_rooks_board();
            pieces_board[3] = black_board & position.get_queens_board();
            pieces_board[4] = black_board & position.get_kings_board();

            non_own_pieces = !position.get_black_board();
            generate_moves_pawn(position, &PieceColor::Black, &en_passant, &mut moves);

            let king_square = position.get_king_coord(&PieceColor::Black);
            if position.can_black_short_castle() {
                moves.push(Move::new(
                    king_square,
                    king_square + 2,
                    MoveType::ShortCastle,
                ));
            }
            if position.can_black_long_castle() {
                moves.push(Move::new(
                    king_square,
                    king_square - 2,
                    MoveType::LongCastle,
                ));
            }
        }
        PieceColor::None => panic!("Invalid color"),
    }

    for (board, piece_type) in pieces_board.iter().zip(pieces_types.iter()) {
        let mut board = *board;

        while board != 0 {
            let source = board.trailing_zeros() as u8;
            let mut mask = non_own_pieces & generate_mask_moves(position, &source, piece_type);

            while mask != 0 {
                let destination = mask.trailing_zeros();
                moves.push(Move::new(source, destination as u8, MoveType::Normal));
                mask &= mask - 1;
            }

            board &= board - 1;
        }
    }

    moves
}

#[inline(always)]
pub fn generate_mask_moves(position: &Position, source: &u8, piece_type: &PieceType) -> u64 {
    match piece_type {
        PieceType::Knight => KNIGHT_ATTACKS[*source as usize],
        PieceType::Bishop => generate_move_mask_for_bishop(&position.get_board(), source),
        PieceType::Rook => generate_move_mask_for_rook(&position.get_board(), source),
        PieceType::Queen => generate_move_mask_for_rook(&position.get_board(), source) | generate_move_mask_for_bishop(&position.get_board(), source),
        PieceType::King => KING_ATTACKS[*source as usize],
        _ => 0,
    }
}

// Rook's moves mask
#[inline(always)]
pub fn generate_move_mask_for_rook(board: &u64, source: &u8) -> u64 {
    let occupancy = board & LOOK_UP_TABLE.rook_blockers_masks[*source as usize];
    LOOK_UP_TABLE.rook_attacks[*source as usize][(occupancy.wrapping_mul(LOOK_UP_TABLE.rook_magics[*source as usize]) >> 52) as usize]
}

// Bishop's moves mask
#[inline(always)]
pub fn generate_move_mask_for_bishop(board: &u64, source: &u8) -> u64 {
    let occupancy = board & LOOK_UP_TABLE.bishop_blockers_masks[*source as usize];
    LOOK_UP_TABLE.bishop_attacks[*source as usize][(occupancy.wrapping_mul(LOOK_UP_TABLE.bishop_magics[*source as usize]) >> 55) as usize]
}

// Pawn's moves
#[inline(always)]
pub fn generate_moves_pawn(position: &Position, color: &PieceColor, en_passant: &u8, moves: &mut Vec<Move>) {
    let board = position.get_board();
    let mut opponent_board = match color {
        PieceColor::None => 0,
        PieceColor::White => position.get_black_board(),
        PieceColor::Black => position.get_white_board(),
    };
    opponent_board |= if *en_passant < 64 { 1u64 << en_passant } else { 0 };

    let mut simple_push: (u64, i8) = (0u64, 0i8);
    let mut double_push: (u64, i8) = (0u64, 0i8);
    let mut left_push: (u64, i8) = (0u64, 0i8);
    let mut right_push: (u64, i8) = (0u64, 0i8);

    match color {
        PieceColor::White => {
            let white_pawns = position.get_white_board() & position.get_pawns_board();

            simple_push = ((white_pawns << 8) & !board, 8);

            let rank_3_rank_4 = ((board & RANK_3) << 8) | (board & RANK_4);
            double_push = (((white_pawns & RANK_2) << 16) & !rank_3_rank_4, 16);

            left_push = ((white_pawns << 7) & NOT_FILE_H & opponent_board, 7);
            right_push = ((white_pawns << 9) & NOT_FILE_A & opponent_board, 9);
        }
        PieceColor::Black => {
            let black_pawns = position.get_black_board() & position.get_pawns_board();

            simple_push = ((black_pawns >> 8) & !board, -8);

            let rank_5_rank_6 = ((board & RANK_6) >> 8) | (board & RANK_5);
            double_push = (((black_pawns & RANK_7) >> 16) & !rank_5_rank_6, -16);

            left_push = ((black_pawns >> 9) & NOT_FILE_H & opponent_board, -9);
            right_push = ((black_pawns >> 7) & NOT_FILE_A & opponent_board, -7);
        }
        _ => {}
    }

    for mut push in [simple_push, double_push, left_push, right_push] {
        while push.0 != 0 {
            let destination = push.0.trailing_zeros() as u8;
            let source = (destination as i8 - push.1) as u8;
            if destination == *en_passant && *en_passant != 255 {
                moves.push(Move::new(source, destination, MoveType::EnPassant));
                push.0 &= push.0 - 1;
                continue;
            }

            let destination_rank = 1 + (destination / 8);
            if destination_rank == 1 || destination_rank == 8 {
                moves.push(Move::new(source, destination, MoveType::PawnToQueen));
                moves.push(Move::new(source, destination, MoveType::PawnToRook));
                moves.push(Move::new(source, destination, MoveType::PawnToBishop));
                moves.push(Move::new(source, destination, MoveType::PawnToKnight));

                push.0 &= push.0 - 1;
                continue;
            }

            moves.push(Move::new(source, destination, MoveType::Normal));
            push.0 &= push.0 - 1;
        }
    }
}
