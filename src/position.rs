use crate::history::{History, UndoMove};
use crate::moves::{Move, MoveType};
use crate::moves_generator::{generate_move_mask_for_bishop, generate_move_mask_for_rook};
use crate::piece::{Piece, PieceColor, PieceType};
use crate::utils::{BLACK_PAWNS_ATTACKS, KING_ATTACKS, KNIGHT_ATTACKS, WHITE_PAWNS_ATTACKS, ZOBRIST_POSITION_KEYS, ZOBRIST_CASTLING_RIGHTS_KEYS, ZOBRIST_EN_PASSANT_FILES_KEYS, ZOBRIST_SIDE_KEY};

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

    white_king_square: u8,
    black_king_square: u8,
    castling_rights: u8, // 0 0 0 0 0(q) 0(k) 0(Q) 0(K)
    en_passant_file: u8,

    turn: PieceColor,
    number_of_move: u8,
    half_move_clock: u8,

    hash: u64,
    history: [UndoMove; 1usize << 15],
    history_count: usize,
}

impl Position {
    pub fn from_fen(fen: &str) -> Position {
        let mut board_index: u64 = 56;

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
                    board_index += skip as u64;
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

        let mut en_passant_file: u8 = 8;
        if en_passant_part != "-" {
            for ch in en_passant_part.chars() {
                match ch {
                    'a'..='h' => en_passant_file = ch as u8 - 'a' as u8,
                    _ => {}
                }
            }
        }

        let mut hash: u64 = 0;
        let boards: [u64; 6] = [pawns_board, knights_board, bishops_board, rooks_board, queens_board, kings_board];
        for piece in 0..6 {
            let mut board = boards[piece] & white_board;
            while board != 0 {
                hash ^= ZOBRIST_POSITION_KEYS[piece * 64 + board.trailing_zeros() as usize];
                board &= board - 1;
            }

            board = boards[piece] & black_board;
            while board != 0 {
                hash ^= ZOBRIST_POSITION_KEYS[(6 + piece) * 64 + board.trailing_zeros() as usize];
                board &= board - 1;
            }
        }

        hash ^= ZOBRIST_CASTLING_RIGHTS_KEYS[castling_rights as usize];

        hash ^= ZOBRIST_EN_PASSANT_FILES_KEYS[en_passant_file as usize];

        if turn == PieceColor::Black { hash ^= ZOBRIST_SIDE_KEY };


        Position {
            white_board,
            black_board,
            pawns_board,
            knights_board,
            bishops_board,
            rooks_board,
            queens_board,
            kings_board,
            white_king_square: (white_board & kings_board).trailing_zeros() as u8,
            black_king_square: (black_board & kings_board).trailing_zeros() as u8,
            castling_rights,
            en_passant_file,
            turn,
            number_of_move: number_of_moves_move_part.parse().unwrap(),
            half_move_clock: half_move_part.parse().unwrap(),
            hash,
            history: [UndoMove::default(); 1usize << 15],
            history_count: 0,
        }
    }

