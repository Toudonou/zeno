use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use std::time::Instant;

use crate::tuner::optimizer::optimize_features;
use crate::uci;
use crate::utils::MAX_TUNER_BUFFER_SIZE;

/// Zeno: a chess engine built by Toudonou 2025-2026
#[derive(Parser, Debug)]
#[command(
  version,
  after_help = "\
  Examples:
    zeno
    zeno uci
    zeno train -i dataset.txt
  "
)]
pub struct Cmd {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Launch the engine in uci mode
  Uci,
  /// Launch the engine in train mode
  Train(TrainArgs),
}

#[derive(Args, Debug)]
pub struct TrainArgs {
  /// Training dataset: a file containing a list of quiets positions and the result of the game (ex: fen | [1.0])
  #[arg(short, long)]
  input_file: PathBuf,

  /// The number of iteration to do for the training
  #[arg(short = 'n', long, default_value_t = 500)]
  iterations: usize,
}

pub fn process_cmd(args: Cmd) {
  match args.command {
    None => uci::uci_loop(),
    Some(commands) => match commands {
      Commands::Uci => uci::uci_loop(),
      Commands::Train(train_args) => {
        let duration = Instant::now();

        let dataset_path = train_args.input_file.into_os_string().into_string().unwrap();
        let iterations = train_args.iterations;

        match optimize_features(&dataset_path, MAX_TUNER_BUFFER_SIZE, iterations, 0.002699149f32) {
          Ok(_) => println!("Training done in {:?}", duration.elapsed()),
          Err(err) => println!("Error: {}", err),
        }
      }
    },
  }
}
