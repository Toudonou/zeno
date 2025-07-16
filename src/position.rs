use crate::utils::{Coord, Move, MoveType, Piece, PieceColor, PieceType};

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
    white_can_castle_kingside: bool,
    white_can_castle_queenside: bool,
    black_can_castle_kingside: bool,
    black_can_castle_queenside: bool,
}

impl Position {
    pub fn from_fen(fen: &str) -> Position {
        /*
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
                    pawns_board |= 1 << board_index;
                    if ch == 'P' {
                        white_board |= 1 << board_index;
                    } else {
                        black_board |= 1 << board_index;
                    }
                }

                'N' | 'n' => {
                    knights_board |= 1 << board_index;
                    if ch == 'N' {
                        white_board |= 1 << board_index;
                    } else {
                        black_board |= 1 << board_index;
                    }
                }

                'B' | 'b' => {
                    bishops_board |= 1 << board_index;
                    if ch == 'B' {
                        white_board |= 1 << board_index;
                    } else {
                        black_board |= 1 << board_index;
                    }
                }

                'R' | 'r' => {
                    rooks_board |= 1 << board_index;
                    if ch == 'R' {
                        white_board |= 1 << board_index;
                    } else {
                        black_board |= 1 << board_index;
                    }
                }

                'Q' | 'q' => {
                    queens_board |= 1 << board_index;
                    if ch == 'Q' {
                        white_board |= 1 << board_index;
                    } else {
                        black_board |= 1 << board_index;
                    }
                }

                'K' | 'k' => {
                    kings_board |= 1 << board_index;
                    if ch == 'K' {
                        white_board |= 1 << board_index;
                    } else {
                        black_board |= 1 << board_index;
                    }
                }

                ' ' => break, // End of board part in FEN
                _ => panic!("Invalid character in FEN: {}", ch),
            }

            board_index = board_index + 1;
        }

        println!("\nmsbsbsdsddsfdfs = {board_index}\n");

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
            white_can_castle_kingside,
            white_can_castle_queenside,
            black_can_castle_kingside,
            black_can_castle_queenside,
        }
    }

    pub fn make_move(&mut self, mov: &Move) {
        if !self.is_legal_move(&mov) {
            return;
        }

        match mov.move_type {
            MoveType::Normal => {
                let piece = self.get_piece_on_square(&mov.source);
                self.set_piece_on_square(&piece, &mov.destination);
            }
            MoveType::ShortCastle => match self.turn {
                PieceColor::None => {}
                PieceColor::White => {
                    self.white_can_castle_kingside = false;
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::None,
                            piece_type: PieceType::None,
                        },
                        &Coord { rank: 1, file: 'e' },
                    );
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::None,
                            piece_type: PieceType::None,
                        },
                        &Coord { rank: 1, file: 'h' },
                    );

                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::White,
                            piece_type: PieceType::King,
                        },
                        &Coord { rank: 1, file: 'g' },
                    );
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::White,
                            piece_type: PieceType::Rook,
                        },
                        &Coord { rank: 1, file: 'f' },
                    );
                }
                PieceColor::Black => {
                    self.black_can_castle_kingside = false;
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::None,
                            piece_type: PieceType::None,
                        },
                        &Coord { rank: 8, file: 'e' },
                    );
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::None,
                            piece_type: PieceType::None,
                        },
                        &Coord { rank: 8, file: 'h' },
                    );

                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::Black,
                            piece_type: PieceType::King,
                        },
                        &Coord { rank: 8, file: 'g' },
                    );
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::Black,
                            piece_type: PieceType::Rook,
                        },
                        &Coord { rank: 8, file: 'f' },
                    );
                }
            },
            MoveType::LongCastle => match self.turn {
                PieceColor::None => {}
                PieceColor::White => {
                    self.white_can_castle_queenside = false;
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::None,
                            piece_type: PieceType::None,
                        },
                        &Coord { rank: 1, file: 'e' },
                    );
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::None,
                            piece_type: PieceType::None,
                        },
                        &Coord { rank: 1, file: 'a' },
                    );

                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::White,
                            piece_type: PieceType::King,
                        },
                        &Coord { rank: 1, file: 'c' },
                    );
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::White,
                            piece_type: PieceType::Rook,
                        },
                        &Coord { rank: 1, file: 'd' },
                    );
                }
                PieceColor::Black => {
                    self.black_can_castle_queenside = false;
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::None,
                            piece_type: PieceType::None,
                        },
                        &Coord { rank: 8, file: 'e' },
                    );
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::None,
                            piece_type: PieceType::None,
                        },
                        &Coord { rank: 8, file: 'a' },
                    );

                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::Black,
                            piece_type: PieceType::King,
                        },
                        &Coord { rank: 8, file: 'c' },
                    );
                    self.set_piece_on_square(
                        &Piece {
                            color: PieceColor::Black,
                            piece_type: PieceType::Rook,
                        },
                        &Coord { rank: 8, file: 'd' },
                    );
                }
            },
            MoveType::PawnToKnight => self.set_piece_on_square(
                &Piece {
                    color: self.turn.clone(),
                    piece_type: PieceType::Knight,
                },
                &mov.destination,
            ),
            MoveType::PawnToBishop => self.set_piece_on_square(
                &Piece {
                    color: self.turn.clone(),
                    piece_type: PieceType::Bishop,
                },
                &mov.destination,
            ),
            MoveType::PawnToRook => self.set_piece_on_square(
                &Piece {
                    color: self.turn.clone(),
                    piece_type: PieceType::Rook,
                },
                &mov.destination,
            ),
            MoveType::PawnToQueen => self.set_piece_on_square(
                &Piece {
                    color: self.turn.clone(),
                    piece_type: PieceType::Queen,
                },
                &mov.destination,
            ),
        }

        self.set_piece_on_square(
            &Piece {
                color: PieceColor::None,
                piece_type: PieceType::None,
            },
            &mov.source,
        );
        match self.turn {
            PieceColor::None => {}
            PieceColor::White => self.turn = PieceColor::Black,
            PieceColor::Black => self.turn = PieceColor::White,
        }
    }

    pub fn get_available_pieces_for_move(&self, color: &PieceColor) -> Vec<(Piece, Coord)> {
        let mut pieces: Vec<(Piece, Coord)> = Vec::new();

        if *color != PieceColor::None {
            for rank in 1..=8 {
                for file in 'a'..='h' {
                    let piece = self.get_piece_on_square(&Coord { rank, file });
                    if piece.color == *color {
                        pieces.push((piece, Coord { rank, file }));
                    }
                }
            }
        }
        pieces
    }

    pub fn get_turn(&self) -> PieceColor {
        self.turn.clone()
    }

    pub fn get_piece_on_square(&self, coord: &Coord) -> Piece {
        if (1 <= coord.rank && coord.rank <= 8) && ('a' <= coord.file && coord.file <= 'h') {
            let i = 8 - coord.rank;
            let j = coord.file as u8 - 'a' as u8;
            return Piece {
                color: PieceColor::Black,
                piece_type: PieceType::Knight,
            }
            .clone();
        }

        Piece {
            color: PieceColor::None,
            piece_type: PieceType::None,
        }
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

    pub fn set_piece_on_square(&mut self, piece: &Piece, coord: &Coord) {
        if (1 <= coord.rank && coord.rank <= 8) && ('a' <= coord.file && coord.file <= 'h') {
            let i = 8 - coord.rank;
            let j = coord.file as u8 - 'a' as u8;
            // self.board[(8 * i as u8 + j) as usize] = piece.clone();
        }
    }

    pub fn is_legal_move(&self, mov: &Move) -> bool {
        if !(1 <= mov.source.rank && mov.source.rank <= 8)
            || !('a' <= mov.source.file && mov.source.file <= 'h')
        {
            return false;
        }

        if !(1 <= mov.destination.rank && mov.destination.rank <= 8)
            || !('a' <= mov.destination.file && mov.destination.file <= 'h')
        {
            return false;
        }

        true
    }

    // TODO: Verify that the move do not put the king in check
    pub fn can_short_castle(&self, color: &PieceColor) -> bool {
        let kin_coord = self.get_king_coord(color);
        let piece_in_h_rank = self.get_piece_on_square(&Coord {
            rank: match color {
                PieceColor::None => 0,
                PieceColor::White => 1,
                PieceColor::Black => 8,
            },
            file: 'h',
        });

        match color {
            PieceColor::None => false,
            PieceColor::White => {
                if kin_coord == (Coord { rank: 1, file: 'e' })
                    && piece_in_h_rank.color == *color
                    && piece_in_h_rank.piece_type == PieceType::Rook
                {
                    return true;
                }
                false
            }
            PieceColor::Black => {
                if kin_coord == (Coord { rank: 8, file: 'e' })
                    && piece_in_h_rank.color == *color
                    && piece_in_h_rank.piece_type == PieceType::Rook
                {
                    return true;
                }
                false
            }
        }
    }

    pub fn can_long_castle(&self, color: &PieceColor) -> bool {
        let kin_coord = self.get_king_coord(color);
        let piece_in_a_rank = self.get_piece_on_square(&Coord {
            rank: match color {
                PieceColor::None => 0,
                PieceColor::White => 1,
                PieceColor::Black => 8,
            },
            file: 'a',
        });

        match color {
            PieceColor::None => false,
            PieceColor::White => {
                if kin_coord == (Coord { rank: 1, file: 'e' })
                    && piece_in_a_rank.color == *color
                    && piece_in_a_rank.piece_type == PieceType::Rook
                {
                    return true;
                }
                false
            }
            PieceColor::Black => {
                if kin_coord == (Coord { rank: 8, file: 'e' })
                    && piece_in_a_rank.color == *color
                    && piece_in_a_rank.piece_type == PieceType::Rook
                {
                    return true;
                }
                false
            }
        }
    }

    fn piece_to_unicode(&self, index: &u64, piece_type: &PieceType) -> char {
        let color = if (self.white_board >> index) & 1 != 0 {
            PieceColor::White
        } else {
            PieceColor::Black
        };

        match (color, piece_type) {
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
        println!("Bitboard:");
        for rank in (0..=7).rev() {
            print!("{}  ", rank + 1);
            for file in 0..=7 {
                let index = rank * 8 + file;
                if (self.pawns_board >> index) & 1 != 0 {
                    print!("{} ", self.piece_to_unicode(&index, &PieceType::Pawn));
                } else if (self.knights_board >> index) & 1 != 0 {
                    print!("{} ", self.piece_to_unicode(&index, &PieceType::Knight));
                } else if (self.bishops_board >> index) & 1 != 0 {
                    print!("{} ", self.piece_to_unicode(&index, &PieceType::Bishop));
                } else if (self.rooks_board >> index) & 1 != 0 {
                    print!("{} ", self.piece_to_unicode(&index, &PieceType::Rook));
                } else if (self.queens_board >> index) & 1 != 0 {
                    print!("{} ", self.piece_to_unicode(&index, &PieceType::Queen));
                } else if (self.kings_board >> index) & 1 != 0 {
                    print!("{} ", self.piece_to_unicode(&index, &PieceType::King));
                } else {
                    print!(". ");
                }
            }
            println!();
        }
        println!("\n   A B C D E F G H\n");
    }
}
