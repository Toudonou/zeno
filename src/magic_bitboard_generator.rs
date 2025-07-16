use crate::utils::{Coord, Move, MoveType, PieceColor, PieceType};
use std::collections::HashMap;
use std::ops::Add;

fn is_legal_move(mov: &Move) -> bool {
    if !((1 <= mov.source.rank && mov.source.rank <= 8)
        && ('a' <= mov.source.file && mov.source.file <= 'h'))
    {
        return false;
    }
    if !((1 <= mov.destination.rank && mov.destination.rank <= 8)
        && ('a' <= mov.destination.file && mov.destination.file <= 'h'))
    {
        return false;
    }

    true
}

pub fn magic_bit_board_rook() {
    let mut first_time = true;
    for rank in 1..=8 {
        for file in 'a'..='h' {
            let mut final_moves: Vec<Move> = Vec::new();
            let mut i = rank;
            let mut j = file as u8 - 1;

            let mut process = true;

            while 'a' as u8 <= j && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                j -= 1;
            }

            i = rank;
            j = file as u8 + 1;
            process = true;
            while j <= 'h' as u8 && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                j += 1;
            }

            i = rank - 1;
            j = file as u8;
            process = true;
            while 1 <= i && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i -= 1;
            }

            i = rank + 1;
            j = file as u8;
            process = true;
            while i <= 8 && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i += 1;
            }

            print_result("rook_attacks".parse().unwrap(), &rank, &file, &final_moves, &first_time);
            first_time = false;
        }
    }
}

pub fn magic_bit_board_bishop() {
    let mut first_time = true;
    for rank in 1..=8 {
        for file in 'a'..='h' {
            let mut final_moves: Vec<Move> = Vec::new();
            let mut i = rank + 1;
            let mut j = file as u8 + 1;
            let mut process = true;
            while i <= 8 && j <= 'h' as u8 && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i += 1;
                j += 1;
            }

            i = rank - 1;
            j = file as u8 - 1;
            process = true;
            while 1 <= i && 'a' as u8 <= j && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i -= 1;
                j -= 1;
            }

            i = rank + 1;
            j = file as u8 - 1;
            process = true;
            while i <= 8 && 'a' as u8 <= j && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i += 1;
                j -= 1;
            }

            i = rank - 1;
            j = file as u8 + 1;
            process = true;
            while 1 <= i && j <= 'h' as u8 && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i -= 1;
                j += 1;
            }

            print_result(
                "bishop_attacks".parse().unwrap(),
                &rank,
                &file,
                &final_moves,
                &first_time
            );
            first_time = false;
        }
    }
}

pub fn magic_bit_board_knight() {
    let mut first_time = true;
    for rank in 1..=8 {
        for file in 'a'..='h' {
            let mut final_moves: Vec<Move> = Vec::new();
            let mut temp_moves: Vec<Move> = Vec::new();

            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank - 1,
                    file: (file as u8 - 2) as char,
                },
                move_type: MoveType::Normal,
            });
            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank - 1,
                    file: (file as u8 + 2) as char,
                },
                move_type: MoveType::Normal,
            });

            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank + 1,
                    file: (file as u8 - 2) as char,
                },
                move_type: MoveType::Normal,
            });
            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank + 1,
                    file: (file as u8 + 2) as char,
                },
                move_type: MoveType::Normal,
            });

            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank - 2,
                    file: (file as u8 - 1) as char,
                },
                move_type: MoveType::Normal,
            });
            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank - 2,
                    file: (file as u8 + 1) as char,
                },
                move_type: MoveType::Normal,
            });

            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank + 2,
                    file: (file as u8 - 1) as char,
                },
                move_type: MoveType::Normal,
            });
            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank + 2,
                    file: (file as u8 + 1) as char,
                },
                move_type: MoveType::Normal,
            });

            for m in temp_moves {
                if !is_legal_move(&m) {
                    continue;
                }
                final_moves.push(m);
            }

            print_result(
                "knight_attacks".parse().unwrap(),
                &rank,
                &file,
                &final_moves,
                &first_time
            );
            first_time = false;
        }
    }
}

