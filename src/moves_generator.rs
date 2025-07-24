use crate::lookup_tables;
use crate::position::Position;
use crate::utils::{
    ANTI_DIAGONAL, Coord, DIAGONAL, FILE_A, Move, MoveType, Piece, PieceColor, PieceType, RANK_1,
};

pub fn generate_mask_moves(
    white_board: &u64,
    black_board: &u64,
    source: &Coord,
    piece: &Piece,
) -> u64 {
    let mut attacks_squares: u64 = match piece.piece_type {
        PieceType::None => 0,
        PieceType::Pawn => {
            generate_move_mask_for_pawn(&(white_board | black_board), source, &piece.color)
        }
        PieceType::Knight => generate_move_mask_for_knight(source),
        PieceType::Bishop => generate_move_mask_for_bishop(&(white_board | black_board), source),
        PieceType::Rook => generate_move_mask_for_rook(&(white_board | black_board), source),
        PieceType::Queen => {
            generate_move_mask_for_rook(&(white_board | black_board), source)
                | generate_move_mask_for_bishop(&(white_board | black_board), source)
        }
        PieceType::King => generate_move_mask_for_king(source),
    };

    // Avoid your own pieces in the attack
    attacks_squares = match piece.color {
        PieceColor::None => attacks_squares,
        PieceColor::White => attacks_squares & !white_board,
        PieceColor::Black => attacks_squares & !black_board,
    };

    attacks_squares
}

pub fn generate_moves(position: &Position, color: &PieceColor) -> Vec<Move> {
    let mut moves = Vec::new();

    let coords = position.get_available_piece_coords(color);
    let white_board = position.get_white_board();
    let black_board = position.get_black_board();

    for source in &coords {
        let piece = position.get_piece_on_square(&source);
        let mut mask = generate_mask_moves(&white_board, &black_board, &source, &piece);

        while mask != 0 {
            let index = mask.trailing_zeros() as u8;
            let rank = 1 + (index / 8) as i8;
            let file = (index % 8 + 'a' as u8) as char;

            match piece.piece_type {
                PieceType::Pawn => {
                    if rank == 1 || rank == 8 {
                        moves.push(Move {
                            source: source.clone(),
                            destination: Coord { rank, file },
                            move_type: MoveType::PawnToKnight,
                        });
                        moves.push(Move {
                            source: source.clone(),
                            destination: Coord { rank, file },
                            move_type: MoveType::PawnToBishop,
                        });
                        moves.push(Move {
                            source: source.clone(),
                            destination: Coord { rank, file },
                            move_type: MoveType::PawnToRook,
                        });
                        moves.push(Move {
                            source: source.clone(),
                            destination: Coord { rank, file },
                            move_type: MoveType::PawnToQueen,
                        });

                        mask &= mask - 1;
                        continue;
                    }
                }
                PieceType::King => {
                    if position.can_short_castle(&piece.color) {
                        moves.push(Move {
                            source: source.clone(),
                            destination: Coord {
                                rank: source.rank,
                                file: (source.file as u8 + 2) as char,
                            },
                            move_type: MoveType::ShortCastle,
                        });
                    }
                    if position.can_long_castle(&piece.color) {
                        moves.push(Move {
                            source: source.clone(),
                            destination: Coord {
                                rank: source.rank,
                                file: (source.file as u8 - 2) as char,
                            },
                            move_type: MoveType::LongCastle,
                        });
                    }
                }
                _ => {}
            }

            moves.push(Move {
                source: source.clone(),
                destination: Coord { rank, file },
                move_type: MoveType::Normal,
            });
            mask &= mask - 1;
        }
    }
    moves

}

// Rook's moves mask
fn generate_move_mask_for_rook(board: &u64, source: &Coord) -> u64 {
    let mut rank_mask = *board;
    rank_mask >>= (source.rank - 1) * 8; // Move the rank to the first one
    rank_mask &= RANK_1.clone(); // Remove everything that is not on the first rank
    rank_mask &= !(1 << (source.file as u8 - 'a' as u8));

    let mut rank_attacks = *lookup_tables::ROOK_RANK_MASK[(source.file as u8 - 'a' as u8) as usize]
        .get(&rank_mask)
        .unwrap();
    rank_attacks <<= (source.rank - 1) * 8; // Move the rank back to its original position

    let mut file_mask = *board;
    file_mask = file_mask >> (source.file as u8 - 'a' as u8); // Move toward the FILE_A
    file_mask &= FILE_A.clone(); // Remove everything that is not on the FILE_A

    // Make a 90° anti-clock-wise rotation
    let mut file_to_rank: u64 = lookup_tables::FILE_TO_RANK[&file_mask];

    file_to_rank &= !(1 << (source.rank - 1));

    let file_rank_attack_mask = *lookup_tables::ROOK_RANK_MASK[(source.rank - 1) as usize]
        .get(&file_to_rank)
        .unwrap();

    // Compute the final disposition for the file
    let mut file_attacks = lookup_tables::RANK_TO_FILE[&file_rank_attack_mask];
    file_attacks = file_attacks << (source.file as u8 - 'a' as u8); // Move the file back to its original position

    rank_attacks | file_attacks
}

