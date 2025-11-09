use crate::moves::{Move, MoveType};
use crate::moves_generator::{generate_move_mask_for_bishop, generate_move_mask_for_rook};
use crate::utils::{BLACK_PAWNS_ATTACKS, KING_ATTACKS, KNIGHT_ATTACKS, Piece, PieceColor, PieceType, WHITE_PAWNS_ATTACKS};

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

    white_king_coord: u8,
    black_king_coord: u8,

    turn: PieceColor,
    number_of_move: u8,
    half_move_clock: u8,
    castling_rights: u8, // 0 0 0 0 0(q) 0(k) 0(Q) 0(K)
    en_passant: u8,
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

        let mut en_passant_rank: Option<u8> = None;
        let mut en_passant_file: Option<u8> = None;
        if en_passant_part != "-" {
            for ch in en_passant_part.chars() {
                match ch {
                    'a'..='z' => en_passant_file = Some(ch as u8 - 'a' as u8),
                    '1'..='8' => en_passant_rank = Some((ch.to_digit(10).unwrap() as u8 - 1) * 8),
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
            white_king_coord: (white_board & kings_board).trailing_zeros() as u8,
            black_king_coord: (black_board & kings_board).trailing_zeros() as u8,
            turn,
            number_of_move: number_of_moves_move_part.parse().unwrap(),
            half_move_clock: half_move_part.parse().unwrap(),
            castling_rights,
            en_passant: if en_passant_rank.is_some() && en_passant_file.is_some() {
                en_passant_rank.unwrap() + en_passant_file.unwrap()
            } else {
                255
            },
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

        // Putting 0 at the index of the destination
        match destination_piece.piece_type {
            PieceType::Pawn => self.pawns_board &= !destination_mask,
            PieceType::Knight => self.knights_board &= !destination_mask,
            PieceType::Bishop => self.bishops_board &= !destination_mask,
            PieceType::Rook => {
                self.rooks_board &= !destination_mask;

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
            PieceType::Queen => self.queens_board &= !destination_mask,
            PieceType::King => self.kings_board &= !destination_mask,
            _ => {}
        }

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
                        self.white_king_coord = destination;
                        self.castling_rights &= 0b11111100;
                    }
                    PieceColor::Black => {
                        self.black_king_coord = destination;
                        self.castling_rights &= 0b11110011;
                    }
                }
            }
            _ => {}
        }

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
                }
                PieceColor::Black => {
                    self.rooks_board ^= 11529215046068469760; // 1u64 << 63 | 1u64 << 61
                    self.black_board ^= 11529215046068469760; // 1u64 << 63 | 1u64 << 61
                }
            }
        } else if move_type == MoveType::LongCastle {
            // LongCastle
            match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.rooks_board ^= 9; // 1u64 << 0 | 1u64 << 3
                    self.white_board ^= 9; // 1u64 << 0 | 1u64 << 3
                }
                PieceColor::Black => {
                    self.rooks_board ^= 648518346341351424; // 1u64 << 56 | 1u64 << 59
                    self.black_board ^= 648518346341351424; // 1u64 << 56 | 1u64 << 59
                }
            }
        } else if move_type == MoveType::PawnToKnight {
            self.pawns_board &= !destination_mask;
            self.knights_board |= destination_mask;
        } else if move_type == MoveType::PawnToBishop {
            self.pawns_board &= !destination_mask;
            self.bishops_board |= destination_mask;
        } else if move_type == MoveType::PawnToRook {
            self.pawns_board &= !destination_mask;
            self.rooks_board |= destination_mask;
        } else if move_type == MoveType::PawnToQueen {
            self.pawns_board &= !destination_mask;
            self.queens_board |= destination_mask;
        } else if move_type == MoveType::EnPassant {
            // Updating the boards (for each color)
            match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.pawns_board &= !(1u64 << (destination - 8));
                    self.black_board &= !(1u64 << (destination - 8));
                }
                PieceColor::Black => {
                    self.pawns_board &= !(1u64 << (destination + 8));
                    self.white_board &= !(1u64 << (destination + 8));
                }
            };
        }

        self.en_passant = 255;
        if source_piece.piece_type == PieceType::Pawn && source.abs_diff(destination) == 16 {
            match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    self.en_passant = destination - 8;
                }
                PieceColor::Black => {
                    self.en_passant = destination + 8;
                }
            }
        }

        self.turn = self.turn.opposite();
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
                PieceColor::White => self.white_king_coord,
                PieceColor::Black => self.black_king_coord,
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
    pub fn get_king_coord(&self, color: &PieceColor) -> u8 {
        match color {
            PieceColor::White => self.white_king_coord,
            PieceColor::Black => self.black_king_coord,
            PieceColor::None => panic!("Invalid color"),
        }
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
    pub fn get_knight_board(&self) -> u64 { self.knights_board }

    #[inline(always)]
    pub fn get_bishops_board(&self) -> u64 { self.bishops_board }

    #[inline(always)]
    pub fn get_rook_board(&self) -> u64 { self.rooks_board }
    #[inline(always)]
    pub fn get_queens_board(&self) -> u64 { self.queens_board }
    #[inline(always)]
    pub fn get_king_board(&self) -> u64 { self.kings_board }

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
    pub fn get_en_passant(&self) -> u8 { self.en_passant }

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
