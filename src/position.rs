use crate::lookup_tables;
use crate::moves_generator::{
    generate_mask_moves, generate_move_mask_for_bishop, generate_move_mask_for_pawn,
    generate_move_mask_for_rook,
};
use crate::utils::{Move, MoveType, Piece, PieceColor, PieceType, UndoMove};
use std::any::Any;
/*
    Directions and shifts
    +-----+-----+-----+
    | << 7| << 8| << 9|
    +-----+-----+-----+
    | >> 1|  0  | << 1|
    +-----+-----+-----+
    | >> 9| >> 8| >> 7|
    +-----+-----+-----+


     Bit Index (square):
     56 57 58 59 60 61 62 63   ← Rank 8
     48 49 50 51 52 53 54 55   ← Rank 7
     40 41 42 43 44 45 46 47   ← Rank 6
     32 33 34 35 36 37 38 39   ← Rank 5
     24 25 26 27 28 29 30 31   ← Rank 4
     16 17 18 19 20 21 22 23   ← Rank 3
     08 09 10 11 12 13 14 15   ← Rank 2
     00 01 02 03 04 05 06 07   ← Rank 1
     ↑
     File A
*/

#[derive(Clone)]
pub struct Position {
    white_pawns_board: u64,
    white_knights_board: u64,
    white_bishops_board: u64,
    white_rooks_board: u64,
    white_queens_board: u64,
    white_kings_board: u64,

    black_pawns_board: u64,
    black_knights_board: u64,
    black_bishops_board: u64,
    black_rooks_board: u64,
    black_queens_board: u64,
    black_kings_board: u64,

    turn: PieceColor,
    number_of_move: u16,
    castling_rights: u8, // 0 0 0 0 0(q) 0(k) 0(Q) 0(K)

    // En passant square
    en_passant: Option<i8>,

    history: [Option<UndoMove>; 512],
    history_index: usize,
}

impl Position {
    pub fn from_fen(fen: &str) -> Position {
        let mut board_index: usize = 56;

        let mut white_pawns_board: u64 = 0;
        let mut white_knights_board: u64 = 0;
        let mut white_bishops_board: u64 = 0;
        let mut white_rooks_board: u64 = 0;
        let mut white_queens_board: u64 = 0;
        let mut white_kings_board: u64 = 0;

        let mut black_pawns_board: u64 = 0;
        let mut black_knights_board: u64 = 0;
        let mut black_bishops_board: u64 = 0;
        let mut black_rooks_board: u64 = 0;
        let mut black_queens_board: u64 = 0;
        let mut black_kings_board: u64 = 0;

        let mut parts = fen.split_whitespace();
        let board_part = parts.next().expect("Missing board part");
        let turn_part = parts.next().expect("Missing turn part");
        let castling_part = parts.next().unwrap_or("-");
        let en_passant_part = parts.next().unwrap_or("-");
        let half_move_part = parts.next().expect("Missing half move part");
        let number_of_moves_move_part = parts.next().expect("Missing number of moves part");

        for ch in board_part.chars() {
            match ch {
                '/' => {
                    board_index = board_index - 16;
                    continue;
                }

                '1'..='8' => {
                    let skip = ch.to_digit(10).unwrap() - 1;
                    board_index += skip as usize;
                }

                'P' | 'p' => {
                    if ch == 'P' {
                        white_pawns_board |= 1u64 << board_index;
                    } else {
                        black_pawns_board |= 1u64 << board_index;
                    }
                }

                'N' | 'n' => {
                    if ch == 'N' {
                        white_knights_board |= 1u64 << board_index;
                    } else {
                        black_knights_board |= 1u64 << board_index;
                    }
                }

                'B' | 'b' => {
                    if ch == 'B' {
                        white_bishops_board |= 1u64 << board_index;
                    } else {
                        black_bishops_board |= 1u64 << board_index;
                    }
                }

                'R' | 'r' => {
                    if ch == 'R' {
                        white_rooks_board |= 1u64 << board_index;
                    } else {
                        black_rooks_board |= 1u64 << board_index;
                    }
                }

                'Q' | 'q' => {
                    if ch == 'Q' {
                        white_queens_board |= 1u64 << board_index;
                    } else {
                        black_queens_board |= 1u64 << board_index;
                    }
                }

                'K' | 'k' => {
                    if ch == 'K' {
                        white_kings_board |= 1u64 << board_index;
                    } else {
                        black_kings_board |= 1u64 << board_index;
                    }
                }

                ' ' => break, // End of board part in FEN
                _ => panic!("Invalid character in FEN: {}", ch),
            }

            board_index = board_index + 1;
        }

        let turn = match turn_part {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _ => panic!("Invalid turn character in FEN: {}", turn_part),
        };

        let mut castling_rights: u8 = 0;

        if castling_part != "-" {
            for ch in castling_part.chars() {
                match ch {
                    'K' => castling_rights |= 1u8 << 0,
                    'Q' => castling_rights |= 1u8 << 1,
                    'k' => castling_rights |= 1u8 << 2,
                    'q' => castling_rights |= 1u8 << 3,
                    _ => {}
                }
            }
        }

        let mut en_passant_rank: Option<i8> = None;
        let mut en_passant_file: Option<i8> = None;
        if en_passant_part != "-" {
            for ch in en_passant_part.chars() {
                match ch {
                    'a'..='z' => en_passant_file = Some((ch as u8 - 'a' as u8) as i8),
                    '1'..='8' => en_passant_rank = Some((ch.to_digit(10).unwrap() as i8 - 1) * 8),
                    _ => {}
                }
            }
        }

        Position {
            white_pawns_board,
            white_knights_board,
            white_bishops_board,
            white_rooks_board,
            white_queens_board,
            white_kings_board,

            black_pawns_board,
            black_knights_board,
            black_bishops_board,
            black_rooks_board,
            black_queens_board,
            black_kings_board,

            turn,
            number_of_move: number_of_moves_move_part.parse().unwrap(),
            castling_rights,
            en_passant: if en_passant_rank.is_some() && en_passant_file.is_some() {
                Some(en_passant_rank.unwrap() + en_passant_file.unwrap())
            } else {
                None
            },
            history: [None; 512],
            history_index: 0,
        }
    }

