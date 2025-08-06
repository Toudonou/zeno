use crate::position::Position;
use crate::utils::{Move, MoveType, PieceColor};
use crate::zobrist_hash::ZobristHash;
use crate::search;
use regex::Regex;
use std::io::Write;
use std::thread::sleep;
use std::{io, time};
use crate::evaluation::evaluate;

pub fn uci_loop() {
    let mut position = Position::from_fen("2k5/8/8/4R3/3Q4/4r3/2K5/8 w - - 0 1");

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "uci" => uci_commands(),
            "isready" => println!("readyok"),
            "ucinewgame" => {}
            c if c.starts_with("position") => uci_position(command, &mut position),
            c if c.starts_with("move") => uci_make_move(command, &mut position),
            c if c.starts_with("go") => go(&mut position),
            "stop" => {}
            "quit" => break,
            _ => println!("Command not found {}", command),
        }
        io::stdout().flush().unwrap();
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

pub fn uci_make_move(command: &str, position: &mut Position) {
    // if command.starts_with("") {
    let moves = command[5usize..].split_whitespace();

    moves.for_each(|move_string| position.make_move(&uci_move(move_string, position), false))
    // }
}

fn uci_position(command: &str, position: &mut Position) {
    if command.starts_with("position fen") {
        *position = Position::from_fen(&command[13usize..])
    }

    if command.starts_with("position startpos") {
        *position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    if command.starts_with("position startpos moves") {
        let moves = command[23usize..].split_whitespace();

        moves.for_each(|move_string| position.make_move(&uci_move(move_string, position), false))
    }
}

fn uci_move(move_string: &str, position: &Position) -> Move {
    let reg = Regex::new(r"^[a-h][1-8][a-h][1-8][nbrq]?$").unwrap();
    if !reg.is_match(move_string) {
        panic!("Incorrect uci move notation");
    }

    let part: Vec<char> = move_string.chars().collect();
    let source_rank = part[1].to_digit(10).unwrap();
    let source_file = part[0];
    let destination_rank = part[3].to_digit(10).unwrap();
    let destination_file = part[2];
    let mut move_type = MoveType::Normal;

    if move_string == "e1g1" || move_string == "e1c1" {
        if position.can_short_castle(&PieceColor::White) && move_string == "e1g1" {
            move_type = MoveType::ShortCastle;
        }
        if position.can_long_castle(&PieceColor::White) && move_string == "e1c1" {
            move_type = MoveType::LongCastle;
        }
    } else if move_string == "e8g8" || move_string == "e8c8" {
        if position.can_short_castle(&PieceColor::Black) && move_string == "e8g8" {
            move_type = MoveType::ShortCastle;
        }
        if position.can_long_castle(&PieceColor::Black) && move_string == "e8c8" {
            move_type = MoveType::LongCastle;
        }
    }

    if part.len() == 5 {
        match part[4] {
            'n' => move_type = MoveType::PawnToKnight,
            'b' => move_type = MoveType::PawnToBishop,
            'r' => move_type = MoveType::PawnToRook,
            'q' => move_type = MoveType::PawnToQueen,
            _ => {}
        }
    }

    Move {
        source: (((source_rank - 1) * 8) as u8 + source_file as u8 - 'a' as u8) as i8,
        destination: (((destination_rank - 1) * 8) as u8 + destination_file as u8 - 'a' as u8) as i8,
        move_type,
    }
}

fn go(position: &mut Position) {
    position.print_board();
    let mov = search::best_move(position);
    let move_type_character: char;

    match mov.move_type {
        MoveType::ShortCastle => move_type_character = ' ',
        MoveType::LongCastle => move_type_character = ' ',
        MoveType::PawnToKnight => move_type_character = 'n',
        MoveType::PawnToBishop => move_type_character = 'b',
        MoveType::PawnToRook => move_type_character = 'r',
        MoveType::PawnToQueen => move_type_character = 'q',
        _ => move_type_character = ' ',
    }

    println!(
        "bestmove {}{}{}{}{}",
        ('a' as u8 + (mov.source % 8) as u8) as char,
        1 + mov.source / 8,
        ('a' as u8 + (mov.destination % 8) as u8) as char,
        1 + mov.destination / 8,
        move_type_character
    );
}
