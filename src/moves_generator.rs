use crate::position::Position;
use crate::utils::{Coord, Move, MoveType, PieceColor, PieceType};

pub fn generate_moves(position: &Position) -> Vec<Move> {
    let available_pieces_for_move = position.get_available_pieces_for_move(&position.get_turn());
    let mut available_moves: Vec<Move> = Vec::new();
    let mut temp_available_moves: Vec<Move> = Vec::new();

    for piece_coord in available_pieces_for_move {
        match piece_coord.0.piece_type {
            PieceType::None => {}
            PieceType::Pawn => {
                temp_available_moves.append(&mut generate_moves_for_pawn(position, &piece_coord.1))
            }
            PieceType::Knight => temp_available_moves
                .append(&mut generate_moves_for_knight(position, &piece_coord.1)),
            PieceType::Bishop => temp_available_moves
                .append(&mut generate_moves_for_bishop(position, &piece_coord.1)),
            PieceType::Rook => {
                temp_available_moves.append(&mut generate_moves_for_rook(position, &piece_coord.1))
            }
            PieceType::Queen => {
                temp_available_moves.append(&mut generate_moves_for_queen(position, &piece_coord.1))
            }
            PieceType::King => {
                temp_available_moves.append(&mut generate_moves_for_king(position, &piece_coord.1))
            }
        }
    }

    // TODO: You can miss the protection of a passed pawn because your king wil avoid moving toward the other king because it will assume that the other king can approach him which an illegal move in reality
    for temp_move in temp_available_moves {
        let mut select_move = true;
        let mut temp_position = position.clone();
        temp_position.make_move(&temp_move);
        let king_coord =
            temp_position.get_king_coord(if temp_position.get_turn() == PieceColor::White {
                &PieceColor::Black
            } else {
                &PieceColor::White
            });
        let opponent_available_pieces_for_move =
            position.get_available_pieces_for_move(&temp_position.get_turn());
        let mut opponent_available_moves: Vec<Move> = Vec::new();
        for piece_coord in opponent_available_pieces_for_move {
            match piece_coord.0.piece_type {
                PieceType::None => {}
                PieceType::Pawn => opponent_available_moves
                    .append(&mut generate_moves_for_pawn(&temp_position, &piece_coord.1)),
                PieceType::Knight => opponent_available_moves.append(
                    &mut generate_moves_for_knight(&temp_position, &piece_coord.1),
                ),
                PieceType::Bishop => opponent_available_moves.append(
                    &mut generate_moves_for_bishop(&temp_position, &piece_coord.1),
                ),
                PieceType::Rook => opponent_available_moves
                    .append(&mut generate_moves_for_rook(&temp_position, &piece_coord.1)),
                PieceType::Queen => opponent_available_moves.append(&mut generate_moves_for_queen(
                    &temp_position,
                    &piece_coord.1,
                )),
                PieceType::King => opponent_available_moves
                    .append(&mut generate_moves_for_king(&temp_position, &piece_coord.1)),
            }
        }
        for opponent_move in opponent_available_moves {
            if opponent_move.destination == king_coord {
                select_move = false;
                break;
            }
        }

        if select_move {
            available_moves.push(temp_move);
        }
    }

    available_moves
}

fn generate_moves_for_pawn(position: &Position, source: &Coord) -> Vec<Move> {
    let piece = position.get_piece_on_square(source);
    let mut moves: Vec<Move> = Vec::new();
    let mut temp_moves: Vec<Move> = Vec::new();

    match piece.color {
        PieceColor::None => {}
        PieceColor::White => {
            if source.rank == 2
                && position
                    .get_piece_on_square(&Coord {
                        rank: 3,
                        file: source.file,
                    })
                    .color
                    == PieceColor::None
            {
                moves.push(Move {
                    source: source.clone(),
                    destination: Coord {
                        rank: 4,
                        file: source.file,
                    },
                    move_type: MoveType::Normal,
                })
            }
            let mut mov = Move {
                source: source.clone(),
                destination: Coord {
                    rank: source.rank + 1,
                    file: (source.file as u8 - 1) as char,
                },
                move_type: MoveType::Normal,
            };
            temp_moves.push(mov.clone());

            if source.rank == 7 {
                mov.move_type = MoveType::PawnToKnight;
                temp_moves.push(mov.clone());
                mov.move_type = MoveType::PawnToBishop;
                temp_moves.push(mov.clone());
                mov.move_type = MoveType::PawnToRook;
                temp_moves.push(mov.clone());
                mov.move_type = MoveType::PawnToQueen;
                temp_moves.push(mov.clone());
            }
        }
        PieceColor::Black => {
            if source.rank == 7
                && position
                    .get_piece_on_square(&Coord {
                        rank: 6,
                        file: source.file,
                    })
                    .color
                    == PieceColor::None
            {
                moves.push(Move {
                    source: source.clone(),
                    destination: Coord {
                        rank: 5,
                        file: source.file,
                    },
                    move_type: MoveType::Normal,
                })
            }
            let mut mov = Move {
                source: source.clone(),
                destination: Coord {
                    rank: source.rank - 1,
                    file: (source.file as u8 - 1) as char,
                },
                move_type: MoveType::Normal,
            };
            temp_moves.push(mov.clone());

            if source.rank == 2 {
                mov.move_type = MoveType::PawnToKnight;
                temp_moves.push(mov.clone());
                mov.move_type = MoveType::PawnToBishop;
                temp_moves.push(mov.clone());
                mov.move_type = MoveType::PawnToRook;
                temp_moves.push(mov.clone());
                mov.move_type = MoveType::PawnToQueen;
                temp_moves.push(mov.clone());
            }
        }
    }

    for temp_move in temp_moves {
        let mut temp_move = temp_move;
        moves.push(temp_move.clone());
        temp_move.destination.file = (temp_move.destination.file as u8 + 1) as char;
        moves.push(temp_move.clone());
        temp_move.destination.file = (temp_move.destination.file as u8 + 1) as char;
        moves.push(temp_move.clone());
    }

    let mut final_move: Vec<Move> = Vec::new();
    for m in moves {
        if !position.is_legal_move(&m) {
            continue;
        }

        if m.source.file == m.destination.file {
            if position.get_piece_on_square(&m.destination).piece_type != PieceType::None {
                continue;
            }
        } else {
            if position.get_piece_on_square(&m.destination).piece_type == PieceType::None
                || position.get_piece_on_square(&m.destination).color
                    == position.get_piece_on_square(&m.source).color
            {
                continue;
            }
        }

        final_move.push(m.clone());
    }

    final_move
}

