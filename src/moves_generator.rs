use crate::lookup_tables;
use crate::position::Position;
use crate::utils::{Move, MoveType, Piece, PieceColor, PieceType};

pub fn generate_legal_moves(position: &Position, color: &PieceColor) -> Vec<Move> {
    let turn = position.get_turn();
    let pseudo_legal_moves: Vec<Move> = generate_pseudo_legal_moves(position, &turn);
    let mut moves: Vec<Move> = Vec::new();
    for m in pseudo_legal_moves {
        let mut temp_position = position.clone();
        temp_position.make_move(&m, false);
        if !temp_position.is_check(&turn) {
            moves.push(m);
        }
    }
    moves
}

fn generate_pseudo_legal_moves(position: &Position, color: &PieceColor) -> Vec<Move> {
    let mut moves = Vec::new();
    let coords = position.get_available_piece_coords(color);
    let en_passant = position.get_en_passant();

    for source in coords {
        let piece = position.get_piece_on_square(&source);
        let mut mask = generate_mask_moves(&position, &source, &piece);

        while mask != 0 {
            let destination = mask.trailing_zeros() as i8;
            let destination_rank = 1 + (destination / 8);
            let mut no_short_castle_manage = true;
            let mut no_long_castle_manage = true;

            match piece.piece_type {
                PieceType::Pawn => {
                    if en_passant.is_some() && destination == en_passant.unwrap(){
                        moves.push(Move {
                            source,
                            destination,
                            move_type: MoveType::EnPassant,
                        });
                        mask &= mask - 1;
                        continue;
                    }else {
                        if destination_rank == 1 || destination_rank == 8 {
                            moves.push(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToKnight,
                            });
                            moves.push(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToBishop,
                            });
                            moves.push(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToRook,
                            });
                            moves.push(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToQueen,
                            });

                            mask &= mask - 1;
                            continue;
                        }
                    }
                }
                PieceType::King => {
                    if no_short_castle_manage && position.can_short_castle(&piece.color) {
                        moves.push(Move {
                            source,
                            destination: source + 2,
                            move_type: MoveType::ShortCastle,
                        });
                        no_short_castle_manage = false;
                    }
                    if no_long_castle_manage && position.can_long_castle(&piece.color) {
                        moves.push(Move {
                            source,
                            destination: source - 2,
                            move_type: MoveType::LongCastle,
                        });
                        no_long_castle_manage = false;
                    }
                }
                _ => {}
            }

            moves.push(Move {
                source,
                destination,
                move_type: MoveType::Normal,
            });
            mask &= mask - 1;
        }
    }
    moves
}

