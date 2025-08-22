use crate::lookup_tables;
use crate::moves_generator::{
    generate_mask_moves, generate_move_mask_for_bishop, generate_move_mask_for_pawn,
    generate_move_mask_for_rook,
};
use crate::utils::{Move, MoveType, Piece, PieceColor, PieceType, UndoMove};
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
    white_board: u64,
    black_board: u64,
    pawns_board: u64,
    knights_board: u64,
    bishops_board: u64,
    rooks_board: u64,
    queens_board: u64,
    kings_board: u64,

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

        let mut white_board: u64 = 0;
        let mut black_board: u64 = 0;
        let mut pawns_board: u64 = 0;
        let mut knights_board: u64 = 0;
        let mut bishops_board: u64 = 0;
        let mut rooks_board: u64 = 0;
        let mut queens_board: u64 = 0;
        let mut kings_board: u64 = 0;

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
                    pawns_board |= 1u64 << board_index;
                    if ch == 'P' {
                        white_board |= 1u64 << board_index;
                    } else {
                        black_board |= 1u64 << board_index;
                    }
                }

                'N' | 'n' => {
                    knights_board |= 1u64 << board_index;
                    if ch == 'N' {
                        white_board |= 1u64 << board_index;
                    } else {
                        black_board |= 1u64 << board_index;
                    }
                }

                'B' | 'b' => {
                    bishops_board |= 1u64 << board_index;
                    if ch == 'B' {
                        white_board |= 1u64 << board_index;
                    } else {
                        black_board |= 1u64 << board_index;
                    }
                }

                'R' | 'r' => {
                    rooks_board |= 1u64 << board_index;
                    if ch == 'R' {
                        white_board |= 1u64 << board_index;
                    } else {
                        black_board |= 1u64 << board_index;
                    }
                }

                'Q' | 'q' => {
                    queens_board |= 1u64 << board_index;
                    if ch == 'Q' {
                        white_board |= 1u64 << board_index;
                    } else {
                        black_board |= 1u64 << board_index;
                    }
                }

                'K' | 'k' => {
                    kings_board |= 1u64 << board_index;
                    if ch == 'K' {
                        white_board |= 1u64 << board_index;
                    } else {
                        black_board |= 1u64 << board_index;
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
            white_board,
            black_board,
            pawns_board,
            knights_board,
            bishops_board,
            rooks_board,
            queens_board,
            kings_board,
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

    #[inline(always)]
    pub fn is_square_attack_by(&self, index: &i8, attacker_color: &PieceColor) -> bool {
        let board = self.white_board | self.black_board;

        let attacker_board = match attacker_color {
            PieceColor::None => panic!("Invalid color"),
            PieceColor::White => self.white_board,
            PieceColor::Black => self.black_board,
        };
        let attacker_pawns_board = self.pawns_board & attacker_board;
        let attacker_knights_board = self.knights_board & attacker_board;
        let attacker_kings_board = self.kings_board & attacker_board;

        let mut superior_king_mask = lookup_tables::LOOK_UP_TABLE.knight_attacks[*index as usize];
        if superior_king_mask & attacker_knights_board != 0 {
            return true;
        }

        superior_king_mask = lookup_tables::LOOK_UP_TABLE.king_attacks[*index as usize];
        if superior_king_mask & attacker_kings_board != 0 {
            return true;
        }

        superior_king_mask = match attacker_color {
            PieceColor::None => panic!("Invalid color"),
            PieceColor::White => lookup_tables::LOOK_UP_TABLE.black_pawn_attacks[*index as usize],
            PieceColor::Black => lookup_tables::LOOK_UP_TABLE.white_pawn_attacks[*index as usize],
        };
        if superior_king_mask & attacker_pawns_board != 0 {
            return true;
        }

        let superior_bishop_mask = generate_move_mask_for_bishop(&board, &index);
        if superior_bishop_mask & (self.bishops_board | self.queens_board) & attacker_board != 0 {
            return true;
        }

        let superior_rook_mask = generate_move_mask_for_rook(&board, &index);
        if superior_rook_mask & (self.rooks_board | self.queens_board) & attacker_board != 0 {
            return true;
        }

        false
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn make_move(&mut self, mov: &Move, is_intern_move_request: bool) {
        // The verification of the origin of the move request helps to avoid the double mask generation well performing move selection:
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
            piece_moved: source_piece.piece_type,
            piece_captured: destination_piece.piece_type,
            castling_rights: self.castling_rights,
            turn: self.get_turn(),
            en_passant: self.en_passant,
        });
        self.history_index += 1;

        // Putting 0 at the index of the destination
        match destination_piece.piece_type {
            PieceType::None => {}
            PieceType::Pawn => self.pawns_board &= !(1u64 << mov.destination),
            PieceType::Knight => self.knights_board &= !(1u64 << mov.destination),
            PieceType::Bishop => self.bishops_board &= !(1u64 << mov.destination),
            PieceType::Rook => self.rooks_board &= !(1u64 << mov.destination),
            PieceType::Queen => self.queens_board &= !(1u64 << mov.destination),
            PieceType::King => self.kings_board &= !(1u64 << mov.destination),
        }

        // Putting 0 at the index of the source
        // And moving the piece at the destination by putting 1 at the destination for the corresponding piece
        match source_piece.piece_type {
            PieceType::None => {}
            PieceType::Pawn => {
                self.pawns_board &= !(1u64 << mov.source);
                self.pawns_board |= 1u64 << mov.destination;
            }
            PieceType::Knight => {
                self.knights_board &= !(1u64 << mov.source);
                self.knights_board |= 1u64 << mov.destination;
            }
            PieceType::Bishop => {
                self.bishops_board &= !(1u64 << mov.source);
                self.bishops_board |= 1u64 << mov.destination;
            }
            PieceType::Rook => {
                self.rooks_board &= !(1u64 << mov.source);
                self.rooks_board |= 1u64 << mov.destination;
                match source_piece.color {
                    PieceColor::None => {}
                    PieceColor::White => {
                        if mov.source == 7 {
                            self.castling_rights &= !(1u8 << 0);
                        } else if mov.source == 0 {
                            self.castling_rights &= !(1u8 << 1);
                        }
                    }
                    PieceColor::Black => {
                        if mov.source == 63 {
                            self.castling_rights &= !(1u8 << 2);
                        } else if mov.source == 56 {
                            self.castling_rights &= !(1u8 << 3);
                        }
                    }
                }
            }
            PieceType::Queen => {
                self.queens_board &= !(1u64 << mov.source);
                self.queens_board |= 1u64 << mov.destination;
            }
            PieceType::King => {
                self.kings_board &= !(1u64 << mov.source);
                self.kings_board |= 1u64 << mov.destination;
                match source_piece.color {
                    PieceColor::None => {}
                    PieceColor::White => {
                        self.castling_rights &= !(1u8 << 0);
                        self.castling_rights &= !(1u8 << 1);
                    }
                    PieceColor::Black => {
                        self.castling_rights &= !(1u8 << 2);
                        self.castling_rights &= !(1u8 << 3);
                    }
                }
            }
        }

        // Updating the boards (for each color)
        match source_piece.color {
            PieceColor::None => {}
            PieceColor::White => {
                self.white_board &= !(1u64 << mov.source);
                self.white_board |= 1u64 << mov.destination;

                self.black_board &= !(1u64 << mov.destination);
            }
            PieceColor::Black => {
                self.black_board &= !(1u64 << mov.source);
                self.black_board |= 1u64 << mov.destination;

                self.white_board &= !(1u64 << mov.destination);
            }
        };

        // Applying castling and promotions rules
        match mov.move_type {
            MoveType::Normal => {}
            MoveType::ShortCastle => match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.rooks_board &= !(1u64 << 7);
                    self.rooks_board |= 1u64 << 5;

                    self.white_board &= !(1u64 << 7);
                    self.white_board |= 1u64 << 5;
                }
                PieceColor::Black => {
                    self.rooks_board &= !(1u64 << 63);
                    self.rooks_board |= 1u64 << 61;

                    self.black_board &= !(1u64 << 63);
                    self.black_board |= 1u64 << 61;
                }
            },
            MoveType::LongCastle => match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.rooks_board &= !(1u64 << 0);
                    self.rooks_board |= 1u64 << 3;

                    self.white_board &= !(1u64 << 0);
                    self.white_board |= 1u64 << 3;
                }
                PieceColor::Black => {
                    self.rooks_board &= !(1u64 << 56);
                    self.rooks_board |= 1u64 << 59;

                    self.black_board &= !(1u64 << 56);
                    self.black_board |= 1u64 << 59;
                }
            },
            MoveType::PawnToKnight => {
                self.pawns_board &= !(1u64 << mov.destination); // Delete the pawn
                self.knights_board |= 1u64 << mov.destination;
            }
            MoveType::PawnToBishop => {
                self.pawns_board &= !(1u64 << mov.destination); // Delete the pawn
                self.bishops_board |= 1u64 << mov.destination;
            }
            MoveType::PawnToRook => {
                self.pawns_board &= !(1u64 << mov.destination); // Delete the pawn
                self.rooks_board |= 1u64 << mov.destination;
            }
            MoveType::PawnToQueen => {
                self.pawns_board &= !(1u64 << mov.destination); // Delete the pawn
                self.queens_board |= 1u64 << mov.destination;
            }
            MoveType::EnPassant => {
                // Updating the boards (for each color)
                match source_piece.color {
                    PieceColor::None => {}
                    PieceColor::White => {
                        self.pawns_board &= !(1u64 << (mov.destination - 8));
                        self.black_board &= !(1u64 << (mov.destination - 8));
                    }
                    PieceColor::Black => {
                        self.pawns_board &= !(1u64 << (mov.destination + 8));
                        self.white_board &= !(1u64 << (mov.destination + 8));
                    }
                };
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

    #[inline(always)]
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

        // Delete source from the destination
        // And putting back at the source
        match last_move_info.piece_moved {
            PieceType::None => {}
            PieceType::Pawn => {
                self.pawns_board &= !(1u64 << last_move_info.destination);
                self.pawns_board |= 1u64 << last_move_info.source;
            }
            PieceType::Knight => {
                self.knights_board &= !(1u64 << last_move_info.destination);
                self.knights_board |= 1u64 << last_move_info.source;
            }
            PieceType::Bishop => {
                self.bishops_board &= !(1u64 << last_move_info.destination);
                self.bishops_board |= 1u64 << last_move_info.source;
            }
            PieceType::Rook => {
                self.rooks_board &= !(1u64 << last_move_info.destination);
                self.rooks_board |= 1u64 << last_move_info.source;
            }
            PieceType::Queen => {
                self.queens_board &= !(1u64 << last_move_info.destination);
                self.queens_board |= 1u64 << last_move_info.source;
            }
            PieceType::King => {
                self.kings_board &= !(1u64 << last_move_info.destination);
                self.kings_board |= 1u64 << last_move_info.source;
            }
        }

        // Updating the boards (for each color)
        match last_move_info.turn {
            PieceColor::None => {}
            PieceColor::White => {
                self.white_board &= !(1u64 << last_move_info.destination);
                self.white_board |= 1u64 << last_move_info.source;

                match last_move_info.piece_captured {
                    PieceType::None => {}
                    _ => {
                        self.black_board |= 1u64 << last_move_info.destination;
                    }
                }
            }
            PieceColor::Black => {
                self.black_board &= !(1u64 << last_move_info.destination);
                self.black_board |= 1u64 << last_move_info.source;

                match last_move_info.piece_captured {
                    PieceType::None => {}
                    _ => {
                        self.white_board |= 1u64 << last_move_info.destination;
                    }
                }
            }
        };

        // Un-applying castling and promotions rules
        match last_move_info.move_type {
            MoveType::Normal => {}
            MoveType::ShortCastle => match last_move_info.turn {
                PieceColor::None => {}
                PieceColor::White => {
                    self.rooks_board &= !(1u64 << 5);
                    self.rooks_board |= 1u64 << 7;

                    self.white_board &= !(1u64 << 5);
                    self.white_board |= 1u64 << 7;
                }
                PieceColor::Black => {
                    self.rooks_board &= !(1u64 << 61);
                    self.rooks_board |= 1u64 << 63;

                    self.black_board &= !(1u64 << 61);
                    self.black_board |= 1u64 << 63;
                }
            },
            MoveType::LongCastle => match last_move_info.turn {
                PieceColor::None => {}
                PieceColor::White => {
                    self.rooks_board &= !(1u64 << 3);
                    self.rooks_board |= 1u64 << 0;

                    self.white_board &= !(1u64 << 3);
                    self.white_board |= 1u64 << 0;
                }
                PieceColor::Black => {
                    self.rooks_board &= !(1u64 << 59);
                    self.rooks_board |= 1u64 << 56;

                    self.black_board &= !(1u64 << 59);
                    self.black_board |= 1u64 << 56;
                }
            },
            MoveType::PawnToKnight => {
                self.knights_board &= !(1u64 << last_move_info.destination);
            }
            MoveType::PawnToBishop => {
                self.bishops_board &= !(1u64 << last_move_info.destination);
            }
            MoveType::PawnToRook => {
                self.rooks_board &= !(1u64 << last_move_info.destination);
            }
            MoveType::PawnToQueen => {
                self.queens_board &= !(1u64 << last_move_info.destination);
            }
            MoveType::EnPassant => {
                // Updating the boards (for each color)
                match last_move_info.turn {
                    PieceColor::None => {}
                    PieceColor::White => {
                        self.pawns_board |= 1u64 << (last_move_info.destination - 8);
                        self.black_board |= 1u64 << (last_move_info.destination - 8);
                    }
                    PieceColor::Black => {
                        self.pawns_board |= 1u64 << (last_move_info.destination + 8);
                        self.white_board |= 1u64 << (last_move_info.destination + 8);
                    }
                };
            }
        }

        // Putting the captured piece back
        match last_move_info.piece_captured {
            PieceType::None => {}
            PieceType::Pawn => self.pawns_board |= 1u64 << last_move_info.destination,
            PieceType::Knight => self.knights_board |= 1u64 << last_move_info.destination,
            PieceType::Bishop => self.bishops_board |= 1u64 << last_move_info.destination,
            PieceType::Rook => self.rooks_board |= 1u64 << last_move_info.destination,
            PieceType::Queen => self.queens_board |= 1u64 << last_move_info.destination,
            PieceType::King => self.kings_board |= 1u64 << last_move_info.destination,
        }

        self.castling_rights = last_move_info.castling_rights;
        self.en_passant = last_move_info.en_passant;
        self.turn = last_move_info.turn;
    }

    #[inline(always)]
    pub fn get_turn(&self) -> PieceColor {
        self.turn
    }

    #[inline(always)]
    pub fn get_piece_on_square(&self, index: &i8) -> Piece {
        let color = if self.white_board & (1u64 << index) != 0 {
            PieceColor::White
        } else if self.black_board & (1u64 << index) != 0 {
            PieceColor::Black
        } else {
            PieceColor::None
        };

        let piece_type = if self.pawns_board & (1u64 << index) != 0 {
            PieceType::Pawn
        } else if self.knights_board & (1u64 << index) != 0 {
            PieceType::Knight
        } else if self.bishops_board & (1u64 << index) != 0 {
            PieceType::Bishop
        } else if self.rooks_board & (1u64 << index) != 0 {
            PieceType::Rook
        } else if self.queens_board & (1u64 << index) != 0 {
            PieceType::Queen
        } else if self.kings_board & (1u64 << index) != 0 {
            PieceType::King
        } else {
            PieceType::None
        };

        Piece { color, piece_type }
    }

    pub fn get_available_piece_coords(&self, piece_color: &PieceColor) -> [Option<i8>; 64] {
        let mut coords = [None; 64];
        let mut board = match piece_color {
            PieceColor::None => 0,
            PieceColor::White => self.white_board,
            PieceColor::Black => self.black_board,
        };
        let mut cursor = 0;
        while board != 0 {
            coords[cursor] = Some(board.trailing_zeros() as i8);
            board &= board - 1;
            cursor += 1;
        }
        coords
    }

    #[inline(always)]
    pub fn get_king_coord(&self, color: &PieceColor) -> i8 {
        if *color == PieceColor::None {
            panic!("Trying to get a king with the color None")
        }
        match color {
            PieceColor::None => {
                panic!("Invalid color")
            }
            PieceColor::White => (self.white_board & self.kings_board).trailing_zeros() as i8,
            PieceColor::Black => (self.black_board & self.kings_board).trailing_zeros() as i8,
        }
    }

    #[inline(always)]
    pub fn get_board(&self) -> u64 {
        self.white_board | self.black_board
    }

    #[inline(always)]
    pub fn get_white_board(&self) -> u64 {
        self.white_board
    }

    #[inline(always)]
    pub fn get_black_board(&self) -> u64 {
        self.black_board
    }

    #[inline(always)]
    pub fn get_pawns_board(&self) -> u64 {
        self.pawns_board
    }

    #[inline(always)]
    pub fn get_knight_board(&self) -> u64 {
        self.knights_board
    }

    #[inline(always)]
    pub fn get_bishops_board(&self) -> u64 {
        self.bishops_board
    }

    #[inline(always)]
    pub fn get_rook_board(&self) -> u64 {
        self.rooks_board
    }
    pub fn get_queens_board(&self) -> u64 {
        self.queens_board
    }

    #[inline(always)]
    pub fn can_short_castle(&self, color: &PieceColor) -> bool {
        let board = self.white_board | self.black_board;
        let king_index = self.get_king_coord(color);

        match color {
            PieceColor::None => false,
            PieceColor::White => {
                (self.castling_rights >> 0 & 1) == 1
                    && (self.white_board & self.rooks_board & (1u64 << 7)) != 0
                    && (board & (1u64 << (king_index + 1))) == 0
                    && (board & (1u64 << (king_index + 2))) == 0
                    && !self.is_square_attack_by(&king_index, &PieceColor::Black)
                    && !self.is_square_attack_by(&(king_index + 1), &PieceColor::Black)
                    && !self.is_square_attack_by(&(king_index + 2), &PieceColor::Black)
            }
            PieceColor::Black => {
                (self.castling_rights >> 2 & 1) == 1
                    && (self.black_board & self.rooks_board & (1u64 << 63)) != 0
                    && (board & (1u64 << (king_index + 1))) == 0
                    && (board & (1u64 << (king_index + 2))) == 0
                    && !self.is_square_attack_by(&king_index, &PieceColor::White)
                    && !self.is_square_attack_by(&(king_index + 1), &PieceColor::White)
                    && !self.is_square_attack_by(&(king_index + 2), &PieceColor::White)
            }
        }
    }

    #[inline(always)]
    pub fn can_long_castle(&self, color: &PieceColor) -> bool {
        let board = self.white_board | self.black_board;
        let king_index = self.get_king_coord(color);

        match color {
            PieceColor::None => false,
            PieceColor::White => {
                (self.castling_rights >> 1 & 1) == 1
                    && (self.white_board & self.rooks_board & (1u64 << 0)) != 0
                    && (board & (1u64 << (king_index - 1))) == 0
                    && (board & (1u64 << (king_index - 2))) == 0
                    && (board & (1u64 << (king_index - 3))) == 0
                    && !self.is_square_attack_by(&king_index, &PieceColor::Black)
                    && !self.is_square_attack_by(&(king_index - 1), &PieceColor::Black)
                    && !self.is_square_attack_by(&(king_index - 2), &PieceColor::Black)
            }
            PieceColor::Black => {
                (self.castling_rights >> 3 & 1) == 1
                    && (self.black_board & self.rooks_board & (1u64 << 56)) != 0
                    && (board & (1u64 << (king_index - 1))) == 0
                    && (board & (1u64 << (king_index - 2))) == 0
                    && (board & (1u64 << (king_index - 3))) == 0
                    && !self.is_square_attack_by(&king_index, &PieceColor::White)
                    && !self.is_square_attack_by(&(king_index - 1), &PieceColor::White)
                    && !self.is_square_attack_by(&(king_index - 2), &PieceColor::White)
            }
        }
    }

    #[inline(always)]
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

    fn piece_to_unicode(&self, piece: &Piece) -> char {
        match (piece.color, piece.piece_type) {
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
