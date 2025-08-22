use crate::evaluation::evaluate_move;
use crate::lookup_tables;
use crate::perft::perft;
use crate::position::Position;
use crate::utils::{Move, MoveType, Piece, PieceColor, PieceType};

pub fn generate_legal_moves(position: &Position, color: &PieceColor) -> [Option<Move>; 256] {
    let mut moves = [None; 256];
    let mut move_cursor = 0;
    let king_square = position.get_king_coord(color);
    let checkers = position.get_attackers_of_square(&king_square, color);

    if checkers.len() == 0 {
        let moves_and_len = generate_pseudo_legal_moves(position, color);
        moves = moves_and_len.1;
        move_cursor = moves_and_len.0;
    } else if checkers.len() == 1 {
        let pins_pieces = position.get_pin_pieces_for(color);
        let checker: (PieceType, i8, u64) = checkers[0];
        match checker.0 {
            PieceType::King => {}
            PieceType::Knight | PieceType::Pawn => {
                let check_resolvers = position.get_attackers_of_square(
                    &checker.1,
                    match color {
                        PieceColor::None => &PieceColor::None,
                        PieceColor::White => &PieceColor::Black,
                        PieceColor::Black => &PieceColor::White,
                    },
                );
                for (_piece_type, square, _ray) in check_resolvers {
                    if pins_pieces[square as usize] == 0
                        || ((pins_pieces[square as usize] & (1u64 << checker.1)) != 0)
                    {
                        moves[move_cursor] = Some(Move {
                            source: square,
                            destination: checker.1,
                            move_type: MoveType::Normal,
                            move_score: 0,
                        });
                        move_cursor += move_cursor;
                    }
                }
            }
            _ => {
                let mut ray = checker.2;
                while ray != 0 {
                    let destination_square = ray.trailing_zeros() as i8;
                    let check_resolvers = position.get_attackers_of_square(
                        &destination_square,
                        match color {
                            PieceColor::None => &PieceColor::None,
                            PieceColor::White => &PieceColor::Black,
                            PieceColor::Black => &PieceColor::White,
                        },
                    );
                    for (_piece_type, square, _ray) in check_resolvers {
                        if pins_pieces[square as usize] == 0
                            || ((pins_pieces[square as usize] & (1u64 << destination_square)) != 0)
                        {
                            moves[move_cursor] = Some(Move {
                                source: square,
                                destination: destination_square,
                                move_type: MoveType::Normal,
                                move_score: 0,
                            });
                            move_cursor += move_cursor;
                        }
                    }
                    ray &= ray - 1;
                }
            }
        }
    };

    let mut king_moves_mask = lookup_tables::LOOK_UP_TABLE.king_attacks[king_square as usize];
    let mut king_moves: [Option<Move>; 8] = [None; 8];
    let mut king_moves_cursor = 0;
    king_moves_mask = match color {
        PieceColor::None => king_moves_mask,
        PieceColor::White => king_moves_mask & !position.get_white_board(),
        PieceColor::Black => king_moves_mask & !position.get_black_board(),
    };
    let mut no_short_castle_manage = position.can_short_castle(&color);
    let mut no_long_castle_manage = position.can_long_castle(&color);
    while king_moves_mask != 0 {
        let destination = king_moves_mask.trailing_zeros() as i8;
        if no_short_castle_manage {
            king_moves[king_moves_cursor] = Some(Move {
                source: king_square,
                destination: king_square + 2,
                move_type: MoveType::ShortCastle,
                move_score: 0,
            });
            king_moves_cursor += 1;
            no_short_castle_manage = false;
        }
        if no_long_castle_manage {
            king_moves[king_moves_cursor] = Some(Move {
                source: king_square,
                destination: king_square - 2,
                move_type: MoveType::LongCastle,
                move_score: 0,
            });
            king_moves_cursor += 1;
            no_long_castle_manage = false;
        }
        king_moves[king_moves_cursor] = Some(Move {
            source: king_square,
            destination,
            move_type: MoveType::Normal,
            move_score: 0,
        });
        king_moves_cursor += 1;
        king_moves_mask &= king_moves_mask - 1;
    }

    for mov in &king_moves {
        match mov {
            None => break,
            Some(m) => {
                let mut temp_position = position.clone();
                temp_position.make_move(&m, true);
                if temp_position.is_check(&color) {
                    continue;
                }
                moves[move_cursor] = mov.clone();
                move_cursor += 1;
            }
        }
    }

    moves
}

pub fn generate_pseudo_legal_moves(
    position: &Position,
    color: &PieceColor,
) -> (usize, [Option<Move>; 256]) {
    let mut moves = [None; 256];
    let mut cursor = 0;
    let coords = position.get_available_piece_coords(color);
    let en_passant = position.get_en_passant();
    let pins_pieces = position.get_pin_pieces_for(color);

    for source in coords {
        let source = match source {
            None => break,
            Some(s) => s,
        };

        let piece = position.get_piece_on_square(&source);
        if piece.piece_type == PieceType::King {
            continue;
        }

        let mut mask = generate_mask_moves(&position, &source, &piece);
        while mask != 0 {
            let destination = mask.trailing_zeros() as i8;
            let destination_rank = 1 + (destination / 8);

            if pins_pieces[source as usize] == 0
                || ((pins_pieces[source as usize] & (1u64 << destination)) != 0)
            {
                if piece.piece_type == PieceType::Pawn {
                    if en_passant.is_some() && destination == en_passant.unwrap() {
                        moves[cursor] = Some(Move {
                            source,
                            destination,
                            move_type: MoveType::EnPassant,
                            move_score: 0,
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
                                move_score: 0,
                            });
                            cursor += 1;
                            moves[cursor] = Some(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToBishop,
                                move_score: 0,
                            });
                            cursor += 1;
                            moves[cursor] = Some(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToRook,
                                move_score: 0,
                            });
                            cursor += 1;
                            moves[cursor] = Some(Move {
                                source,
                                destination,
                                move_type: MoveType::PawnToQueen,
                                move_score: 0,
                            });
                            cursor += 1;
                            mask &= mask - 1;
                            continue;
                        }
                    }
                }

                moves[cursor] = Some(Move {
                    source,
                    destination,
                    move_type: MoveType::Normal,
                    move_score: 0,
                });
                cursor += 1;
            }
            mask &= mask - 1;
        }
    }
    (cursor, moves)
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
        PieceType::King => 0,
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

            let mut pawn_diag_attacks_mask =
                lookup_tables::LOOK_UP_TABLE.white_pawn_attacks[*source as usize];
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

            let mut pawn_diag_attacks_mask =
                lookup_tables::LOOK_UP_TABLE.black_pawn_attacks[*source as usize];
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