fn generate_moves_for_knight(position: &Position, source: &Coord) -> Vec<Move> {
    let mut final_moves: Vec<Move> = Vec::new();
    let mut temp_moves: Vec<Move> = Vec::new();

    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank - 1,
            file: (source.file as u8 - 2) as char,
        },
        move_type: MoveType::Normal,
    });
    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank - 1,
            file: (source.file as u8 + 2) as char,
        },
        move_type: MoveType::Normal,
    });

    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank + 1,
            file: (source.file as u8 - 2) as char,
        },
        move_type: MoveType::Normal,
    });
    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank + 1,
            file: (source.file as u8 + 2) as char,
        },
        move_type: MoveType::Normal,
    });

    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank - 2,
            file: (source.file as u8 - 1) as char,
        },
        move_type: MoveType::Normal,
    });
    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank - 2,
            file: (source.file as u8 + 1) as char,
        },
        move_type: MoveType::Normal,
    });

    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank + 2,
            file: (source.file as u8 - 1) as char,
        },
        move_type: MoveType::Normal,
    });
    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank + 2,
            file: (source.file as u8 + 1) as char,
        },
        move_type: MoveType::Normal,
    });

    for m in temp_moves {
        if !position.is_legal_move(&m) {
            continue;
        }
        if position.get_piece_on_square(source).color
            == position.get_piece_on_square(&m.destination).color
        {
            continue;
        }

        final_moves.push(m);
    }

    final_moves
}

fn generate_moves_for_bishop(position: &Position, source: &Coord) -> Vec<Move> {
    let mut final_moves: Vec<Move> = Vec::new();

    let mut i = source.rank + 1;
    let mut j = source.file as u8 + 1;
    let mut process = true;
    while i <= 8 && j <= 'h' as u8 && process {
        let piece = position.get_piece_on_square(&Coord {
            rank: i,
            file: j as char,
        });
        if piece.color == position.get_piece_on_square(source).color {
            break;
        }

        if piece.color != position.get_piece_on_square(source).color
            && piece.color != PieceColor::None
        {
            process = false;
        }

        final_moves.push(Move {
            source: source.clone(),
            destination: Coord {
                rank: i,
                file: j as char,
            },
            move_type: MoveType::Normal,
        });
        i += 1;
        j += 1;
    }

    i = source.rank - 1;
    j = source.file as u8 - 1;
    process = true;
    while 1 <= i && 'a' as u8 <= j && process {
        let piece = position.get_piece_on_square(&Coord {
            rank: i,
            file: j as char,
        });
        if piece.color == position.get_piece_on_square(source).color {
            break;
        }

        if piece.color != position.get_piece_on_square(source).color
            && piece.color != PieceColor::None
        {
            process = false;
        }

        final_moves.push(Move {
            source: source.clone(),
            destination: Coord {
                rank: i,
                file: j as char,
            },
            move_type: MoveType::Normal,
        });
        i -= 1;
        j -= 1;
    }

    i = source.rank + 1;
    j = source.file as u8 - 1;
    process = true;
    while i <= 8 && 'a' as u8 <= j && process {
        let piece = position.get_piece_on_square(&Coord {
            rank: i,
            file: j as char,
        });
        if piece.color == position.get_piece_on_square(source).color {
            break;
        }

        if piece.color != position.get_piece_on_square(source).color
            && piece.color != PieceColor::None
        {
            process = false;
        }

        final_moves.push(Move {
            source: source.clone(),
            destination: Coord {
                rank: i,
                file: j as char,
            },
            move_type: MoveType::Normal,
        });
        i += 1;
        j -= 1;
    }

    i = source.rank - 1;
    j = source.file as u8 + 1;
    process = true;
    while 1 <= i && j <= 'h' as u8 && process {
        let piece = position.get_piece_on_square(&Coord {
            rank: i,
            file: j as char,
        });
        if piece.color == position.get_piece_on_square(source).color {
            break;
        }

        if piece.color != position.get_piece_on_square(source).color
            && piece.color != PieceColor::None
        {
            process = false;
        }

        final_moves.push(Move {
            source: source.clone(),
            destination: Coord {
                rank: i,
                file: j as char,
            },
            move_type: MoveType::Normal,
        });
        i -= 1;
        j += 1;
    }

    final_moves
}

