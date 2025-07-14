use crate::utils::{Coord, Move, MoveType, Piece, PieceColor, PieceType};

#[derive(Clone)]
pub struct Position {
    board: Vec<Piece>,
    turn: PieceColor,
    number_of_move: u32,
    white_can_castle_kingside: bool,
    white_can_castle_queenside: bool,
    black_can_castle_kingside: bool,
    black_can_castle_queenside: bool,
}

impl Position {
    pub fn from_fen(fen: &str) -> Position {
        let mut board = vec![
            Piece {
                color: PieceColor::None,
                piece_type: PieceType::None
            };
            8 * 8
        ];

        let mut board_index: usize = 0;

        let mut parts = fen.split_whitespace();
        let board_part = parts.next().expect("Missing board part");
        let turn_part = parts.next().expect("Missing turn part");
        let castling_part = parts.next().unwrap_or("-");
        let en_passant_part = parts.next().unwrap_or("-");
        let half_move_part = parts.next().expect("Missing half move part");
        let number_of_moves_move_part = parts.next().expect("Missing number of moves part");

        for ch in board_part.chars() {
            match ch {
                '/' => continue,

                '1'..='8' => {
                    let skip = ch.to_digit(10).unwrap();
                    board_index += skip as usize;
                }

                'p' => {
                    board[board_index] = Piece {
                        color: PieceColor::Black,
                        piece_type: PieceType::Pawn,
                    };
                    board_index += 1;
                }
                'r' => {
                    board[board_index] = Piece {
                        color: PieceColor::Black,
                        piece_type: PieceType::Rook,
                    };
                    board_index += 1;
                }
                'n' => {
                    board[board_index] = Piece {
                        color: PieceColor::Black,
                        piece_type: PieceType::Knight,
                    };
                    board_index += 1;
                }
                'b' => {
                    board[board_index] = Piece {
                        color: PieceColor::Black,
                        piece_type: PieceType::Bishop,
                    };
                    board_index += 1;
                }
                'q' => {
                    board[board_index] = Piece {
                        color: PieceColor::Black,
                        piece_type: PieceType::Queen,
                    };
                    board_index += 1;
                }
                'k' => {
                    board[board_index] = Piece {
                        color: PieceColor::Black,
                        piece_type: PieceType::King,
                    };
                    board_index += 1;
                }

                'P' => {
                    board[board_index] = Piece {
                        color: PieceColor::White,
                        piece_type: PieceType::Pawn,
                    };
                    board_index += 1;
                }
                'R' => {
                    board[board_index] = Piece {
                        color: PieceColor::White,
                        piece_type: PieceType::Rook,
                    };
                    board_index += 1;
                }
                'N' => {
                    board[board_index] = Piece {
                        color: PieceColor::White,
                        piece_type: PieceType::Knight,
                    };
                    board_index += 1;
                }
                'B' => {
                    board[board_index] = Piece {
                        color: PieceColor::White,
                        piece_type: PieceType::Bishop,
                    };
                    board_index += 1;
                }
                'Q' => {
                    board[board_index] = Piece {
                        color: PieceColor::White,
                        piece_type: PieceType::Queen,
                    };
                    board_index += 1;
                }
                'K' => {
                    board[board_index] = Piece {
                        color: PieceColor::White,
                        piece_type: PieceType::King,
                    };
                    board_index += 1;
                }

                ' ' => break, // End of board part in FEN
                _ => panic!("Invalid character in FEN: {}", ch),
            }
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
            board,
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
            return self.board[(8 * i as u8 + j) as usize].clone();
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
            self.board[(8 * i as u8 + j) as usize] = piece.clone();
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

    fn piece_to_unicode(&self, piece: Piece) -> char {
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

    pub fn print_board(&self) {
        println!();
        for rank in (1..=8).rev() {
            print!("{} ", rank);
            for file in 'a'..='h' {
                let piece = self.get_piece_on_square(&Coord { rank, file });
                print!("{} ", self.piece_to_unicode(piece));
            }
            println!();
        }
        println!("  a b c d e f g h\n");
    }
}
