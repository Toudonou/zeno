use rand::prelude::SliceRandom;
use rand::rng;
use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use textplots::{Chart, Plot, Shape};
use thousands::Separable;

use crate::eval_params::EVAL_PARAMS_DEFAULT;
use crate::position::Position;
use crate::search::Searcher;
use crate::transposition_table::TranspositionTable;
use crate::tuner::features::MAX_FEATURES;
use crate::tuner::position_ir::PositionIR;
use crate::tuner::tuner_evaluation::tuner_evaluation;
use crate::tuner::tuner_params::TunerParams;
use crate::utils::{mse, sigmoid, MAX_TUNER_BUFFER_SIZE, ZENO_INFINITY};

pub fn optimize_features(dataset_path: &str, max_number_of_samples: usize, max_iterations: usize, k: f32) -> Result<(), io::Error> {
  let mut number_of_samples = 0;

  // Vector of (Position Intermediary Representation, result)
  let mut total_positions: Vec<(PositionIR, f32)> = Vec::with_capacity(MAX_TUNER_BUFFER_SIZE);
  let file = File::open(dataset_path)?;
  let reader = BufReader::with_capacity(MAX_TUNER_BUFFER_SIZE, file);

  let lines = reader.lines();
  for line in lines {
    number_of_samples += 1;
    if number_of_samples >= max_number_of_samples {
      break;
    }

    let line = match line {
      Ok(line) => line,
      Err(err) => {
        println!("Error when extracting a line: {}", err);
        continue;
      }
    };

    let line = line.trim();
    if let Some((fen, result)) = line.split_once('|') {
      let result = match result.trim() {
        "[1.0]" => 1.0,
        "[0.5]" => 0.5,
        "[0.0]" => 0.0,
        _ => {
          println!("Invalid token while parsing training data: {}", result);
          continue;
        }
      };
      total_positions.push((PositionIR::from_position(&Position::from_fen(fen)), result));
    }
  }

  let mut rng = rng();
  total_positions.shuffle(&mut rng);

  let mut eval_params = TunerParams::from_eval_param(&EVAL_PARAMS_DEFAULT);
  let learning_rate = 0.01;
  let inverse_n = 1.0 / total_positions.len() as f32;

  let mut errors: Vec<(f32, f32)> = Vec::new();
  let duration = Instant::now();

  println!("{} samples for training", total_positions.len().separate_with_commas());
  for iteration in 0..max_iterations {
    let speed = iteration as f32 / duration.elapsed().as_secs() as f32;

    // For parallelism: https://www.rust-patterns.com/book/17-parallel-algorithms.html
    let gradients = total_positions
      .par_iter()
      .fold(|| TunerParams::fill_with(0.0), {
        |mut local_grad: TunerParams, (position, result)| {
          let score = tuner_evaluation(position, &eval_params);
          let sigmoid = sigmoid(k, score);
          let common = (sigmoid - *result) * sigmoid * (1.0 - sigmoid) * k;

          for index in 0..MAX_FEATURES {
            local_grad.mg[index] += common * position.mg_factor * position.board[index] as f32;
            local_grad.eg[index] += common * position.eg_factor * position.board[index] as f32;
          }

          local_grad
        }
      })
      .reduce(
        || TunerParams::fill_with(0.0),
        |mut grad_a, grad_b| {
          for index in 0..MAX_FEATURES {
            grad_a.mg[index] += grad_b.mg[index];
            grad_a.eg[index] += grad_b.eg[index];
          }
          grad_a
        },
      );

    for index in 0..MAX_FEATURES {
      eval_params.mg[index] -= learning_rate * inverse_n * gradients.mg[index];
      eval_params.eg[index] -= learning_rate * inverse_n * gradients.eg[index];
    }

    if iteration % 10 == 0 {
      let loss = total_positions
        .par_iter()
        .map(|(position, result)| {
          let score = tuner_evaluation(position, &eval_params);
          let sigmoid = sigmoid(k, score);
          (sigmoid - result).powi(2)
        })
        .sum::<f32>()
        * inverse_n;
      errors.push((iteration as f32, loss));

      println!("Iteration {}", iteration.separate_with_commas());
      println!("Remaining time = {:?}s", (max_iterations - iteration) as f32 / speed);
      println!("Loss = {}", loss);
    }
  }

  println!("\nError evolution on the validation");
  Chart::new(100, 40, 0.0, max_iterations as f32 + 0.0001).lineplot(&Shape::Lines(&errors)).display();
  println!();

  let header_comment =
    format!("// Dataset: {}\n// Number of samples: {}\n// Number of iterations: {}\n", dataset_path, number_of_samples.separate_with_commas(), max_iterations.separate_with_commas());
  eval_params.save_to_file("params.rs", &header_comment)?;

  Ok(())
}

pub fn optimize_k(file_path: &str, max_number_of_samples: usize) -> Result<f32, io::Error> {
  let mut tt = TranspositionTable::new();
  let mut searcher = Searcher::new(&mut tt);
  let mut number_of_samples = 0;

  // Vector of (qScore, (result))
  let mut games_infos: Vec<(i32, f32)> = Vec::with_capacity(MAX_TUNER_BUFFER_SIZE);
  let file = File::open(file_path)?;
  let reader = BufReader::with_capacity(MAX_TUNER_BUFFER_SIZE, file);

  let lines = reader.lines();
  for line in lines {
    number_of_samples += 1;
    if number_of_samples >= max_number_of_samples {
      break;
    }

    let line = match line {
      Ok(line) => line,
      Err(err) => {
        println!("Error when extracting a line: {}", err);
        continue;
      }
    };

    let line = line.trim();
    if let Some((fen, result)) = line.split_once('|') {
      let result = match result.trim() {
        "[1.0]" => 1.0,
        "[0.5]" => 0.5,
        "[0.0]" => 0.0,
        _ => {
          println!("Invalid token while parsing training data: {}", result);
          continue;
        }
      };
      let mut position = Position::from_fen(fen);
      let score = searcher.quiescence_search(&mut position, -ZENO_INFINITY, ZENO_INFINITY, &EVAL_PARAMS_DEFAULT).value() * position.get_side().to_i32();
      games_infos.push((score, result));
    }
  }

  let mut rng = rng();
  games_infos.shuffle(&mut rng);

  let mut errors: Vec<(f32, f32)> = Vec::new();
  let mut best_k = 0.0;
  let mut best_error = ZENO_INFINITY as f32;
  let mut k = 0.5;
  while k <= 1.2 {
    let error = games_infos
      .par_iter()
      .map(|(score, result)| {
        let sigmoid = sigmoid(k, *score as f32);
        mse(sigmoid, *result)
      })
      .sum();

    errors.push((k, error));
    if error <= best_error {
      best_error = error;
      best_k = k;
    }

    k += 0.001;
  }

  println!("\n{} training data used for optimizing K", number_of_samples.separate_with_commas());

  println!("Error Evolution");
  Chart::new(100, 40, 0.0, k + 0.1).lineplot(&Shape::Lines(&errors)).display();

  println!("Winnig Probability with {best_k}");
  Chart::new(100, 40, -20.0, 20.0).lineplot(&Shape::Continuous(Box::new(|x| sigmoid(best_k, x * 100.0)))).display();

  Ok(best_k)
}
