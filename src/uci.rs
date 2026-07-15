use crate::configs::Config;
use crate::history::History;
use crate::moves::Move;
use crate::perft;
use crate::piece::PieceColor;
use crate::position::Position;
use crate::search_constants::{DEFAULT_NUMBERS_OF_THREADS, MAX_NUMBERS_OF_THREADS, MIN_NUMBERS_OF_THREADS, SearchLimits};
use crate::search_pool::SearchPool;
use crate::transposition_table::{DEFAULT_TRANSPOSITION_SIZE, MAX_TRANSPOSITION_SIZE, MIN_TRANSPOSITION_SIZE, TranspositionTable};
use crate::utils::START_POSITION;
use std::io;
use std::sync::Arc;
use std::time::Instant;
use thousands::Separable;

pub static NAME: &str = "Zeno 2.0-dev";
pub static AUTHOR_NAME: &str = "Toudonou";

pub fn uci_loop() {
  let mut config: Config = Config {
    threads: DEFAULT_NUMBERS_OF_THREADS,
    hash: DEFAULT_TRANSPOSITION_SIZE,
  };

  let mut history = History::new();
  let mut position = Position::from_fen(START_POSITION);
  perft::perft(1, &mut position); // To init the lookup tables

  let mut transposition_table = Arc::new(TranspositionTable::with_capacity(config.hash));
  let mut search_pool = SearchPool::new(transposition_table, config.threads);

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
        search_pool.reset();
        position = Position::from_fen(START_POSITION);

        history.clear();
        history.save_hash(position.get_zobrist_hash());
      }
      c if c.starts_with("position") => uci_position(command, &mut position, &mut history),
      c if c.starts_with("go") => go(command, &mut position, &search_pool, &mut history),
      c if c.starts_with("setoption") => {
        set_option(command, &mut config);
        println!("New engine config : {config:?}");

        transposition_table = Arc::new(TranspositionTable::with_capacity(config.hash));
        search_pool = SearchPool::new(transposition_table, config.threads);
      }
      "stop" => {}
      "quit" => break,
      _ => println!("Command not found {}", command),
    }
  }
}

fn uci_commands() {
  println!("\nid name {}", NAME);
  println!("id author {}\n", AUTHOR_NAME);

  println!(
    "option name Threads type spin default {} min {} max {}",
    DEFAULT_NUMBERS_OF_THREADS, MIN_NUMBERS_OF_THREADS, MAX_NUMBERS_OF_THREADS
  );
  println!(
    "option name Hash type spin default {} min {} max {}",
    DEFAULT_TRANSPOSITION_SIZE, MIN_TRANSPOSITION_SIZE, MAX_TRANSPOSITION_SIZE
  );

  println!();
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
    history.save_hash(position.get_zobrist_hash());
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

fn go(command: &str, position: &mut Position, searcher_pool: &SearchPool, history: &mut History) {
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
  let mov = searcher_pool.search(position, history, SearchLimits::ThinkingTime(search_time));

  match mov {
    Some(best_move) => println!("bestmove {}", best_move),
    None => println!("bestmove 0000"),
  }
}

fn set_option(command: &str, config: &mut Config) {
  if command.starts_with("setoption name Threads value") {
    config.threads = command[("setoption name Threads value".len() + 1)..].parse().unwrap_or(DEFAULT_NUMBERS_OF_THREADS);
  } else if command.starts_with("setoption name Hash value") {
    config.hash = command[("setoption name Hash value".len() + 1)..].parse().unwrap_or(DEFAULT_TRANSPOSITION_SIZE);
  }
}

fn handle_perft(command: &str, position: &mut Position) {
  let depth: i32 = command.strip_prefix("go perft").unwrap_or("").trim().parse().unwrap_or(1);

  let start = Instant::now();
  let nodes = perft::perft(depth, position);
  let duration = start.elapsed();

  let seconds = duration.as_secs_f64();
  let nps = if seconds > 0.0 { (nodes as f64 / seconds) as u64 } else { 0 };

  println!(
    "Perft({}) = {} in {:?}; Speed: {} NPS",
    depth,
    nodes.separate_with_commas(),
    duration,
    nps.separate_with_commas()
  );
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
