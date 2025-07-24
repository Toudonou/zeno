use crate::utils::{
    ANTI_DIAGONAL, Coord, DIAGONAL, FILE_A, Move, MoveType, Piece, PieceColor, PieceType, RANK_1,
};
use crate::{lookup_tables, moves_generator};
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
    number_of_move: u32,
    // Clearly not the best way to keep track of castling rights
    white_rook_king_side_moves: u32,
    white_rook_queen_side_moves: u32,
    white_king_moves: u32,
    black_rook_king_side_moves: u32,
    black_rook_queen_side_moves: u32,
    black_king_moves: u32,
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

        let mut white_can_castle_kingside = false;
        let mut white_can_castle_queenside = false;
        let mut black_can_castle_kingside = false;
        let mut black_can_castle_queenside = false;

        if castling_part != "-" {
            for ch in castling_part.chars() {
                match ch {
                    'K' => white_can_castle_kingside = true,
                    'Q' => white_can_castle_queenside = true,
                    'k' => black_can_castle_kingside = true,
                    'q' => black_can_castle_queenside = true,
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
            white_rook_king_side_moves: if white_can_castle_kingside { 0 } else { 1 },
            white_rook_queen_side_moves: if white_can_castle_queenside { 0 } else { 1 },
            white_king_moves: if white_can_castle_kingside | white_can_castle_queenside {
                0
            } else {
                1
            },

            black_rook_king_side_moves: if black_can_castle_kingside { 0 } else { 1 },
            black_rook_queen_side_moves: if black_can_castle_queenside { 0 } else { 1 },
            black_king_moves: if black_can_castle_kingside | black_can_castle_queenside {
                0
            } else {
                1
            },
        }
    }

    pub fn is_legal_move(&self, mov: &Move) -> bool {
        // TODO: Pawn moves (Promotion(avoid having a pawn on the fist and the last rank) and en-passant)
        // TODO: Can not castle if in check And promotions moves

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
            MoveType::Normal => {
                // The destination square must appear as one the square that the attacker piece can reach
                let r = mov.destination.rank - 1;
                let f = mov.destination.file as u8 - 'a' as u8;
                let destination_index = r * 8 + f as i8;
                let attacks_squares = moves_generator::generate_mask_moves(
                    &self.white_board,
                    &self.black_board,
                    &mov.source,
                    &source_piece,
                );
                if (attacks_squares >> destination_index) & 1 == 0 {
                    return false;
                }

                true
            }
            MoveType::ShortCastle => {
                let board = self.white_board | self.black_board;
                let king_index =
                    (mov.source.rank - 1) * 8 + (mov.source.file as u8 - 'a' as u8) as i8;
                source_piece.piece_type == PieceType::King
                    && self.can_short_castle(&self.turn)
                    && ((mov.source.file as u8).abs_diff(mov.destination.file as u8) == 2)
                    && ((board >> (king_index + 1) & 1) == 0)
                    && ((board >> (king_index + 2) & 1) == 0)
            }
            MoveType::LongCastle => {
                let board = self.white_board | self.black_board;
                let king_index =
                    (mov.source.rank - 1) * 8 + (mov.source.file as u8 - 'a' as u8) as i8;
                source_piece.piece_type == PieceType::King
                    && self.can_long_castle(&self.turn)
                    && ((mov.source.file as u8).abs_diff(mov.destination.file as u8) == 2)
                    && ((board >> (king_index - 1) & 1) == 0)
                    && ((board >> (king_index - 2) & 1) == 0)
            }
            MoveType::PawnToKnight => true,
            MoveType::PawnToBishop => true,
            MoveType::PawnToRook => true,
            MoveType::PawnToQueen => true,
        }
    }

    pub fn make_move(&mut self, mov: &Move, is_intern_move_request: bool) {
        // The verification of the origin of the move request helps to avoid the double mask generation well performing move selection:
        // The function generate_mask in move_generator will be call for move generation and for the verification of the move
        // That is unnecessary and may slow down the process
        if !is_intern_move_request && !self.is_legal_move(&mov) {
            println!("Illegal move {:?}", mov);
            return;
        }

        let source_piece = self.get_piece_on_square(&mov.source);
        let destination_piece = self.get_piece_on_square(&mov.destination);

        let source_index = (mov.source.rank - 1) * 8 + (mov.source.file as u8 - 'a' as u8) as i8;
        let destination_index =
            (mov.destination.rank - 1) * 8 + (mov.destination.file as u8 - 'a' as u8) as i8;

        // Putting 0 at the index of the destination
        match destination_piece.piece_type {
            PieceType::None => {}
            PieceType::Pawn => self.pawns_board &= !(1u64 << destination_index),
            PieceType::Knight => self.knights_board &= !(1u64 << destination_index),
            PieceType::Bishop => self.bishops_board &= !(1u64 << destination_index),
            PieceType::Rook => self.rooks_board &= !(1u64 << destination_index),
            PieceType::Queen => self.queens_board &= !(1u64 << destination_index),
            PieceType::King => self.kings_board &= !(1u64 << destination_index),
        }

        // Putting 0 at the index of the source
        // And moving the piece at the destination by putting 1 at the destination for the corresponding piece
        match source_piece.piece_type {
            PieceType::None => {}
            PieceType::Pawn => {
                self.pawns_board &= !(1u64 << source_index);
                self.pawns_board |= 1u64 << destination_index;
            }
            PieceType::Knight => {
                self.knights_board &= !(1u64 << source_index);
                self.knights_board |= 1u64 << destination_index;
            }
            PieceType::Bishop => {
                self.bishops_board &= !(1u64 << source_index);
                self.bishops_board |= 1u64 << destination_index;
            }
            PieceType::Rook => {
                self.rooks_board &= !(1u64 << source_index);
                self.rooks_board |= 1u64 << destination_index;
                match source_piece.color {
                    PieceColor::None => {}
                    PieceColor::White => {
                        if source_index == 7 {
                            self.white_rook_king_side_moves += 1;
                        } else if source_index == 0 {
                            self.white_rook_queen_side_moves += 1;
                        }
                    }
                    PieceColor::Black => {
                        if source_index == 63 {
                            self.black_rook_king_side_moves += 1;
                        } else if source_index == 56 {
                            self.black_rook_queen_side_moves += 1;
                        }
                    }
                }
            }
            PieceType::Queen => {
                self.queens_board &= !(1u64 << source_index);
                self.queens_board |= 1u64 << destination_index;
            }
            PieceType::King => {
                self.kings_board &= !(1u64 << source_index);
                self.kings_board |= 1u64 << destination_index;
                match source_piece.color {
                    PieceColor::None => {}
                    PieceColor::White => {
                        if source_index == 4 {
                            self.white_rook_king_side_moves += 1;
                        }
                    }
                    PieceColor::Black => {
                        if source_index == 60 {
                            self.black_rook_king_side_moves += 1;
                        }
                    }
                }
            }
        }

        // Updating the boards (for each color)
        match source_piece.color {
            PieceColor::None => {}
            PieceColor::White => {
                self.white_board &= !(1u64 << source_index);
                self.white_board |= 1u64 << destination_index;

                self.black_board &= !(1u64 << destination_index);
            }
            PieceColor::Black => {
                self.black_board &= !(1u64 << source_index);
                self.black_board |= 1u64 << destination_index;

                self.white_board &= !(1u64 << destination_index);
            }
        };

        // Applying castling and promotions rules
        match mov.move_type {
            MoveType::Normal => {}
            MoveType::ShortCastle => match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    let rook_source = (1 - 1) * 8 + ('h' as u8 - 'a' as u8) as i8;
                    let rook_destination = (1 - 1) * 8 + ('f' as u8 - 'a' as u8) as i8;

                    self.rooks_board &= !(1u64 << rook_source);
                    self.rooks_board |= 1u64 << rook_destination;

                    self.white_board &= !(1u64 << rook_source);
                    self.white_board |= 1u64 << rook_destination;
                }
                PieceColor::Black => {
                    let rook_source = (8 - 1) * 8 + ('h' as u8 - 'a' as u8) as i8;
                    let rook_destination = (8 - 1) * 8 + ('f' as u8 - 'a' as u8) as i8;

                    self.rooks_board &= !(1u64 << rook_source);
                    self.rooks_board |= 1u64 << rook_destination;

                    self.black_board &= !(1u64 << rook_source);
                    self.black_board |= 1u64 << rook_destination;
                }
            },
            MoveType::LongCastle => match source_piece.color {
                PieceColor::None => {}
                PieceColor::White => {
                    let rook_source = (1 - 1) * 8 + ('a' as u8 - 'a' as u8) as i8;
                    let rook_destination = (1 - 1) * 8 + ('d' as u8 - 'a' as u8) as i8;

                    self.rooks_board &= !(1u64 << rook_source);
                    self.rooks_board |= 1u64 << rook_destination;

                    self.white_board &= !(1u64 << rook_source);
                    self.white_board |= 1u64 << rook_destination;
                }
                PieceColor::Black => {
                    let rook_source = (8 - 1) * 8 + ('a' as u8 - 'a' as u8) as i8;
                    let rook_destination = (8 - 1) * 8 + ('d' as u8 - 'a' as u8) as i8;

                    self.rooks_board &= !(1u64 << rook_source);
                    self.rooks_board |= 1u64 << rook_destination;

                    self.black_board &= !(1u64 << rook_source);
                    self.black_board |= 1u64 << rook_destination;
                }
            },
            MoveType::PawnToKnight => {
                self.pawns_board &= !(1u64 << destination_index); // Delete the pawn
                self.knights_board |= 1u64 << destination_index;
            }
            MoveType::PawnToBishop => {
                self.pawns_board &= !(1u64 << destination_index); // Delete the pawn
                self.bishops_board |= 1u64 << destination_index;
            }
            MoveType::PawnToRook => {
                self.pawns_board &= !(1u64 << destination_index); // Delete the pawn
                self.rooks_board |= 1u64 << destination_index;
            }
            MoveType::PawnToQueen => {
                self.pawns_board &= !(1u64 << destination_index); // Delete the pawn
                self.queens_board |= 1u64 << destination_index;
            }
        }

        match self.turn {
            PieceColor::None => {}
            PieceColor::White => self.turn = PieceColor::Black,
            PieceColor::Black => self.turn = PieceColor::White,
        }
    }

    pub fn get_turn(&self) -> PieceColor {
        self.turn.clone()
    }

    pub fn get_piece_on_square(&self, coord: &Coord) -> Piece {
        let r = coord.rank - 1;
        let f = coord.file as u8 - 'a' as u8;
        let index = r * 8 + f as i8;
        let color = if (self.white_board >> index) & 1 != 0 {
            PieceColor::White
        } else if (self.black_board >> index) & 1 != 0 {
            PieceColor::Black
        } else {
            PieceColor::None
        };

        let piece_type = if (self.pawns_board >> index) & 1 != 0 {
            PieceType::Pawn
        } else if (self.knights_board >> index) & 1 != 0 {
            PieceType::Knight
        } else if (self.bishops_board >> index) & 1 != 0 {
            PieceType::Bishop
        } else if (self.rooks_board >> index) & 1 != 0 {
            PieceType::Rook
        } else if (self.queens_board >> index) & 1 != 0 {
            PieceType::Queen
        } else if (self.kings_board >> index) & 1 != 0 {
            PieceType::King
        } else {
            PieceType::None
        };

        Piece { color, piece_type }
    }

    pub fn get_available_piece_coords(&self) -> Vec<Coord> {
        let mut coords = Vec::new();
        let mut board = match self.turn {
            PieceColor::None => 0,
            PieceColor::White => self.white_board,
            PieceColor::Black => self.black_board,
        };

        while board != 0 {
            let index = board.trailing_zeros() as u8;
            let rank = 1 + (index / 8) as i8;
            let file = (index % 8 + 'a' as u8) as char;
            coords.push(Coord { rank, file });
            board &= board - 1;
        }
        coords
    }

    pub fn get_king_coord(&self, color: &PieceColor) -> Coord {
        let mut king_coord: Coord = Coord { rank: 0, file: 'a' };
        if *color == PieceColor::None {
            panic!("Trying to get a king with the color None")
        }

        for rank in 1..=8 {
            for file in 'a'..='h' {
                let piece = self.get_piece_on_square(&Coord { rank, file });
                if piece.color == *color && piece.piece_type == PieceType::King {
                    king_coord = Coord { rank, file };
                }
            }
        }
        king_coord
    }

    pub fn get_board(&self) -> u64 {
        self.white_board | self.black_board
    }
    pub fn get_white_board(&self) -> u64 {
        self.white_board
    }
    pub fn get_black_board(&self) -> u64 {
        self.black_board
    }

    // TODO: Verify that the move do not put the king in check and that the king is not in check
    pub fn can_short_castle(&self, color: &PieceColor) -> bool {
        match color {
            PieceColor::None => false,
            PieceColor::White => {
                (self.white_king_moves == 0) && (self.white_rook_king_side_moves == 0)
            }
            PieceColor::Black => {
                (self.black_king_moves == 0) && (self.black_rook_king_side_moves == 0)
            }
        }
    }

    pub fn can_long_castle(&self, color: &PieceColor) -> bool {
        match color {
            PieceColor::None => false,
            PieceColor::White => {
                (self.white_king_moves == 0) && (self.white_rook_queen_side_moves == 0)
            }
            PieceColor::Black => {
                (self.black_king_moves == 0) && (self.black_rook_queen_side_moves == 0)
            }
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

    pub fn print_board(&self) {
        for rank in (1..=8).rev() {
            print!("{} ", rank);
            for file in 'a'..='h' {
                print!(
                    "{} ",
                    self.piece_to_unicode(&self.get_piece_on_square(&Coord { rank, file }))
                );
            }
            println!();
        }
        println!("\n  a b c d e f g h\n");
    }
}
