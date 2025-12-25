use thousands::Separable;

use crate::moves::Move;
use crate::perft;
use crate::position::Position;
use crate::search::Searcher;
use crate::utils::START_POSITION;
use std::io;
use std::time::Instant;

pub fn uci_loop() {
  let mut searcher = Searcher::new();

  let mut position = Position::from_fen(START_POSITION);

  perft::perft(1, &mut position); // To init the lookup tables

  loop {
    position.print_board();
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    let command = command.trim();

    match command {
      "uci" => uci_commands(),
      "isready" => println!("readyok"),
      "ucinewgame" => {
        position = Position::from_fen(START_POSITION);
      }
      c if c.starts_with("position") => uci_position(command, &mut position),
      c if c.starts_with("go") => go(command, &mut position, &mut searcher),
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
  println!("\t\t * wtime <MILLISECONDS>\t* btime <MILLISECONDS>\t* winc <MILLISECONDS>\t* binc <MILLISECONDS>");
  println!("\t\t * movestogo <INTEGER>\n\t\t * depth <INTEGER>\n\t\t * nodes <INTEGER>\n\t\t * movetime <MILLISECONDS>");
  println!("\t\t * infinite");
  println!("\t\t * perft <INTEGER>");

  println!("\t * stop");
  println!("\t * quit\n");
  println!("uciok\n");
}

fn uci_position(command: &str, position: &mut Position) {
  if command.starts_with("position fen") {
    let is_there_some_moves = command.find("moves");
    match is_there_some_moves {
      None => {
        *position = Position::from_fen(&command[13usize..]);
      }
      Some(moves_index) => {
        *position = Position::from_fen(&command[13usize..moves_index]);

        let moves = command.strip_prefix("position fen moves").unwrap().split_whitespace();
        moves.for_each(|move_string| match Move::from_uci_notation(move_string, position) {
          Some(mov) => position.make_move(mov),
          None => {}
        });
      }
    }
  }

  if command.starts_with("position startpos") {
    *position = Position::from_fen(START_POSITION);
  }

  if command.starts_with("position startpos moves") {
    let moves = command.strip_prefix("position startpos moves").unwrap().split_whitespace();
    moves.for_each(|move_string| match Move::from_uci_notation(move_string, position) {
      Some(mov) => position.make_move(mov),
      None => {}
    });
  }
}

fn go(command: &str, position: &mut Position, searcher: &mut Searcher) {
  if command.starts_with("go perft") {
    handle_perft(command, position);
    return;
  }

  match searcher.search(position) {
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