fn generate_moves_for_rook(position: &Position, source: &Coord) -> Vec<Move> {
    let mut final_moves: Vec<Move> = Vec::new();

    let mut i = source.rank;
    let mut j = source.file as u8 - 1;
    let mut process = true;
    while 'a' as u8 <= j && process {
        let piece = position.get_piece_on_square(&Coord {
            rank: i,
            file: j as char,
        });
        if piece.color == position.get_piece_on_square(source).color {
            break;
        }
        if piece.color != position.get_piece_on_square(source).color
            && piece.color != PieceColor::None
        {
            process = false;
        }
        final_moves.push(Move {
            source: source.clone(),
            destination: Coord {
                rank: i,
                file: j as char,
            },
            move_type: MoveType::Normal,
        });
        j -= 1;
    }

    i = source.rank;
    j = source.file as u8 + 1;
    process = true;
    while j <= 'h' as u8 && process {
        let piece = position.get_piece_on_square(&Coord {
            rank: i,
            file: j as char,
        });
        if piece.color == position.get_piece_on_square(source).color {
            break;
        }
        if piece.color != position.get_piece_on_square(source).color
            && piece.color != PieceColor::None
        {
            process = false;
        }
        final_moves.push(Move {
            source: source.clone(),
            destination: Coord {
                rank: i,
                file: j as char,
            },
            move_type: MoveType::Normal,
        });
        j += 1;
    }

    i = source.rank - 1;
    j = source.file as u8;
    process = true;
    while 1 <= i && process {
        let piece = position.get_piece_on_square(&Coord {
            rank: i,
            file: j as char,
        });
        if piece.color == position.get_piece_on_square(source).color {
            break;
        }
        if piece.color != position.get_piece_on_square(source).color
            && piece.color != PieceColor::None
        {
            process = false;
        }
        final_moves.push(Move {
            source: source.clone(),
            destination: Coord {
                rank: i,
                file: j as char,
            },
            move_type: MoveType::Normal,
        });
        i -= 1;
    }

    i = source.rank + 1;
    j = source.file as u8;
    process = true;
    while i <= 8 && process {
        let piece = position.get_piece_on_square(&Coord {
            rank: i,
            file: j as char,
        });
        if piece.color == position.get_piece_on_square(source).color {
            break;
        }
        if piece.color != position.get_piece_on_square(source).color
            && piece.color != PieceColor::None
        {
            process = false;
        }
        final_moves.push(Move {
            source: source.clone(),
            destination: Coord {
                rank: i,
                file: j as char,
            },
            move_type: MoveType::Normal,
        });
        i += 1;
    }

    final_moves
}

fn generate_moves_for_queen(position: &Position, source: &Coord) -> Vec<Move> {
    let mut final_moves = generate_moves_for_bishop(position, source);
    final_moves.append(&mut generate_moves_for_rook(position, source));

    final_moves
}

fn generate_moves_for_king(position: &Position, source: &Coord) -> Vec<Move> {
    let mut final_moves: Vec<Move> = Vec::new();
    let mut temp_moves: Vec<Move> = Vec::new();

    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank - 1,
            file: (source.file as u8 - 1) as char,
        },
        move_type: MoveType::Normal,
    });
    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank - 1,
            file: source.file,
        },
        move_type: MoveType::Normal,
    });
    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank - 1,
            file: (source.file as u8 + 1) as char,
        },
        move_type: MoveType::Normal,
    });

    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank + 1,
            file: (source.file as u8 - 1) as char,
        },
        move_type: MoveType::Normal,
    });
    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank + 1,
            file: source.file,
        },
        move_type: MoveType::Normal,
    });
    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank + 1,
            file: (source.file as u8 + 1) as char,
        },
        move_type: MoveType::Normal,
    });

    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank,
            file: (source.file as u8 - 1) as char,
        },
        move_type: MoveType::Normal,
    });
    temp_moves.push(Move {
        source: source.clone(),
        destination: Coord {
            rank: source.rank,
            file: (source.file as u8 + 1) as char,
        },
        move_type: MoveType::Normal,
    });

    for m in temp_moves {
        if !position.is_legal_move(&m) {
            continue;
        }
        if position.get_piece_on_square(source).color
            == position.get_piece_on_square(&m.destination).color
        {
            continue;
        }

        final_moves.push(m);
    }

    final_moves
}