    pub fn is_legal_move(&self, mov: &Move) -> bool {
        let source_piece = self.get_piece_on_square(&mov.source);
        let destination_piece = self.get_piece_on_square(&mov.destination);

        // The piece involve in the attack must be well-defined
        if source_piece.piece_type == PieceType::None {
            return false;
        }
        // The attacker must have a well-defined color and must be the one that has to play
        if source_piece.color == PieceColor::None || source_piece.color != self.turn {
            return false;
        }
        // The piece at the destination square could not have the same color as the attacker
        if destination_piece.color == source_piece.color {
            return false;
        }

        match mov.move_type {
            MoveType::ShortCastle => {
                source_piece.piece_type == PieceType::King
                    && self.can_short_castle(&self.turn)
                    && (mov.source.abs_diff(mov.destination) == 2)
            }
            MoveType::LongCastle => {
                source_piece.piece_type == PieceType::King
                    && self.can_long_castle(&self.turn)
                    && (mov.source.abs_diff(mov.destination) == 2)
            }
            _ => {
                // The destination square must appear as one the square that the attacker piece can reach
                let attacks_squares = generate_mask_moves(&self, &mov.source, &source_piece);
                if (attacks_squares & (1u64 << mov.destination)) == 0 {
                    return false;
                }
                true
            }
        }
    }

