use crate::evaluation::{evaluate_move, get_pst_value};
use crate::lookup_tables;
use crate::position::Position;
use crate::utils::{Move, MoveType, Piece, PieceColor, PieceType};

pub fn generate_legal_moves(position: &mut Position, color: &PieceColor) -> [Option<Move>; 256] {
    let turn = position.get_turn();
    let pseudo_legal_moves = generate_pseudo_legal_moves(position, &turn);
    let mut moves = [None; 256];
    let mut cursor = 0;
    for mov in pseudo_legal_moves {
        match mov {
            None => break,
            Some(m) => {
                position.make_move(&m, false);
                if !position.is_check(&turn) {
                    moves[cursor] = mov;
                    cursor = cursor + 1;
                }
                position.undo_last_move();
            }
        }
    }
    moves
}

pub fn generate_pseudo_legal_moves(position: &Position, color: &PieceColor) -> [Option<Move>; 256] {
    let mut moves = [None; 256];
    let mut cursor = 0;
    let coords = position.get_available_piece_coords(color);
    let en_passant = position.get_en_passant();

    for source in coords {
        let source = match source {
            None => break,
            Some(s) => s,
        };

        let piece = position.get_piece_on_square(&source);
        let mut mask = generate_mask_moves(&position, &source, &piece);
        let mut no_short_castle_manage = true;
        let mut no_long_castle_manage = true;
        while mask != 0 {
            let destination = mask.trailing_zeros() as i8;
            let destination_rank = 1 + (destination / 8);
            match piece.piece_type {
                PieceType::Pawn => {
                    if en_passant.is_some() && destination == en_passant.unwrap() {
                        moves[cursor] = Some(Move {
                            source,
                            destination,
                            move_type: MoveType::EnPassant,
                            move_score: evaluate_move(position, &source, &destination),
                        });
                        mask &= mask - 1;
                        cursor += 1;
                        continue;
                    } else {
                        if destination_rank == 1 || destination_rank == 8 {
                            moves[cursor] = Some(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToKnight,
                                move_score: evaluate_move(position, &source, &destination),
                            });
                            cursor += 1;
                            moves[cursor] = Some(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToBishop,
                                move_score: evaluate_move(position, &source, &destination),
                            });
                            cursor += 1;
                            moves[cursor] = Some(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToRook,
                                move_score: evaluate_move(position, &source, &destination),
                            });
                            cursor += 1;
                            moves[cursor] = Some(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToQueen,
                                move_score: evaluate_move(position, &source, &destination),
                            });
                            cursor += 1;
                            mask &= mask - 1;
                            continue;
                        }
                    }
                }
                PieceType::King => {
                    if no_short_castle_manage && position.can_short_castle(&piece.color) {
                        moves[cursor] = Some(Move {
                            source,
                            destination: source + 2,
                            move_type: MoveType::ShortCastle,
                            move_score: evaluate_move(position, &source, &destination),
                        });
                        cursor += 1;
                        no_short_castle_manage = false;
                    }
                    if no_long_castle_manage && position.can_long_castle(&piece.color) {
                        moves[cursor] = Some(Move {
                            source,
                            destination: source - 2,
                            move_type: MoveType::LongCastle,
                            move_score: evaluate_move(position, &source, &destination),
                        });
                        cursor += 1;
                        no_long_castle_manage = false;
                    }
                }
                _ => {}
            }

            moves[cursor] = Some(Move {
                source,
                destination,
                move_type: MoveType::Normal,
                move_score: evaluate_move(position, &source, &destination),
            });
            cursor += 1;
            mask &= mask - 1;
        }
    }
    moves
}