// Bishop's moves mask
fn generate_move_mask_for_bishop(board: &u64, source: &Coord) -> u64 {
    // The file that contains the anti-diagonal for each rank
    // That allows us to know in which direction we will have to move to be on the anti-diagonal
    let anti_diagonal_reference = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let mut anti_diagonal_mask = *board;
    let anti_diag_file = anti_diagonal_reference[(source.rank - 1) as usize];

    let distance_to_anti_diag: i8 = ((source.file as u8) as i8 - (anti_diag_file as u8) as i8);
    if distance_to_anti_diag < 0 {
        anti_diagonal_mask <<= (-1 * distance_to_anti_diag); // Move to the right
    } else if distance_to_anti_diag > 0 {
        anti_diagonal_mask >>= distance_to_anti_diag; // Move to the left
    }

    anti_diagonal_mask &= ANTI_DIAGONAL.clone();

    let mut anti_diag_to_rank: u64 = ((anti_diagonal_mask >> 0 & 1) << 0)
        | ((anti_diagonal_mask >> 9 & 1) << 1)
        | ((anti_diagonal_mask >> 18 & 1) << 2)
        | ((anti_diagonal_mask >> 27 & 1) << 3)
        | ((anti_diagonal_mask >> 36 & 1) << 4)
        | ((anti_diagonal_mask >> 45 & 1) << 5)
        | ((anti_diagonal_mask >> 54 & 1) << 6)
        | ((anti_diagonal_mask >> 63 & 1) << 7);

    anti_diag_to_rank &= !(1 << (source.rank - 1));

    let anti_diag_rank_attack_mask = *lookup_tables::ROOK_RANK_MASK[(source.rank - 1) as usize]
        .get(&anti_diag_to_rank)
        .unwrap();

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
        anti_diag_attacks >>= (-1 * distance_to_anti_diag); // Move to the left
    } else if distance_to_anti_diag > 0 {
        anti_diag_attacks <<= distance_to_anti_diag; // Move to the right
    }

    // Applying the anti-diagonal mask
    // Anti-diag index in the lookup table: file - a + 8 - rank (for the upper anti-diagonal)
    // Anti-diag index in the lookup table: 8 - (file - a + rank) (for the lower anti-diagonal)
    let anti_diag_mask = if distance_to_anti_diag < 0 {
        lookup_tables::ANTI_DIAG_MASK
            [(source.file as u8 - 'a' as u8) as usize + 8 - source.rank as usize]
    } else if distance_to_anti_diag > 0 {
        let index = ('h' as u8 - source.file as u8) as i8 + source.rank - 1;
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
    let diagonal_reference = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let mut diagonal_mask = *board;
    let diag_file = diagonal_reference[(8 - source.rank) as usize];

    let distance_to_diag: i8 = ((source.file as u8) as i8 - (diag_file as u8) as i8);
    if distance_to_diag < 0 {
        diagonal_mask <<= (-1 * distance_to_diag); // Move to the right
    } else if distance_to_diag > 0 {
        diagonal_mask >>= distance_to_diag; // Move to the left
    }

    diagonal_mask &= DIAGONAL.clone();

    let mut diag_to_rank: u64 = ((diagonal_mask >> 7 & 1) << 7)
        | ((diagonal_mask >> 14 & 1) << 6)
        | ((diagonal_mask >> 21 & 1) << 5)
        | ((diagonal_mask >> 28 & 1) << 4)
        | ((diagonal_mask >> 35 & 1) << 3)
        | ((diagonal_mask >> 42 & 1) << 2)
        | ((diagonal_mask >> 49 & 1) << 1)
        | ((diagonal_mask >> 56 & 1) << 0);

    diag_to_rank &= !(1 << (8 - source.rank));

    let diag_rank_attack_mask = *lookup_tables::ROOK_RANK_MASK[(8 - source.rank) as usize]
        .get(&diag_to_rank)
        .unwrap();

    let mut diag_attacks: u64 = ((diag_rank_attack_mask >> 0 & 56) << 7)
        | ((diag_rank_attack_mask >> 1 & 1) << 49)
        | ((diag_rank_attack_mask >> 2 & 1) << 42)
        | ((diag_rank_attack_mask >> 3 & 1) << 35)
        | ((diag_rank_attack_mask >> 4 & 1) << 28)
        | ((diag_rank_attack_mask >> 5 & 1) << 21)
        | ((diag_rank_attack_mask >> 6 & 1) << 14)
        | ((diag_rank_attack_mask >> 7 & 1) << 7);

    // Re-position the diagonal attacks
    if distance_to_diag < 0 {
        diag_attacks >>= (-1 * distance_to_diag); // Move to the left
    } else if distance_to_diag > 0 {
        diag_attacks <<= distance_to_diag; // Move to the right
    }

    // Applying the diagonal mask
    // Diag index in the lookup table: file - a + rank - 1 (for the lower diagonal)
    // Diag index in the lookup table: h - file + 8 - rank (for the upper diagonal)
    let diag_mask = if distance_to_diag < 0 {
        lookup_tables::DIAG_MASK
            [(source.file as u8 - 'a' as u8) as usize + source.rank as usize - 1]
    } else if distance_to_diag > 0 {
        let index = ('h' as u8 - source.file as u8) as usize + 8 - source.rank as usize;
        let mut temp_mask = lookup_tables::DIAG_MASK[index];
        temp_mask >>= (7 - index) * 8; // 7 - index times to the bottom
        temp_mask >>= 7 - index; // 7 - index times to the left
        temp_mask
    } else {
        // The diag itself
        lookup_tables::DIAG_MASK[7]
    };
    diag_attacks &= diag_mask;

    anti_diag_attacks | diag_attacks
}