pub fn magic_bit_board_queen() {
    let mut first_time = true;
    for rank in 1..=8 {
        for file in 'a'..='h' {
            let mut final_moves: Vec<Move> = Vec::new();
            let mut i = rank;
            let mut j = file as u8 - 1;

            let mut process = true;

            while 'a' as u8 <= j && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                j -= 1;
            }

            i = rank;
            j = file as u8 + 1;
            process = true;
            while j <= 'h' as u8 && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                j += 1;
            }

            i = rank - 1;
            j = file as u8;
            process = true;
            while 1 <= i && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i -= 1;
            }

            i = rank + 1;
            j = file as u8;
            process = true;
            while i <= 8 && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i += 1;
            }

            i = rank + 1;
            j = file as u8 + 1;
            process = true;
            while i <= 8 && j <= 'h' as u8 && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i += 1;
                j += 1;
            }

            i = rank - 1;
            j = file as u8 - 1;
            process = true;
            while 1 <= i && 'a' as u8 <= j && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i -= 1;
                j -= 1;
            }

            i = rank + 1;
            j = file as u8 - 1;
            process = true;
            while i <= 8 && 'a' as u8 <= j && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i += 1;
                j -= 1;
            }

            i = rank - 1;
            j = file as u8 + 1;
            process = true;
            while 1 <= i && j <= 'h' as u8 && process {
                final_moves.push(Move {
                    source: Coord { rank, file },
                    destination: Coord {
                        rank: i,
                        file: j as char,
                    },
                    move_type: MoveType::Normal,
                });
                i -= 1;
                j += 1;
            }
            print_result("queen_attacks".parse().unwrap(), &rank, &file, &final_moves, &first_time);
            first_time = false;
        }
    }
}

pub fn magic_bit_board_king() {
    let mut first_time = true;
    
    for rank in 1..=8 {
        for file in 'a'..='h' {
            let mut final_moves: Vec<Move> = Vec::new();
            let mut temp_moves: Vec<Move> = Vec::new();

            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank - 1,
                    file: (file as u8 - 1) as char,
                },
                move_type: MoveType::Normal,
            });
            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank - 1,
                    file,
                },
                move_type: MoveType::Normal,
            });
            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank - 1,
                    file: (file as u8 + 1) as char,
                },
                move_type: MoveType::Normal,
            });

            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank + 1,
                    file: (file as u8 - 1) as char,
                },
                move_type: MoveType::Normal,
            });
            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank + 1,
                    file,
                },
                move_type: MoveType::Normal,
            });
            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank: rank + 1,
                    file: (file as u8 + 1) as char,
                },
                move_type: MoveType::Normal,
            });

            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank,
                    file: (file as u8 - 1) as char,
                },
                move_type: MoveType::Normal,
            });
            temp_moves.push(Move {
                source: Coord { rank, file },
                destination: Coord {
                    rank,
                    file: (file as u8 + 1) as char,
                },
                move_type: MoveType::Normal,
            });

            for m in temp_moves {
                if !is_legal_move(&m) {
                    continue;
                }
                final_moves.push(m);
            }

            print_result("king_attacks".parse().unwrap(), &rank, &file, &final_moves, &first_time);
            first_time = false;
        }
    }
}

fn print_result(variable_name: String, rank: &i8, file: &char, final_moves: &Vec<Move>, first_time: &bool) {
    if *first_time{
        println!(
            "let mut {} : HashMap<Coord, u64> = HashMap::new();",
            variable_name
        );
    }
    
    let mut final_string: String = String::from("");
    for r in (1..=8).rev() {
        for f in ('a'..='h').rev() {
            let mut ok = false;
            for mov in final_moves {
                if mov.destination.rank == r && mov.destination.file == f {
                    ok = true;
                    break;
                }
            }

            final_string = final_string + if ok { "1" } else { "0" }
        }
    }
    println!(
        "{}.insert(Coord{{rank:{}, file:'{}'}}, 0b{});\n",
        variable_name, rank, file, final_string
    );

    // let mut i = 0;
    // for x in final_string.chars() {
    //     print!("{x} ");
    //     i = i + 1;
    //     if i % 8 == 0 {
    //         println!();
    //     }
    // }
}