    pub fn is_square_attack_by(&self, index: &i8, attacker_color: &PieceColor) -> bool {
        let board = self.get_board();
        let your_color = match attacker_color {
            PieceColor::None => panic!("Invalid color"),
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };

        let attacker_pawns_board: u64;
        let attacker_knights_board: u64;
        let attacker_bishops_board: u64;
        let attacker_rooks_board: u64;
        let attacker_queens_board: u64;
        let attacker_kings_board: u64;

        match attacker_color {
            PieceColor::None => panic!("Invalid color"),
            PieceColor::White => {
                attacker_pawns_board = self.white_pawns_board;
                attacker_knights_board = self.white_knights_board;
                attacker_bishops_board = self.white_bishops_board;
                attacker_rooks_board = self.white_rooks_board;
                attacker_queens_board = self.white_queens_board;
                attacker_kings_board = self.white_kings_board;
            }
            PieceColor::Black => {
                attacker_pawns_board = self.black_pawns_board;
                attacker_knights_board = self.black_knights_board;
                attacker_bishops_board = self.black_bishops_board;
                attacker_rooks_board = self.black_rooks_board;
                attacker_queens_board = self.black_queens_board;
                attacker_kings_board = self.black_kings_board;
            }
        }

        let mut superior_king_mask = lookup_tables::LOOK_UP_TABLE.knight_attacks[*index as usize];
        if superior_king_mask & attacker_knights_board != 0 {
            return true;
        }

        superior_king_mask = lookup_tables::LOOK_UP_TABLE.king_attacks[*index as usize];
        if superior_king_mask & attacker_kings_board != 0 {
            return true;
        }

        superior_king_mask = match your_color {
            PieceColor::None => panic!("Invalid color"),
            PieceColor::White => lookup_tables::LOOK_UP_TABLE.white_pawn_attacks[*index as usize],
            PieceColor::Black => lookup_tables::LOOK_UP_TABLE.black_pawn_attacks[*index as usize],
        };

        if superior_king_mask & attacker_pawns_board != 0 {
            return true;
        }

        let superior_bishop_mask = generate_move_mask_for_bishop(&board, &index);
        if superior_bishop_mask & attacker_bishops_board != 0 {
            return true;
        }

        let superior_rook_mask = generate_move_mask_for_rook(&board, &index);
        if superior_rook_mask & attacker_rooks_board != 0 {
            return true;
        }

        superior_king_mask = superior_bishop_mask | superior_rook_mask;
        if superior_king_mask & attacker_queens_board != 0 {
            return true;
        }

        false
    }