// Knight's moves mask
fn generate_move_mask_for_knight(source: &Coord) -> u64 {
    let r = source.rank - 1;
    let f = (source.file as u8 - 'a' as u8) as i8;
    let index = r * 8 + f;
    lookup_tables::KNIGHT_MASK[index as usize]
}

// Pawn's moves mask
fn generate_move_mask_for_pawn(board: &u64, source: &Coord, color: &PieceColor) -> u64 {
    let board = *board;

    let r = source.rank - 1;
    let f = source.file as u8 - 'a' as u8;
    let index = r * 8 + f as i8;

    let mut pawn_attacks_mask: u64 = 0;
    match color {
        PieceColor::None => {}
        PieceColor::White => {
            let mut pawn_front_attacks_mask = lookup_tables::PAWN_WHITE_FRONT_MASK[index as usize];
            // Avoid moving forward if there is ANY piece of the front square
            pawn_front_attacks_mask &= !board;

            let mut pawn_diag_attacks_mask = lookup_tables::PAWN_WHITE_DIAG_MASK[index as usize];
            // Avoid moving in the diagonal if there is no piece there
            pawn_diag_attacks_mask &= board;

            pawn_attacks_mask = pawn_front_attacks_mask | pawn_diag_attacks_mask;

            // if the pawn is at the second rank and if the two square in front od it are empty,
            // it can move two-square ahead
            if source.rank == 2
                && (board >> (index + 8)) & 1 == 0
                && (board >> (index + 16)) & 1 == 0
            {
                pawn_attacks_mask |= 1 << (index + 16)
            }
        }
        PieceColor::Black => {
            let mut pawn_front_attacks_mask = lookup_tables::PAWN_BLACK_FRONT_MASK[index as usize];
            // Avoid moving forward if there is ANY piece of the front square
            pawn_front_attacks_mask &= !board;

            let mut pawn_diag_attacks_mask = lookup_tables::PAWN_BLACK_DIAG_MASK[index as usize];
            // Avoid moving in the diagonal if there is no piece there
            pawn_diag_attacks_mask &= board;

            pawn_attacks_mask = pawn_front_attacks_mask | pawn_diag_attacks_mask;

            // if the pawn is at the seven rank and if the two square in front of it are empty,
            // it can move two-square ahead
            if source.rank == 7
                && (board >> (index - 8)) & 1 == 0
                && (board >> (index - 16)) & 1 == 0
            {
                pawn_attacks_mask |= 1 << (index - 16)
            }
        }
    }
    pawn_attacks_mask
}

// King's moves mask
fn generate_move_mask_for_king(source: &Coord) -> u64 {
    let r = source.rank - 1;
    let f = (source.file as u8 - 'a' as u8) as i8;
    let index = r * 8 + f;
    lookup_tables::KING_MASK[index as usize]
}