    #[inline(always)]
    pub fn make_move(&mut self, mov: &Move) {
        let source: u8 = mov.source();
        let destination: u8 = mov.destination();
        let move_type = mov.move_type();

        let source_mask = 1u64 << source;
        let destination_mask = 1u64 << destination;
        let source_piece = self.get_piece_on_square(&source);
        let destination_piece = self.get_piece_on_square(&destination);

        let old_castling_rights = self.castling_rights;

        self.history[self.history_count] = UndoMove::new(self);
        self.history_count += 1;

        self.hash ^= ZOBRIST_SIDE_KEY;

        // Putting 0 at the index of the destination
        match destination_piece.piece_type {
            PieceType::Pawn => {
                self.pawns_board &= !destination_mask;
                self.hash ^= ZOBRIST_POSITION_KEYS[destination_piece.to_usize() * 64 + destination as usize];
            }
            PieceType::Knight => {
                self.knights_board &= !destination_mask;
                self.hash ^= ZOBRIST_POSITION_KEYS[destination_piece.to_usize() * 64 + destination as usize];
            }
            PieceType::Bishop => {
                self.bishops_board &= !destination_mask;
                self.hash ^= ZOBRIST_POSITION_KEYS[destination_piece.to_usize() * 64 + destination as usize];
            }
            PieceType::Rook => {
                self.rooks_board &= !destination_mask;
                self.hash ^= ZOBRIST_POSITION_KEYS[destination_piece.to_usize() * 64 + destination as usize];

                if destination == 7 {
                    self.castling_rights &= 0b11111110;
                } else if destination == 0 {
                    self.castling_rights &= 0b11111101;
                } else if destination == 63 {
                    self.castling_rights &= 0b11111011;
                } else if destination == 56 {
                    self.castling_rights &= 0b11110111;
                }
            }
            PieceType::Queen => {
                self.queens_board &= !destination_mask;
                self.hash ^= ZOBRIST_POSITION_KEYS[destination_piece.to_usize() * 64 + destination as usize];
            }
            PieceType::King => panic!("King cannot be capture"),
            _ => {}
        }


        self.hash ^= ZOBRIST_POSITION_KEYS[source_piece.to_usize() * 64 + source as usize];
        self.hash ^= ZOBRIST_POSITION_KEYS[source_piece.to_usize() * 64 + destination as usize];

        // Putting 0 at the index of the source
        // And moving the piece at the destination by putting 1 at the destination for the corresponding piece
        match source_piece.piece_type {
            PieceType::Pawn => self.pawns_board ^= source_mask | destination_mask,
            PieceType::Knight => self.knights_board ^= source_mask | destination_mask,
            PieceType::Bishop => self.bishops_board ^= source_mask | destination_mask,
            PieceType::Queen => self.queens_board ^= source_mask | destination_mask,
            PieceType::Rook => {
                self.rooks_board ^= source_mask | destination_mask;
                if source == 7 {
                    self.castling_rights &= 0b11111110;
                }
                if source == 0 {
                    self.castling_rights &= 0b11111101;
                }
                if source == 63 {
                    self.castling_rights &= 0b11111011;
                }
                if source == 56 {
                    self.castling_rights &= 0b11110111;
                }
            }
            PieceType::King => {
                self.kings_board ^= source_mask | destination_mask;
                match source_piece.color {
                    PieceColor::None => {}
                    PieceColor::White => {
                        self.white_king_square = destination;
                        self.castling_rights &= 0b11111100;
                    }
                    PieceColor::Black => {
                        self.black_king_square = destination;
                        self.castling_rights &= 0b11110011;
                    }
                }
            }
            _ => {}
        }

        self.hash ^= ZOBRIST_CASTLING_RIGHTS_KEYS[old_castling_rights as usize];
        self.hash ^= ZOBRIST_CASTLING_RIGHTS_KEYS[self.castling_rights as usize];

        self.hash ^= ZOBRIST_EN_PASSANT_FILES_KEYS[self.en_passant_file as usize];

        // Updating the boards (for each color)
        match source_piece.color {
            PieceColor::White => {
                self.white_board ^= source_mask | destination_mask;
                self.black_board &= !destination_mask;
            }
            PieceColor::Black => {
                self.black_board ^= source_mask | destination_mask;
                self.white_board &= !destination_mask;
            }
            _ => {}
        };

        // Applying castling and promotions rules
        if move_type == MoveType::ShortCastle {
            // ShortCastle
            match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.rooks_board ^= 160; // 1u64 << 7 | 1u64 << 5
                    self.white_board ^= 160; // 1u64 << 7 | 1u64 << 5

                    // Piece { color: PieceColor::White, piece_type: PieceType::Rook }.to_usize() == 3
                    self.hash ^= ZOBRIST_POSITION_KEYS[3 * 64 + 7];
                    self.hash ^= ZOBRIST_POSITION_KEYS[3 * 64 + 5];
                }
                PieceColor::Black => {
                    self.rooks_board ^= 11529215046068469760; // 1u64 << 63 | 1u64 << 61
                    self.black_board ^= 11529215046068469760; // 1u64 << 63 | 1u64 << 61

                    // Piece { color: PieceColor::Black, piece_type: PieceType::Rook }.to_usize() == 9
                    self.hash ^= ZOBRIST_POSITION_KEYS[9 * 64 + 63];
                    self.hash ^= ZOBRIST_POSITION_KEYS[9 * 64 + 61];
                }
            }
        } else if move_type == MoveType::LongCastle {
            // LongCastle
            match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.rooks_board ^= 9; // 1u64 << 0 | 1u64 << 3
                    self.white_board ^= 9; // 1u64 << 0 | 1u64 << 3

                    // Piece { color: PieceColor::White, piece_type: PieceType::Rook }.to_usize() == 3
                    self.hash ^= ZOBRIST_POSITION_KEYS[3 * 64 + 0];
                    self.hash ^= ZOBRIST_POSITION_KEYS[3 * 64 + 3];
                }
                PieceColor::Black => {
                    self.rooks_board ^= 648518346341351424; // 1u64 << 56 | 1u64 << 59
                    self.black_board ^= 648518346341351424; // 1u64 << 56 | 1u64 << 59

                    // Piece { color: PieceColor::Black, piece_type: PieceType::Rook }.to_usize() == 9
                    self.hash ^= ZOBRIST_POSITION_KEYS[9 * 64 + 56];
                    self.hash ^= ZOBRIST_POSITION_KEYS[9 * 64 + 59];
                }
            }
        } else if move_type == MoveType::EnPassant {
            // Updating the boards (for each color)
            match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.pawns_board &= !(1u64 << (destination - 8));
                    self.black_board &= !(1u64 << (destination - 8));

                    self.hash ^= ZOBRIST_POSITION_KEYS[6 * 64 + destination as usize - 8];
                }
                PieceColor::Black => {
                    self.pawns_board &= !(1u64 << (destination + 8));
                    self.white_board &= !(1u64 << (destination + 8));

                    self.hash ^= ZOBRIST_POSITION_KEYS[0 * 64 + destination as usize + 8];
                }
            };
        } else {
            let mut promotion_index: usize = 0;
            if move_type == MoveType::PawnToKnight {
                self.pawns_board &= !destination_mask;
                self.knights_board |= destination_mask;
                promotion_index = 1;
            } else if move_type == MoveType::PawnToBishop {
                self.pawns_board &= !destination_mask;
                self.bishops_board |= destination_mask;
                promotion_index = 2;
            } else if move_type == MoveType::PawnToRook {
                self.pawns_board &= !destination_mask;
                self.rooks_board |= destination_mask;
                promotion_index = 3;
            } else if move_type == MoveType::PawnToQueen {
                self.pawns_board &= !destination_mask;
                self.queens_board |= destination_mask;
                promotion_index = 4;
            }

            // WHITE_PAWN or BLACK_PAWN
            let pawn: usize = (self.turn.to_u8() * 6) as usize;
            self.hash ^= ZOBRIST_POSITION_KEYS[pawn * 64 + destination as usize] ^ ZOBRIST_POSITION_KEYS[(pawn + promotion_index) * 64 + destination as usize];
        }

        self.en_passant_file = 8;
        if source_piece.piece_type == PieceType::Pawn && source.abs_diff(destination) == 16 {
            match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.en_passant_file = (destination - 8) & 7;
                }
                PieceColor::Black => {
                    self.en_passant_file = (destination + 8) & 7;
                }
            }

            self.hash ^= ZOBRIST_EN_PASSANT_FILES_KEYS[self.en_passant_file as usize];
        }

        self.turn = self.turn.opposite();
    }

    #[inline(always)]
    pub fn undo_last_move(&mut self) {
        self.history_count -= 1;
        let last_move_info = &self.history[self.history_count];

        self.white_board = last_move_info.white_board;
        self.black_board = last_move_info.black_board;

        self.pawns_board = last_move_info.pawns_board;
        self.knights_board = last_move_info.knights_board;
        self.bishops_board = last_move_info.bishops_board;
        self.rooks_board = last_move_info.rooks_board;
        self.queens_board = last_move_info.queens_board;
        self.kings_board = last_move_info.kings_board;

        self.hash = last_move_info.hash;

        self.turn = last_move_info.turn;
        self.castling_rights = last_move_info.castling_rights;
        self.white_king_square = last_move_info.white_king_square;
        self.black_king_square = last_move_info.black_king_square;
        self.en_passant_file = last_move_info.en_passant_file;
        self.half_move_clock = last_move_info.half_move_clock;
    }

    #[inline(always)]
    pub fn is_square_attack_by(&self, square: &u8, attacker_color: &PieceColor) -> bool {
        let board = self.white_board | self.black_board;

        let attacker_board = match attacker_color {
            PieceColor::None => panic!("Invalid color"),
            PieceColor::White => self.white_board,
            PieceColor::Black => self.black_board,
        };

        let mut superior_king_mask = KNIGHT_ATTACKS[*square as usize];
        if superior_king_mask & self.knights_board & attacker_board != 0 {
            return true;
        }

        superior_king_mask = KING_ATTACKS[*square as usize];
        if superior_king_mask & self.kings_board & attacker_board != 0 {
            return true;
        }

        superior_king_mask = match attacker_color {
            PieceColor::None => panic!("Invalid color"),
            PieceColor::White => BLACK_PAWNS_ATTACKS[*square as usize],
            PieceColor::Black => WHITE_PAWNS_ATTACKS[*square as usize],
        };
        if superior_king_mask & self.pawns_board & attacker_board != 0 {
            return true;
        }

        let superior_bishop_mask = generate_move_mask_for_bishop(&board, &square);
        if superior_bishop_mask & (self.bishops_board | self.queens_board) & attacker_board != 0 {
            return true;
        }

        let superior_rook_mask = generate_move_mask_for_rook(&board, &square);
        if superior_rook_mask & (self.rooks_board | self.queens_board) & attacker_board != 0 {
            return true;
        }

        false
    }

    #[inline(always)]
    pub fn is_check(&self, color: &PieceColor) -> bool {
        self.is_square_attack_by(
            &match color {
                PieceColor::White => self.white_king_square,
                PieceColor::Black => self.black_king_square,
                PieceColor::None => panic!("Invalid color"),
            },
            &color.opposite(),
        )
    }

    #[inline(always)]
    pub fn get_turn(&self) -> PieceColor { self.turn }


    #[inline(always)]
    pub fn get_piece_on_square(&self, square: &u8) -> Piece {
        let square_mask = 1u64 << square;
        let color = if self.white_board & square_mask != 0 {
            PieceColor::White
        } else if self.black_board & square_mask != 0 {
            PieceColor::Black
        } else {
            // If the square has no color, there is no piece on it
            return Piece {
                color: PieceColor::None,
                piece_type: PieceType::None,
            };
        };

        let piece_type = if self.pawns_board & square_mask != 0 {
            PieceType::Pawn
        } else if self.knights_board & square_mask != 0 {
            PieceType::Knight
        } else if self.bishops_board & square_mask != 0 {
            PieceType::Bishop
        } else if self.rooks_board & square_mask != 0 {
            PieceType::Rook
        } else if self.queens_board & square_mask != 0 {
            PieceType::Queen
        } else if self.kings_board & square_mask != 0 {
            PieceType::King
        } else {
            PieceType::None
        };

        Piece { color, piece_type }
    }

    #[inline(always)]
    pub fn get_white_king_square(&self) -> u8 { self.white_king_square }

    #[inline(always)]
    pub fn get_black_king_square(&self) -> u8 { self.black_king_square }

    #[inline(always)]
    pub fn get_repetition_count_for_current_position(&self) -> usize {
        // self.history.iter().take(self.history_count).filter(|k| (**k).hash == self.hash).count()
        let mut count = 0;

        for i in 0..self.history_count {
            if self.history[i].hash == self.hash {
                count += 1;
            }
        }

        count
    }

    #[inline(always)]
    pub fn get_board(&self) -> u64 { self.white_board | self.black_board }

    #[inline(always)]
    pub fn get_white_board(&self) -> u64 { self.white_board }

    #[inline(always)]
    pub fn get_black_board(&self) -> u64 { self.black_board }

    #[inline(always)]
    pub fn get_pawns_board(&self) -> u64 { self.pawns_board }

    #[inline(always)]
    pub fn get_knights_board(&self) -> u64 { self.knights_board }

    #[inline(always)]
    pub fn get_bishops_board(&self) -> u64 { self.bishops_board }

    #[inline(always)]
    pub fn get_rooks_board(&self) -> u64 { self.rooks_board }
    #[inline(always)]
    pub fn get_queens_board(&self) -> u64 { self.queens_board }
    #[inline(always)]
    pub fn get_kings_board(&self) -> u64 { self.kings_board }

    #[inline(always)]
    pub fn can_white_short_castle(&self) -> bool {
        let board = self.white_board | self.black_board;
        (self.castling_rights & 0b00000001) != 0
            && (board & (32 | 64)) == 0 // 1u64 << (5 | 6)
            && !self.is_square_attack_by(&4, &PieceColor::Black)
            && !self.is_square_attack_by(&5, &PieceColor::Black)
            && !self.is_square_attack_by(&6, &PieceColor::Black)
    }

    #[inline(always)]
    pub fn can_black_short_castle(&self) -> bool {
        let board = self.white_board | self.black_board;
        (self.castling_rights & 0b00000100) != 0
            && (board & (2305843009213693952u64 | 4611686018427387904u64)) == 0 // 1u64 << (61 | 62)
            && !self.is_square_attack_by(&60, &PieceColor::White)
            && !self.is_square_attack_by(&61, &PieceColor::White)
            && !self.is_square_attack_by(&62, &PieceColor::White)
    }

    #[inline(always)]
    pub fn can_white_long_castle(&self) -> bool {
        let board = self.white_board | self.black_board;
        (self.castling_rights & 0b00000010) != 0
            && (board & (8 | 4 | 2)) == 0 // 1u64 << (3 | 2 | 1)
            && !self.is_square_attack_by(&4, &PieceColor::Black)
            && !self.is_square_attack_by(&3, &PieceColor::Black)
            && !self.is_square_attack_by(&2, &PieceColor::Black)
    }

    #[inline(always)]
    pub fn can_black_long_castle(&self) -> bool {
        let board = self.white_board | self.black_board;
        (self.castling_rights & 0b00001000) != 0
            && (board & (576460752303423488u64 | 288230376151711744u64 | 144115188075855872u64)) == 0 // 1u64 << (59 | 58 | 57)
            && !self.is_square_attack_by(&60, &PieceColor::White)
            && !self.is_square_attack_by(&59, &PieceColor::White)
            && !self.is_square_attack_by(&58, &PieceColor::White)
    }

    #[inline(always)]
    pub fn get_en_passant(&self) -> u8 {
        if self.en_passant_file < 8 {
            match self.turn {
                PieceColor::White => {
                    5 * 8 + self.en_passant_file
                }
                PieceColor::Black => {
                    2 * 8 + self.en_passant_file
                }
                PieceColor::None => { 64 }
            }
        } else { 64 }
    }

    #[inline(always)]
    pub fn get_hash(&self) -> u64 { self.hash }

    #[inline(always)]
    pub fn get_castling_rights(&self) -> u8 { self.castling_rights }

    #[inline(always)]
    pub fn get_number_of_move(&self) -> u8 { self.number_of_move }

    #[inline(always)]
    pub fn get_half_move_clock(&self) -> u8 { self.half_move_clock }

    pub fn print_board(&self) {
        for rank in (0..=7).rev() {
            print!("{} ", rank + 1);
            for file in 0..=7 {
                let index = (rank * 8 + file as usize) as u8;
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