    pub fn is_check(&self, color: &PieceColor) -> bool {
        let opponent_color = match color {
            PieceColor::None => {
                panic!("Invalid color")
            }
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
        self.is_square_attack_by(&self.get_king_coord(color), &opponent_color)
    }

    pub fn make_move(&mut self, mov: &Move, is_intern_move_request: bool) {
        // The verification of the origin of the move request helps to avoid the double mask generation when performing move generation:
        // The function generate_mask in move_generator will be call for move generation and for the verification of the move
        // That is unnecessary and may slow down the process
        if !is_intern_move_request && !self.is_legal_move(&mov) {
            println!("Illegal move {:?}", mov);
            self.print_board();
            return;
        }

        let source_piece = self.get_piece_on_square(&mov.source);
        let destination_piece = self.get_piece_on_square(&mov.destination);

        self.history[self.history_index] = Some(UndoMove {
            source: mov.source,
            destination: mov.destination,
            move_type: mov.move_type,
            piece_moved: source_piece.piece_type.clone(),
            piece_captured: destination_piece.piece_type.clone(),
            castling_rights: self.castling_rights,
            turn: self.get_turn(),
            en_passant: self.en_passant,
        });
        self.history_index += 1;

        self.remove_piece_from(&mov.destination, &destination_piece);

        // Putting 0 at the index of the source
        // And moving the piece at the destination by putting 1 at the destination for the corresponding piece
        self.remove_piece_from(&mov.source, &source_piece);
        self.set_piece_to(&mov.destination, &source_piece);

        // Applying castling and promotions rules
        match mov.move_type {
            MoveType::Normal => {}
            MoveType::ShortCastle => {
                self.remove_piece_from(
                    &(mov.source + 3),
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Rook,
                    },
                );
                self.set_piece_to(
                    &(mov.source + 1),
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Rook,
                    },
                );
            }
            MoveType::LongCastle => {
                self.remove_piece_from(
                    &(mov.source - 4),
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Rook,
                    },
                );
                self.set_piece_to(
                    &(mov.source - 1),
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Rook,
                    },
                );
            }
            MoveType::EnPassant => {
                match self.turn {
                    PieceColor::None => { },
                    PieceColor::White => {self.remove_piece_from(
                        &(mov.destination - 8),
                        &Piece {
                            color: PieceColor::Black,
                            piece_type: PieceType::Pawn,
                        },
                    );},
                    PieceColor::Black => {self.remove_piece_from(
                        &(mov.destination + 8),
                        &Piece {
                            color: PieceColor::White,
                            piece_type: PieceType::Pawn,
                        },
                    ); },
                }
            }
            MoveType::PawnToKnight => {
                self.remove_piece_from(
                    &mov.destination,
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Pawn,
                    },
                );
                self.set_piece_to(
                    &mov.destination,
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Knight,
                    },
                );
            }
            MoveType::PawnToBishop => {
                self.remove_piece_from(
                    &mov.destination,
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Pawn,
                    },
                );
                self.set_piece_to(
                    &mov.destination,
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Bishop,
                    },
                );
            }
            MoveType::PawnToRook => {
                self.remove_piece_from(
                    &mov.destination,
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Pawn,
                    },
                );
                self.set_piece_to(
                    &mov.destination,
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Rook,
                    },
                );
            }
            MoveType::PawnToQueen => {
                self.remove_piece_from(
                    &mov.destination,
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Pawn,
                    },
                );
                self.set_piece_to(
                    &mov.destination,
                    &Piece {
                        color: source_piece.color,
                        piece_type: PieceType::Queen,
                    },
                );
            }
        }

        self.en_passant = None;
        if source_piece.piece_type == PieceType::Pawn && mov.source.abs_diff(mov.destination) == 16
        {
            match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.en_passant = Some(mov.destination - 8);
                }
                PieceColor::Black => {
                    self.en_passant = Some(mov.destination + 8);
                }
            }
        }
        match self.turn {
            PieceColor::None => {}
            PieceColor::White => self.turn = PieceColor::Black,
            PieceColor::Black => self.turn = PieceColor::White,
        }
    }

    pub fn undo_last_move(&mut self) {
        self.history_index -= 1;
        let last_move_info = self.history[self.history_index];
        let last_move_info = match last_move_info {
            None => {
                println!("No last move to undo");
                return;
            }
            Some(_) => last_move_info.unwrap(),
        };

        // Delete moved piece from the destination
        // And putting it back at the source
        self.remove_piece_from(
            &last_move_info.destination,
            &Piece {
                color: last_move_info.turn,
                piece_type: last_move_info.piece_moved,
            },
        );
        self.set_piece_to(
            &last_move_info.source,
            &Piece {
                color: last_move_info.turn,
                piece_type: last_move_info.piece_moved,
            },
        );

        // Un-applying castling and promotions rules
        match last_move_info.move_type {
            MoveType::Normal => {}
            MoveType::ShortCastle => {
                self.set_piece_to(
                    &(last_move_info.source + 3),
                    &Piece {
                        color: last_move_info.turn,
                        piece_type: PieceType::Rook,
                    },
                );
                self.remove_piece_from(
                    &(last_move_info.source + 1),
                    &Piece {
                        color: last_move_info.turn,
                        piece_type: PieceType::Rook,
                    },
                );
            }
            MoveType::LongCastle => {
                self.set_piece_to(
                    &(last_move_info.source - 4),
                    &Piece {
                        color: last_move_info.turn,
                        piece_type: PieceType::Rook,
                    },
                );
                self.remove_piece_from(
                    &(last_move_info.source - 1),
                    &Piece {
                        color: last_move_info.turn,
                        piece_type: PieceType::Rook,
                    },
                );
            }
            MoveType::EnPassant => match last_move_info.turn {
                PieceColor::None => {}
                PieceColor::White => {
                    self.set_piece_to(
                        &(last_move_info.destination - 8),
                        &Piece {
                            color: PieceColor::Black,
                            piece_type: PieceType::Pawn,
                        },
                    );
                }
                PieceColor::Black => {
                    self.set_piece_to(
                        &(last_move_info.destination + 8),
                        &Piece {
                            color: PieceColor::White,
                            piece_type: PieceType::Pawn,
                        },
                    );
                }
            }
            MoveType::PawnToKnight => {
                self.remove_piece_from(
                    &last_move_info.destination,
                    &Piece {
                        color: last_move_info.turn,
                        piece_type: PieceType::Knight,
                    },
                );
            }
            MoveType::PawnToBishop => {
                self.remove_piece_from(
                    &last_move_info.destination,
                    &Piece {
                        color: last_move_info.turn,
                        piece_type: PieceType::Bishop,
                    },
                );
            }
            MoveType::PawnToRook => {
                self.remove_piece_from(
                    &last_move_info.destination,
                    &Piece {
                        color: last_move_info.turn,
                        piece_type: PieceType::Rook,
                    },
                );
            }
            MoveType::PawnToQueen => {
                self.remove_piece_from(
                    &last_move_info.destination,
                    &Piece {
                        color: last_move_info.turn,
                        piece_type: PieceType::Queen,
                    },
                );
            }
        }

        // Putting the captured piece back
        self.set_piece_to(
            &last_move_info.destination,
            &Piece {
                color: match last_move_info.turn {
                    PieceColor::None => PieceColor::None,
                    PieceColor::White => PieceColor::Black,
                    PieceColor::Black => PieceColor::White,
                },
                piece_type: last_move_info.piece_captured,
            },
        );

        self.castling_rights = last_move_info.castling_rights;
        self.en_passant = last_move_info.en_passant;
        self.turn = last_move_info.turn.clone();
    }

    pub fn get_turn(&self) -> PieceColor {
        self.turn.clone()
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub fn get_piece_on_square(&self, index: &i8) -> Piece {
        if self.white_pawns_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::White, piece_type: PieceType::Pawn }
        } else if self.white_knights_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::White, piece_type: PieceType::Knight }
        } else if self.white_bishops_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::White, piece_type: PieceType::Bishop }
        } else if self.white_rooks_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::White, piece_type: PieceType::Rook }
        } else if self.white_queens_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::White, piece_type: PieceType::Queen }
        } else if self.white_kings_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::White, piece_type: PieceType::King }
        }

        else if self.black_pawns_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::Black, piece_type: PieceType::Pawn }
        } else if self.black_knights_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::Black, piece_type: PieceType::Knight }
        } else if self.black_bishops_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::Black, piece_type: PieceType::Bishop }
        } else if self.black_rooks_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::Black, piece_type: PieceType::Rook }
        } else if self.black_queens_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::Black, piece_type: PieceType::Queen }
        } else if self.black_kings_board & (1u64 << index) != 0 {
            Piece {color: PieceColor::Black, piece_type: PieceType::King }
        } else {
            Piece {color: PieceColor::None, piece_type: PieceType::None }
        }
    }

    pub fn get_available_piece_coords(&self, piece_color: &PieceColor) -> [Option<i8>; 64] {
        let mut coords = [None; 64];
        let mut board = match piece_color {
            PieceColor::None => 0,
            PieceColor::White => {
                self.white_pawns_board
                    | self.white_knights_board
                    | self.white_bishops_board
                    | self.white_rooks_board
                    | self.white_queens_board
                    | self.white_kings_board
            }
            PieceColor::Black => {
                self.black_pawns_board
                    | self.black_knights_board
                    | self.black_bishops_board
                    | self.black_rooks_board
                    | self.black_queens_board
                    | self.black_kings_board
            }
        };
        let mut cursor = 0;
        while board != 0 {
            coords[cursor] = Some(board.trailing_zeros() as i8);
            board &= board - 1;
            cursor += 1;
        }
        coords
    }

    pub fn get_king_coord(&self, color: &PieceColor) -> i8 {
        if *color == PieceColor::None {
            panic!("Trying to get a king with the color None")
        }
        match color {
            PieceColor::None => {
                panic!("Invalid color")
            }
            PieceColor::White => self.white_kings_board.trailing_zeros() as i8,
            PieceColor::Black => self.black_kings_board.trailing_zeros() as i8,
        }
    }

    pub fn get_board(&self) -> u64 {
        self.white_pawns_board
            | self.white_knights_board
            | self.white_bishops_board
            | self.white_rooks_board
            | self.white_queens_board
            | self.white_kings_board
            | self.black_pawns_board
            | self.black_knights_board
            | self.black_bishops_board
            | self.black_rooks_board
            | self.black_queens_board
            | self.black_kings_board
    }
    pub fn get_white_board(&self) -> u64 {
        self.white_pawns_board
            | self.white_knights_board
            | self.white_bishops_board
            | self.white_rooks_board
            | self.white_queens_board
            | self.white_kings_board
    }
    pub fn get_black_board(&self) -> u64 {
        self.black_pawns_board
            | self.black_knights_board
            | self.black_bishops_board
            | self.black_rooks_board
            | self.black_queens_board
            | self.black_kings_board
    }
    pub fn get_pawns_board(&self) -> u64 {
        self.white_pawns_board | self.black_pawns_board
    }
    pub fn get_knight_board(&self) -> u64 {
        self.white_knights_board | self.black_knights_board
    }
    pub fn get_bishops_board(&self) -> u64 {
        self.white_bishops_board | self.black_bishops_board
    }
    pub fn get_rook_board(&self) -> u64 {
        self.white_rooks_board | self.black_rooks_board
    }
    pub fn get_queens_board(&self) -> u64 {
        self.white_queens_board | self.black_queens_board
    }

    pub fn can_short_castle(&self, color: &PieceColor) -> bool {
        let board = self.get_board();
        let king_index = self.get_king_coord(color);

        match color {
            PieceColor::None => false,
            PieceColor::White => {
                ((self.castling_rights >> 0 & 1) == 1)
                    && (board & (1u64 << (king_index + 1))) == 0
                    && (board & (1u64 << (king_index + 2))) == 0
                    && !self.is_square_attack_by(&king_index, &PieceColor::Black)
                    && !self.is_square_attack_by(&(king_index + 1), &PieceColor::Black)
                    && !self.is_square_attack_by(&(king_index + 2), &PieceColor::Black)
            }
            PieceColor::Black => {
                ((self.castling_rights >> 2 & 1) == 1)
                    && (board & (1u64 << (king_index + 1))) == 0
                    && (board & (1u64 << (king_index + 2))) == 0
                    && !self.is_square_attack_by(&king_index, &PieceColor::White)
                    && !self.is_square_attack_by(&(king_index + 1), &PieceColor::White)
                    && !self.is_square_attack_by(&(king_index + 2), &PieceColor::White)
            }
        }
    }

    pub fn can_long_castle(&self, color: &PieceColor) -> bool {
        let board = self.get_board();
        let king_index = self.get_king_coord(color);

        match color {
            PieceColor::None => false,
            PieceColor::White => {
                ((self.castling_rights >> 1 & 1) == 1)
                    && (board & (1u64 << (king_index - 1))) == 0
                    && (board & (1u64 << (king_index - 2))) == 0
                    && (board & (1u64 << (king_index - 3))) == 0
                    && !self.is_square_attack_by(&king_index, &PieceColor::Black)
                    && !self.is_square_attack_by(&(king_index - 1), &PieceColor::Black)
                    && !self.is_square_attack_by(&(king_index - 2), &PieceColor::Black)
            }
            PieceColor::Black => {
                ((self.castling_rights >> 3 & 1) == 1)
                    && (board & (1u64 << (king_index - 1))) == 0
                    && (board & (1u64 << (king_index - 2))) == 0
                    && (board & (1u64 << (king_index - 3))) == 0
                    && !self.is_square_attack_by(&king_index, &PieceColor::White)
                    && !self.is_square_attack_by(&(king_index - 1), &PieceColor::White)
                    && !self.is_square_attack_by(&(king_index - 2), &PieceColor::White)
            }
        }
    }

    pub fn get_en_passant(&self) -> Option<i8> {
        self.en_passant
    }

    pub fn print_board(&self) {
        for rank in (0..=7).rev() {
            print!("{} ", rank + 1);
            for file in 0..=7 {
                let index = (rank * 8 + file as usize) as i8;
                print!(
                    "{} ",
                    self.piece_to_unicode(&self.get_piece_on_square(&index))
                );
            }
            println!();
        }
        println!("\n  a b c d e f g h\n");
    }

    #[inline(always)]
    fn set_piece_to(&mut self, square: &i8, piece: &Piece) {
        match piece.color {
            PieceColor::None => {}
            PieceColor::White => match piece.piece_type {
                PieceType::None => {}
                PieceType::Pawn => self.white_pawns_board |= 1u64 << square,
                PieceType::Knight => self.white_knights_board |= 1u64 << square,
                PieceType::Bishop => self.white_bishops_board |= 1u64 << square,
                PieceType::Rook => self.white_rooks_board |= 1u64 << square,
                PieceType::Queen => self.white_queens_board |= 1u64 << square,
                PieceType::King => self.white_kings_board |= 1u64 << square,
            },
            PieceColor::Black => match piece.piece_type {
                PieceType::None => {}
                PieceType::Pawn => self.black_pawns_board |= 1u64 << square,
                PieceType::Knight => self.black_knights_board |= 1u64 << square,
                PieceType::Bishop => self.black_bishops_board |= 1u64 << square,
                PieceType::Rook => self.black_rooks_board |= 1u64 << square,
                PieceType::Queen => self.black_queens_board |= 1u64 << square,
                PieceType::King => self.black_kings_board |= 1u64 << square,
            },
        }
    }

    #[inline(always)]
    fn remove_piece_from(&mut self, square: &i8, piece: &Piece) {
        match piece.color {
            PieceColor::None => {}
            PieceColor::White => match piece.piece_type {
                PieceType::None => {}
                PieceType::Pawn => self.white_pawns_board &= !(1u64 << square),
                PieceType::Knight => self.white_knights_board &= !(1u64 << square),
                PieceType::Bishop => self.white_bishops_board &= !(1u64 << square),
                PieceType::Rook => {
                    self.white_rooks_board &= !(1u64 << square);
                    if *square == 7 {
                        self.castling_rights &= !(1u8 << 0);
                    } else if *square == 0 {
                        self.castling_rights &= !(1u8 << 1);
                    }
                }
                PieceType::Queen => self.white_queens_board &= !(1u64 << square),
                PieceType::King => {
                    self.white_kings_board &= !(1u64 << square);
                    self.castling_rights &= !(1u8 << 0);
                    self.castling_rights &= !(1u8 << 1);
                }
            },
            PieceColor::Black => match piece.piece_type {
                PieceType::None => {}
                PieceType::Pawn => self.black_pawns_board &= !(1u64 << square),
                PieceType::Knight => self.black_knights_board &= !(1u64 << square),
                PieceType::Bishop => self.black_bishops_board &= !(1u64 << square),
                PieceType::Rook => {
                    self.black_rooks_board &= !(1u64 << square);
                    if *square == 63 {
                        self.castling_rights &= !(1u8 << 2);
                    } else if *square == 56 {
                        self.castling_rights &= !(1u8 << 3);
                    }
                }
                PieceType::Queen => self.black_queens_board &= !(1u64 << square),
                PieceType::King => {
                    self.black_kings_board &= !(1u64 << square);
                    self.castling_rights &= !(1u8 << 2);
                    self.castling_rights &= !(1u8 << 3);
                }
            },
        }
    }

    fn piece_to_unicode(&self, piece: &Piece) -> char {
        match (piece.color.clone(), piece.piece_type.clone()) {
            (PieceColor::White, PieceType::Pawn) => '♙',
            (PieceColor::White, PieceType::Knight) => '♘',
            (PieceColor::White, PieceType::Bishop) => '♗',
            (PieceColor::White, PieceType::Rook) => '♖',
            (PieceColor::White, PieceType::Queen) => '♕',
            (PieceColor::White, PieceType::King) => '♔',

            (PieceColor::Black, PieceType::Pawn) => '♟',
            (PieceColor::Black, PieceType::Knight) => '♞',
            (PieceColor::Black, PieceType::Bishop) => '♝',
            (PieceColor::Black, PieceType::Rook) => '♜',
            (PieceColor::Black, PieceType::Queen) => '♛',
            (PieceColor::Black, PieceType::King) => '♚',

            _ => '·',
        }
    }
}
