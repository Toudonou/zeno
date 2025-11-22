use crate::position::Position;
use regex::Regex;
use std::io;
use std::time::Instant;
use crate::history::History;
use crate::moves::{Move, MoveType};
use crate::perft;
use crate::piece::{PieceColor, PieceType};
use crate::psqt::TOTAL_PHASE;
use crate::search::Searcher;
use crate::transposition_table::TranspositionTable;
use crate::utils::START_POSITION;

pub fn uci_loop() {
    let mut history = History::new();
    let mut searcher = Searcher::new();
    let mut transposition_table = TranspositionTable::new();

    let mut position = Position::from_fen(START_POSITION, Some(&mut history));

    perft::perft(1, &position); // To init the lookup tables

    loop {
        position.print_board();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "uci" => uci_commands(),
            "isready" => println!("readyok"),
            "ucinewgame" => {
                position = Position::from_fen(START_POSITION, Some(&mut history));
                transposition_table.clear();
                history.clear()
            }
            c if c.starts_with("position") => uci_position(command, &mut position, &mut history),
            c if c.starts_with("go") => go(command, &mut position, &mut searcher, &mut history, &mut transposition_table),
            "stop" => {}
            "quit" => break,
            _ => println!("Command not found {}", command),
        }
    }

    transposition_table.print_transposition_stats();
}

fn uci_commands() {
    println!("\nid name {}", "Zeno");
    println!("id author {}\n", "Toudonou");

    println!("Available UCI commands:");
    println!("\t * uci");
    println!("\t * isready");
    println!("\t * ucinewgame");

    println!("\t * position");
    println!("\t\t * fen <FEN>");
    println!("\t\t * startpos");

    println!("\t * go");
    println!("\t\t * wtime <MILLISECONDS>\n\t* btime <MILLISECONDS>");
    println!("\t\t * winc <MILLISECONDS>\n\t* binc <MILLISECONDS>");
    println!(
        "\t\t * movestogo <INTEGER>\n\t* depth <INTEGER>\n\t* nodes <INTEGER>\n\t* movetime <MILLISECONDS>"
    );
    println!("\t\t * infinite");

    println!("\t * stop");
    println!("\t * quit\n");
    println!("uciok\n");
}


fn uci_position(command: &str, position: &mut Position, mut history: &mut History) {
    if command.starts_with("position fen") {
        let is_there_some_moves = command.find("moves");
        match is_there_some_moves {
            None => {
                *position = Position::from_fen(&command[13usize..], Some(&mut history));
            }
            Some(moves_index) => {
                *position = Position::from_fen(&command[13usize..moves_index], Some(&mut history));

                let moves = command[(moves_index + "moves".len())..].split_whitespace();
                moves.for_each(|move_string| position.make_move(&uci_move(move_string, position), Some(&mut history)));
            }
        }
    }

    if command.starts_with("position startpos") {
        *position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", Some(&mut history));
    }

    if command.starts_with("position startpos moves") {
        let moves = command[23usize..].split_whitespace();
        moves.for_each(|move_string| position.make_move(&uci_move(move_string, position), Some(&mut history)));
    }
}

