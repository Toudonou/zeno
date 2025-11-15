use crate::position::Position;
use regex::Regex;
use std::io;
use std::time::Instant;
use crate::history::History;
use crate::moves::{Move, MoveType};
use crate::perft;
use crate::piece::PieceType;
use crate::search::Searcher;
use crate::utils::START_POSITION;

pub fn uci_loop() {
    let mut history = History::new();
    let mut position = Position::from_fen(START_POSITION, &mut history);
    let mut searcher = Searcher::new();
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
                position = Position::from_fen(START_POSITION, &mut history)
            }
            c if c.starts_with("position") => uci_position(command, &mut position, &mut history),
            c if c.starts_with("go") => go(&mut position, &mut searcher, &mut history),
            "stop" => {}
            "quit" => break,
            _ => println!("Command not found {}", command),
        }
    }
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


fn uci_position(command: &str, position: &mut Position, history: &mut History) {
    if command.starts_with("position fen") {
        let is_there_some_moves = command.find("moves");
        match is_there_some_moves {
            None => {
                *position = Position::from_fen(&command[13usize..], history);
            }
            Some(moves_index) => {
                *position = Position::from_fen(&command[13usize..moves_index], history);

                let moves = command[(moves_index + "moves".len())..].split_whitespace();
                moves.for_each(|move_string| position.make_move(&uci_move(move_string, position), history));
            }
        }
    }

    if command.starts_with("position startpos") {
        *position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", history);
    }

    if command.starts_with("position startpos moves") {
        let moves = command[23usize..].split_whitespace();
        moves.for_each(|move_string| position.make_move(&uci_move(move_string, position), history));
    }
}

fn uci_move(move_string: &str, position: &Position) -> Move {
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
    } else if (8 * destination_rank as u8 + destination_file as u8 - 'a' as u8) == position.get_en_passant() &&
        position.get_piece_on_square(&(8 * source_rank as u8 + source_file as u8 - 'a' as u8)).piece_type == PieceType::Pawn {
        move_type = MoveType::EnPassant;
    }

    let source = (source_rank * 8) as u8 + source_file as u8 - 'a' as u8;
    let destination = (destination_rank * 8) as u8 + destination_file as u8 - 'a' as u8;

    Move::new(source, destination, move_type)
}

fn go(position: &mut Position, searcher: &mut Searcher, history: &mut History) {
    let it = Instant::now();
    let best_move = searcher.search(&position, history);

    println!("Evaluation: {}", searcher.get_evaluation());
    println!("Number of nodes visited: {} in {:?}", searcher.get_number_of_nodes_visited(), it.elapsed());
    print!("PV Line: ");
    for mov in searcher.get_pv_line() {
        print!("{} ", mov.to_uci_string());
    }
    println!();

    match best_move {
        None => println!("No move found"),
        Some(mov) => {
            println!("bestmove {}", mov.to_uci_string())
        }
    }
}