pub fn generate_mask_moves(position: &Position, source: &i8, piece: &Piece) -> u64 {
    let mut attacks_squares: u64 = match piece.piece_type {
        PieceType::None => 0,
        PieceType::Pawn => generate_move_mask_for_pawn(&position, source, &piece.color),
        PieceType::Knight => generate_move_mask_for_knight(source),
        PieceType::Bishop => generate_move_mask_for_bishop(&position.get_board(), source),
        PieceType::Rook => generate_move_mask_for_rook(&position.get_board(), source),
        PieceType::Queen => {
            generate_move_mask_for_rook(&position.get_board(), source)
                | generate_move_mask_for_bishop(&position.get_board(), source)
        }
        PieceType::King => generate_move_mask_for_king(source),
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
    let rank = source / 8;
    let file = source % 8;
    let mut rank_mask = *board;
    rank_mask >>= rank * 8; // Move the rank to the first one
    rank_mask &= 0b0000000000000000000000000000000000000000000000000000000011111111; // Remove everything that is not on the first rank
    rank_mask &= !(1 << file);

    let mut rank_attacks = lookup_tables::ROOK_RANK_MASK[file as usize][rank_mask as usize];
    rank_attacks <<= rank * 8; // Move the rank back to its original position

    let mut file_mask = *board;
    file_mask = file_mask >> file; // Move toward the FILE_A
    file_mask &= 0b0000000100000001000000010000000100000001000000010000000100000001; // Remove everything that is not on the FILE_A

    // Make a 90° anti-clock-wise rotation
    let mut file_to_rank: u64 = ((file_mask >> 0) & 1) << 0
        | ((file_mask >> 8) & 1) << 1
        | ((file_mask >> 16) & 1) << 2
        | ((file_mask >> 24) & 1) << 3
        | ((file_mask >> 32) & 1) << 4
        | ((file_mask >> 40) & 1) << 5
        | ((file_mask >> 48) & 1) << 6
        | ((file_mask >> 56) & 1) << 7;

    file_to_rank &= !(1 << rank);

    let file_rank_attack_mask = lookup_tables::ROOK_RANK_MASK[rank as usize][file_to_rank as usize];

    // Compute the final disposition for the file
    let mut file_attacks = ((file_rank_attack_mask >> 0) & 1) << 0
        | ((file_rank_attack_mask >> 1) & 1) << 8
        | ((file_rank_attack_mask >> 2) & 1) << 16
        | ((file_rank_attack_mask >> 3) & 1) << 24
        | ((file_rank_attack_mask >> 4) & 1) << 32
        | ((file_rank_attack_mask >> 5) & 1) << 40
        | ((file_rank_attack_mask >> 6) & 1) << 48
        | ((file_rank_attack_mask >> 7) & 1) << 56;
    file_attacks = file_attacks << file; // Move the file back to its original position

    rank_attacks | file_attacks
}

// Bishop's moves mask
#[inline(always)]
pub fn generate_move_mask_for_bishop(board: &u64, source: &i8) -> u64 {
    let rank = source / 8;
    let file = source % 8;

    // The file that contains the anti-diagonal for each rank
    // That allows us to know in which direction we will have to move to be on the anti-diagonal
    // let anti_diagonal_reference = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let anti_diagonal_reference = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut anti_diagonal_mask = *board;

    let distance_to_anti_diag: i8 = file - anti_diagonal_reference[rank as usize];
    if distance_to_anti_diag < 0 {
        anti_diagonal_mask <<= -1 * distance_to_anti_diag; // Move to the right
    } else if distance_to_anti_diag > 0 {
        anti_diagonal_mask >>= distance_to_anti_diag; // Move to the left
    }

    anti_diagonal_mask &= 0b1000000001000000001000000001000000001000000001000000001000000001;

    let mut anti_diag_to_rank: u64 = ((anti_diagonal_mask >> 0 & 1) << 0)
        | ((anti_diagonal_mask >> 9 & 1) << 1)
        | ((anti_diagonal_mask >> 18 & 1) << 2)
        | ((anti_diagonal_mask >> 27 & 1) << 3)
        | ((anti_diagonal_mask >> 36 & 1) << 4)
        | ((anti_diagonal_mask >> 45 & 1) << 5)
        | ((anti_diagonal_mask >> 54 & 1) << 6)
        | ((anti_diagonal_mask >> 63 & 1) << 7);

    anti_diag_to_rank &= !(1 << rank);

    let anti_diag_rank_attack_mask =
        lookup_tables::ROOK_RANK_MASK[rank as usize][anti_diag_to_rank as usize];

    let mut anti_diag_attacks: u64 = ((anti_diag_rank_attack_mask >> 0 & 1) << 0)
        | ((anti_diag_rank_attack_mask >> 1 & 1) << 9)
        | ((anti_diag_rank_attack_mask >> 2 & 1) << 18)
        | ((anti_diag_rank_attack_mask >> 3 & 1) << 27)
        | ((anti_diag_rank_attack_mask >> 4 & 1) << 36)
        | ((anti_diag_rank_attack_mask >> 5 & 1) << 45)
        | ((anti_diag_rank_attack_mask >> 6 & 1) << 54)
        | ((anti_diag_rank_attack_mask >> 7 & 1) << 63);

    // Re-position the anti-diagonal attacks
    if distance_to_anti_diag < 0 {
        anti_diag_attacks >>= -1 * distance_to_anti_diag; // Move to the left
    } else if distance_to_anti_diag > 0 {
        anti_diag_attacks <<= distance_to_anti_diag; // Move to the right
    }

    // Applying the anti-diagonal mask
    // Anti-diag index in the lookup table: file - a + 8 - rank (for the upper anti-diagonal)
    // Anti-diag index in the lookup table: 8 - (file - a + rank) (for the lower anti-diagonal)
    let anti_diag_mask = if distance_to_anti_diag < 0 {
        lookup_tables::ANTI_DIAG_MASK[file as usize + 7 - rank as usize]
    } else if distance_to_anti_diag > 0 {
        let index = 7 - file + rank;
        let mut temp_mask = lookup_tables::ANTI_DIAG_MASK[index as usize];
        temp_mask >>= (7 - index) * 8; // 7 - index times to the bottom
        temp_mask <<= 7 - index; // 7 - index times to the right
        temp_mask
    } else {
        // The anti-diag itself
        lookup_tables::ANTI_DIAG_MASK[7]
    };

    anti_diag_attacks &= anti_diag_mask;

    // The same thing wil be done for the DIAGONAL

    // The file that contains the diagonal for each rank
    // That allows us to know in which direction we will have to move to be on the diagonal
    let diagonal_reference = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut diagonal_mask = *board;

    let distance_to_diag: i8 = file - diagonal_reference[(7 - rank) as usize];
    if distance_to_diag < 0 {
        diagonal_mask <<= -1 * distance_to_diag; // Move to the right
    } else if distance_to_diag > 0 {
        diagonal_mask >>= distance_to_diag; // Move to the left
    }

    diagonal_mask &= 0b0000000100000010000001000000100000010000001000000100000010000000;

    let mut diag_to_rank: u64 = ((diagonal_mask >> 7 & 1) << 7)
        | ((diagonal_mask >> 14 & 1) << 6)
        | ((diagonal_mask >> 21 & 1) << 5)
        | ((diagonal_mask >> 28 & 1) << 4)
        | ((diagonal_mask >> 35 & 1) << 3)
        | ((diagonal_mask >> 42 & 1) << 2)
        | ((diagonal_mask >> 49 & 1) << 1)
        | ((diagonal_mask >> 56 & 1) << 0);

    diag_to_rank &= !(1 << (7 - rank));

    let diag_rank_attack_mask =
        lookup_tables::ROOK_RANK_MASK[(7 - rank) as usize][diag_to_rank as usize];

    let mut diag_attacks: u64 = ((diag_rank_attack_mask >> 0 & 1) << 56)
        | ((diag_rank_attack_mask >> 1 & 1) << 49)
        | ((diag_rank_attack_mask >> 2 & 1) << 42)
        | ((diag_rank_attack_mask >> 3 & 1) << 35)
        | ((diag_rank_attack_mask >> 4 & 1) << 28)
        | ((diag_rank_attack_mask >> 5 & 1) << 21)
        | ((diag_rank_attack_mask >> 6 & 1) << 14)
        | ((diag_rank_attack_mask >> 7 & 1) << 7);

    // Re-position the diagonal attacks
    if distance_to_diag < 0 {
        diag_attacks >>= -1 * distance_to_diag; // Move to the left
    } else if distance_to_diag > 0 {
        diag_attacks <<= distance_to_diag; // Move to the right
    }

    // Applying the diagonal mask
    // Diag index in the lookup table: file - a + rank - 1 (for the lower diagonal)
    // Diag index in the lookup table: h - file + 8 - rank (for the upper diagonal)
    let diag_mask = if distance_to_diag < 0 {
        lookup_tables::DIAG_MASK[file as usize + rank as usize]
    } else if distance_to_diag > 0 {
        let index = 7 - file + 7 - rank;
        let mut temp_mask = lookup_tables::DIAG_MASK[index as usize];
        temp_mask <<= (7 - index) * 8; // 7 - index times to the up
        temp_mask <<= 7 - index; // 7 - index times to the right
        temp_mask
    } else {
        // The diag itself
        lookup_tables::DIAG_MASK[7]
    };
    diag_attacks &= diag_mask;

    anti_diag_attacks | diag_attacks
}

// Knight's moves mask
#[inline(always)]
pub fn generate_move_mask_for_knight(source: &i8) -> u64 {
    lookup_tables::KNIGHT_MASK[*source as usize]
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

            let mut pawn_diag_attacks_mask = lookup_tables::PAWN_WHITE_DIAG_MASK[*source as usize];
            // Avoid moving in the diagonal if there is no piece there
            // And adding the en-passant square, if any
            pawn_diag_attacks_mask &= match en_passant {
                Some(en_passant_square) => board | (1 << en_passant_square),
                None => board,
            };

            pawn_attacks_mask = pawn_front_attacks_mask | pawn_diag_attacks_mask;

            // if the pawn is at the second rank and if the two square in front od it are empty,
            // it can move two-square ahead
            if rank == 2 && (board >> (source + 8)) & 1 == 0 && (board >> (source + 16)) & 1 == 0 {
                pawn_attacks_mask |= 1 << (source + 16)
            }
        }
        PieceColor::Black => {
            let mut pawn_front_attacks_mask = 1 << (*source - 8);
            // Avoid moving forward if there is ANY piece of the front square
            pawn_front_attacks_mask &= !board;

            let mut pawn_diag_attacks_mask = lookup_tables::PAWN_BLACK_DIAG_MASK[*source as usize];
            // Avoid moving in the diagonal if there is no piece there
            // And adding the en-passant square, if any
            pawn_diag_attacks_mask &= match en_passant {
                Some(en_passant_square) => board | (1 << en_passant_square),
                None => board,
            };

            pawn_attacks_mask = pawn_front_attacks_mask | pawn_diag_attacks_mask;

            // if the pawn is at the seven rank and if the two square in front of it are empty,
            // it can move two-square ahead
            if rank == 7 && (board >> (source - 8)) & 1 == 0 && (board >> (source - 16)) & 1 == 0 {
                pawn_attacks_mask |= 1 << (source - 16)
            }
        }
    }
    pawn_attacks_mask
}

// King's moves mask
#[inline(always)]
pub fn generate_move_mask_for_king(source: &i8) -> u64 {
    lookup_tables::KING_MASK[*source as usize]
}
