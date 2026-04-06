use std::io;
use std::time::Instant;
use thousands::Separable;

use crate::history::History;
use crate::moves::Move;
use crate::perft;
use crate::piece::PieceColor;
use crate::position::Position;
use crate::search::Searcher;
use crate::transposition_table::TranspositionTable;
use crate::utils::START_POSITION;

pub static NAME: &str = "Zeno 2.0-dev";
pub static AUTHOR_NAME: &str = "Toudonou";

pub fn uci_loop() {
  let mut transposition_table = TranspositionTable::new();
  let mut history = History::new();
  let mut searcher = Searcher::new(&mut transposition_table);

  let mut position = Position::from_fen(START_POSITION);
  perft::perft(1, &mut position); // To init the lookup tables

  println!("id name {}", NAME);
  println!("id author {}\n", AUTHOR_NAME);

  loop {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    let command = command.trim();

    match command {
      "uci" | "help" => uci_commands(),
      "isready" => println!("readyok"),
      "ucinewgame" => {
        position = Position::from_fen(START_POSITION);
        history.clear();
        history.save_hash(position.get_zobrist_hash());
        searcher.reset();
      }
      c if c.starts_with("position") => uci_position(command, &mut position, &mut history),
      c if c.starts_with("go") => go(command, &mut position, &mut searcher, &mut history),
      "stop" => {}
      "quit" => break,
      _ => println!("Command not found {}", command),
    }
  }
}

fn uci_commands() {
  println!("\nid name {}", NAME);
  println!("id author {}\n", AUTHOR_NAME);

  println!("Available UCI commands:");
  println!("\t * uci");
  println!("\t * isready");
  println!("\t * ucinewgame");

  println!("\t * position");
  println!("\t\t * fen <FEN>");
  println!("\t\t * startpos");

  println!("\t * go");
  println!("\t\t * wtime <MILLISECONDS>\t* btime <MILLISECONDS>\t* winc <MILLISECONDS>\t* binc <MILLISECONDS>");
  println!("\t\t * movestogo <INTEGER>\n\t\t * depth <INTEGER>\n\t\t * nodes <INTEGER>\n\t\t * movetime <MILLISECONDS>");
  println!("\t\t * infinite");
  println!("\t\t * perft <INTEGER>");

  println!("\t * stop");
  println!("\t * quit\n");
  println!("uciok\n");
}

fn uci_position(command: &str, position: &mut Position, history: &mut History) {
  if command.starts_with("position fen") {
    let is_there_some_moves = command.find("moves");
    match is_there_some_moves {
      None => {
        *position = Position::from_fen(&command[13usize..]);
        history.clear();
        history.save_hash(position.get_zobrist_hash());
      }
      Some(moves_index) => {
        *position = Position::from_fen(&command[13usize..moves_index]);
        history.clear();
        history.save_hash(position.get_zobrist_hash());

        let moves = command[(moves_index + "moves".len())..].split_whitespace();
        moves.for_each(|move_string| match Move::from_uci_notation(move_string, position) {
          Some(mov) => {
            position.make_move(mov);
            history.save_hash(position.get_zobrist_hash());
          }
          None => {}
        });
      }
    }
  }

  if command.starts_with("position startpos") {
    *position = Position::from_fen(START_POSITION);
    history.clear();
  }

  if command.starts_with("position startpos moves") {
    let moves = command.strip_prefix("position startpos moves").unwrap().split_whitespace();
    moves.for_each(|move_string| match Move::from_uci_notation(move_string, position) {
      Some(mov) => {
        position.make_move(mov);
        history.save_hash(position.get_zobrist_hash());
      }
      None => {}
    });
  }
}

fn go(command: &str, position: &mut Position, searcher: &mut Searcher, history: &mut History) {
  if command.starts_with("go perft") {
    handle_perft(command, position);
    return;
  }

  let mut search_time: u128 = 5 * 1000;
  if command.starts_with("go infinite") {
    search_time = 60 * 1000;
  } else if command.starts_with("go movetime") {
    search_time = command[("go movetime".len() + 1)..].parse().unwrap_or(search_time);
  } else if command.starts_with("go wtime") {
    let mut w_time: u32 = 1;
    let mut b_time: u32 = 1;
    let mut w_inc: u32 = 0;
    let mut b_inc: u32 = 0;
    let mut moves_to_go: u32 = 0;

    let mut parts = command["go".len() + 1..].split_whitespace();

    while let Some(token) = parts.next() {
      match token {
        "wtime" => {
          w_time = parts.next().unwrap_or("1").parse().unwrap_or(1);
        }
        "btime" => {
          b_time = parts.next().unwrap_or("1").parse().unwrap_or(1);
        }
        "winc" => {
          w_inc = parts.next().unwrap_or("0").parse().unwrap_or(0);
        }
        "binc" => {
          b_inc = parts.next().unwrap_or("0").parse().unwrap_or(0);
        }
        "movestogo" => {
          moves_to_go = parts.next().unwrap_or("0").parse().unwrap_or(0);
        }
        _ => {}
      }
    }

    match position.get_side() {
      PieceColor::White => search_time = allocate_time(position, w_time, w_inc, moves_to_go),
      PieceColor::Black => search_time = allocate_time(position, b_time, b_inc, moves_to_go),
      PieceColor::None => {}
    }
  }

  search_time = search_time.max(1);
  match searcher.search(position, history, search_time) {
    Some(best_move) => println!("bestmove {}", best_move),
    None => println!("bestmove 0000"),
  }
}

fn handle_perft(command: &str, position: &mut Position) {
  let depth: i32 = command.strip_prefix("go perft").unwrap_or("").trim().parse().unwrap_or(1);

  let start = Instant::now();
  let nodes = perft::perft(depth, position);
  let duration = start.elapsed();

  let seconds = duration.as_secs_f64();
  let nps = if seconds > 0.0 { (nodes as f64 / seconds) as u64 } else { 0 };

  println!("Perft({}) = {} in {:?}; Speed: {} NPS", depth, nodes.separate_with_commas(), duration, nps.separate_with_commas());
}

fn allocate_time(position: &Position, remaining_time: u32, increment: u32, move_to_go: u32) -> u128 {
  let estimated_move_to_go: u32 = move_to_go.max(20);
  let mut allocated_time: u32 = remaining_time / estimated_move_to_go + increment;

  // Still in the opening
  if position.get_number_of_moves() < 7 {
    allocated_time = (50 * allocated_time) / 100;
  }

  allocated_time.min((remaining_time * 20) / 100).max(1) as u128
}