pub fn generate_mask_moves(position: &Position, source: &i8, piece: &Piece) -> u64 {
    let mut attacks_squares: u64 = match piece.piece_type {
        PieceType::None => 0,
        PieceType::Pawn => generate_move_mask_for_pawn(&position, source, &piece.color),
        PieceType::Knight => lookup_tables::LOOK_UP_TABLE.knight_attacks[*source as usize],
        PieceType::Bishop => generate_move_mask_for_bishop(&position.get_board(), source),
        PieceType::Rook => generate_move_mask_for_rook(&position.get_board(), source),
        PieceType::Queen => {
            generate_move_mask_for_rook(&position.get_board(), source)
                | generate_move_mask_for_bishop(&position.get_board(), source)
        }
        PieceType::King => lookup_tables::LOOK_UP_TABLE.king_attacks[*source as usize],
    };

    // Avoid your own pieces in the attack
    attacks_squares = match piece.color {
        PieceColor::None => attacks_squares,
        PieceColor::White => attacks_squares & !position.get_white_board(),
        PieceColor::Black => attacks_squares & !position.get_black_board(),
    };

    attacks_squares
}

// Rook's moves mask
#[inline(always)]
pub fn generate_move_mask_for_rook(board: &u64, source: &i8) -> u64 {
    let occupancy = board & lookup_tables::LOOK_UP_TABLE.rook_blockers_masks[*source as usize];
    lookup_tables::LOOK_UP_TABLE.rook_attacks[*source as usize][(occupancy
        .wrapping_mul(lookup_tables::LOOK_UP_TABLE.rook_magics[*source as usize])
        >> (64 - 12)) as usize]
}

// Bishop's moves mask
#[inline(always)]
pub fn generate_move_mask_for_bishop(board: &u64, source: &i8) -> u64 {
    let occupancy = board & lookup_tables::LOOK_UP_TABLE.bishop_blockers_masks[*source as usize];
    lookup_tables::LOOK_UP_TABLE.bishop_attacks[*source as usize][(occupancy
        .wrapping_mul(lookup_tables::LOOK_UP_TABLE.bishop_magics[*source as usize])
        >> (64 - 9)) as usize]
}

// Pawn's moves mask
#[inline(always)]
pub fn generate_move_mask_for_pawn(position: &Position, source: &i8, color: &PieceColor) -> u64 {
    let board = position.get_board();
    let rank = 1 + source / 8;
    let en_passant = position.get_en_passant();

    let mut pawn_attacks_mask: u64 = 0;
    match color {
        PieceColor::None => {}
        PieceColor::White => {
            let mut pawn_front_attacks_mask = 1 << (*source + 8);
            // Avoid moving forward if there is ANY piece of the front square
            pawn_front_attacks_mask &= !board;

            let mut pawn_diag_attacks_mask = lookup_tables::LOOK_UP_TABLE.white_pawn_attacks[*source as usize];
            // Avoid moving in the diagonal if there is no piece there
            // And adding the en-passant square, if any
            pawn_diag_attacks_mask &= match en_passant {
                Some(en_passant_square) => board | (1 << en_passant_square),
                None => board,
            };

            pawn_attacks_mask = pawn_front_attacks_mask | pawn_diag_attacks_mask;

            // if the pawn is at the second rank and if the two square in front od it are empty,
            // it can move two-square ahead
            if rank == 2
                && (board & (1u64 << (source + 8))) == 0
                && (board & (1u64 << (source + 16))) == 0
            {
                pawn_attacks_mask |= 1 << (source + 16)
            }
        }
        PieceColor::Black => {
            let mut pawn_front_attacks_mask = 1 << (*source - 8);
            // Avoid moving forward if there is ANY piece of the front square
            pawn_front_attacks_mask &= !board;

            let mut pawn_diag_attacks_mask = lookup_tables::LOOK_UP_TABLE.black_pawn_attacks[*source as usize];
            // Avoid moving in the diagonal if there is no piece there
            // And adding the en-passant square, if any
            pawn_diag_attacks_mask &= match en_passant {
                Some(en_passant_square) => board | (1 << en_passant_square),
                None => board,
            };

            pawn_attacks_mask = pawn_front_attacks_mask | pawn_diag_attacks_mask;

            // if the pawn is at the seven rank and if the two square in front of it are empty,
            // it can move two-square ahead
            if rank == 7
                && (board & (1u64 << (source - 8))) == 0
                && (board & (1u64 << (source - 16))) == 0
            {
                pawn_attacks_mask |= 1 << (source - 16)
            }
        }
    }
    pawn_attacks_mask
}