pub fn uci_move(move_string: &str, position: &Position) -> Move {
    let reg = Regex::new(r"^[a-h][1-8][a-h][1-8][nbrq]?$").unwrap();
    if !reg.is_match(move_string) {
        panic!("Incorrect uci move notation");
    }

    let part: Vec<char> = move_string.chars().collect();
    let source_rank = part[1].to_digit(10).unwrap() - 1;
    let source_file = part[0];
    let destination_rank = part[3].to_digit(10).unwrap() - 1;
    let destination_file = part[2];
    let mut move_type = MoveType::Normal;

    if move_string == "e1g1" || move_string == "e1c1" {
        if position.can_white_short_castle() && move_string == "e1g1" {
            move_type = MoveType::ShortCastle;
        }
        if position.can_white_long_castle() && move_string == "e1c1" {
            move_type = MoveType::LongCastle;
        }
    } else if move_string == "e8g8" || move_string == "e8c8" {
        if position.can_black_short_castle() && move_string == "e8g8" {
            move_type = MoveType::ShortCastle;
        }
        if position.can_black_long_castle() && move_string == "e8c8" {
            move_type = MoveType::LongCastle;
        }
    } else if part.len() == 5 {
        match part[4] {
            'n' => move_type = MoveType::PawnToKnight,
            'b' => move_type = MoveType::PawnToBishop,
            'r' => move_type = MoveType::PawnToRook,
            'q' => move_type = MoveType::PawnToQueen,
            _ => {}
        }
    } else if (8 * destination_rank as u8 + destination_file as u8 - 'a' as u8) == position.get_en_passant() && position.get_piece_on_square(&(8 * source_rank as u8 + source_file as u8 - 'a' as u8)).piece_type == PieceType::Pawn {
        move_type = MoveType::EnPassant;
    }

    let source = (source_rank * 8) as u8 + source_file as u8 - 'a' as u8;
    let destination = (destination_rank * 8) as u8 + destination_file as u8 - 'a' as u8;

    Move::new(source, destination, move_type)
}

fn go(command: &str, position: &mut Position, searcher: &mut Searcher, history: &mut History, transposition_table: &mut TranspositionTable) {
    let mut search_time: u128 = 60 * 1000;

    if command.starts_with("go infinite") {
        search_time = 60 * 1000;
    } else if command.starts_with("go movetime") {
        search_time = command[("go movetime".len() + 1)..].parse().unwrap();
    } else if command.starts_with("go wtime") {
        let mut w_time: u32 = 0;
        let mut b_time: u32 = 0;
        let mut w_inc: u32 = 0;
        let mut b_inc: u32 = 0;
        let mut moves_to_go: u32 = 0;

        let mut parts = command["go".len() + 1..].split_whitespace();

        while let Some(token) = parts.next() {
            match token {
                "wtime" => {
                    w_time = parts.next().expect("missing value for wtime").parse().expect("invalid wtime");
                }
                "btime" => {
                    b_time = parts.next().expect("missing value for btime").parse().expect("invalid btime");
                }
                "winc" => {
                    w_inc = parts.next().expect("missing value for winc").parse().expect("invalid winc");
                }
                "binc" => {
                    b_inc = parts.next().expect("missing value for binc").parse().expect("invalid binc");
                }
                "movestogo" => {
                    moves_to_go = parts.next().expect("missing value for movestogo").parse().expect("invalid movestogo");
                }
                _ => {}
            }
        }

        match position.get_turn() {
            PieceColor::White => search_time = allocate_time(position, w_time, b_time, w_inc, b_inc, moves_to_go),
            PieceColor::Black => search_time = allocate_time(position, b_time, w_time, b_inc, w_inc, moves_to_go),
            PieceColor::None => {}
        }
    }

    search_time = search_time.max(100);

    let it = Instant::now();
    println!("Given search time: {}ms", search_time);
    let best_move = searcher.search(&position, history, transposition_table, search_time);
    println!("Finish in {:?}", it.elapsed());

    match best_move {
        None => println!("No move found"),
        Some(mov) => {
            println!("bestmove {}", mov.to_uci_string())
        }
    }
}

fn allocate_time(position: &Position, remaining_time: u32, opponent_time: u32, increment: u32, opponent_increment: u32, move_to_go: u32) -> u128 {
    let phase = ((position.get_phase() * 256 + TOTAL_PHASE / 2) / TOTAL_PHASE) as u32;

    let estimated_move_to_go: u32 = move_to_go.max(20);
    let base_time: u32 = (remaining_time - 2000) / estimated_move_to_go + increment;

    let middle_game_factor: u32 = 150;
    let end_game_factor: u32 = 100;

    let mut allocated_time: u32 = base_time; // * (middle_game_factor * (256 - phase) + end_game_factor * phase) / 256) / 100;

    // Still in the opening
    if position.get_number_of_move() < 7 {
        allocated_time = (50 * base_time) / 100;
    }

    allocated_time.min((remaining_time * 20) / 100).max(100) as u128
}
