use rand::prelude::SliceRandom;
use rand::rng;
use std::f32;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use textplots::{Chart, Plot, Shape};
use thousands::Separable;

use crate::eval_params::EVAL_PARAMS_DEFAULT;
use crate::position::Position;
use crate::tuner::features::MAX_FEATURES;
use crate::tuner::position_ir::PositionIR;
use crate::tuner::tuner_evaluation::tuner_evaluation;
use crate::tuner::tuner_params::TunerParams;
use crate::utils::{MAX_TUNER_BUFFER_SIZE, ZENO_INFINITY, mse, sigmoid};

pub fn optimize_features(dataset_path: &str, max_number_of_samples: usize, max_iterations: usize, k: f32) -> Result<(), io::Error> {
  adam_optimizer(dataset_path, max_number_of_samples, max_iterations, k)
}

/// Features optimization using logistic regression with a cross entropy loss and adam optimizer
/// * y_pred = sigmoid =  1.0 / (1.0 + exp(-z)) see [`crate::utils::sigmoid`] with z = k * x
///   * where `k` is const that has been tweaked once to calibrate the sigmoid with the previous evaluation parameters
///   * and `x` is the position evaluation from the white point of view
///     * x = sum(w_i * x_i) for i in (1; len(eval_parameters)) where w_i is an eval parameter and x_i a feature value
/// * Cross Entropy Loss Function : https://jaketae.github.io/study/logistic-regression/
///   * H(y, y_pred) = -1 * sum(y_i * log(y_pred_i) + (1 - y_i) * log(1 - y_pred_i)) for i in (1; len(positions))
///   * Gradient
///     * ∂H/dw_i = (∂H/∂y_pred) * (∂y_pred/∂z) * (∂z/∂w_i) = (y_pred - y) * k * x_i
///       * ∂z/∂w_i = d(k * x)/∂w_i = d(k * sum(w_i * x_i))/∂w_i = k * x_i
///       * ∂y_pred/∂z = y_pred * (1 - y_pred)
///       * ∂H/∂y_pred = (1 - y) / (1 - y_pred) - y / y_pred
///    * Adam Optimizer : https://en.wikipedia.org/wiki/Stochastic_gradient_descent#Adam
pub fn adam_optimizer(dataset_path: &str, max_number_of_samples: usize, max_iterations: usize, k: f32) -> Result<(), io::Error> {
  let mut rng = rng();
  let mut dataset: Vec<(PositionIR, f32)> = extract_dataset(dataset_path, max_number_of_samples)?;
  let training_size = (90 * dataset.len() / 100).max(1);

  dataset.shuffle(&mut rng);
  let (training_positions, validation_positions) = dataset.split_at(training_size);
  let mut indexes: Vec<usize> = (0..training_size).collect();

  println!();
  println!("Dataset size: {}", dataset.len().separate_with_commas());
  println!("Number of features to optimize: {}", MAX_FEATURES);
  println!("Training dataset size size: {}", training_size.separate_with_commas());
  println!("Validation dataset size size: {}", validation_positions.len().separate_with_commas());
  println!();

  let mut best_tuner_params = TunerParams::from_value(0.0);
  let mut current_tuner_params = TunerParams::from_value(0.0);
  let mut gradients = TunerParams::from_value(0.0);

  let learning_rate = 1e-2;
  let beta1: f32 = 0.9;
  let beta2: f32 = 0.999;
  let epsilon: f32 = 1e-8;

  let mut m = TunerParams::from_value(0.0);
  let mut v = TunerParams::from_value(0.0);
  let mut adam_t: i32 = 0;

  let mut best_training_error = ZENO_INFINITY as f32;
  let mut best_validation_error = ZENO_INFINITY as f32;

  for iteration in 0..max_iterations {
    println!("Iteration {iteration}");

    indexes.shuffle(&mut rng);

    for batch_indexes in indexes.chunks(1024) {
      gradients.fill_with(0.0);

      for batch_index in batch_indexes {
        let (position, y) = &training_positions[*batch_index];
        let score = tuner_evaluation(position, &current_tuner_params);
        let y_pred = sigmoid(k, score);
        let common_grad = (y_pred - y) * k;

        for (index, feature) in &position.board {
          gradients.mg[*index] += common_grad * position.mg_factor * *feature as f32;
          gradients.eg[*index] += common_grad * position.eg_factor * *feature as f32;
        }
      }

      adam_t += 1;
      let bias1: f32 = 1.0 - beta1.powi(adam_t);
      let bias2: f32 = 1.0 - beta2.powi(adam_t);
      let inv = 1.0 / batch_indexes.len() as f32;

      for index in 0..MAX_FEATURES {
        gradients.mg[index] *= inv;
        gradients.eg[index] *= inv;

        m.mg[index] = beta1 * m.mg[index] + (1.0 - beta1) * gradients.mg[index];
        v.mg[index] = beta2 * v.mg[index] + (1.0 - beta2) * gradients.mg[index] * gradients.mg[index];

        m.eg[index] = beta1 * m.eg[index] + (1.0 - beta1) * gradients.eg[index];
        v.eg[index] = beta2 * v.eg[index] + (1.0 - beta2) * gradients.eg[index] * gradients.eg[index];

        let m_at_mg = m.mg[index] / bias1;
        let v_at_mg = v.mg[index] / bias2;

        let m_at_eg = m.eg[index] / bias1;
        let v_at_eg = v.eg[index] / bias2;

        current_tuner_params.mg[index] -= learning_rate * m_at_mg / (v_at_mg.sqrt() + epsilon);
        current_tuner_params.eg[index] -= learning_rate * m_at_eg / (v_at_eg.sqrt() + epsilon);
      }
    }

    if (iteration % 10) == 0 {
      let current_training_error = compute_error(training_positions, &current_tuner_params, k);
      let current_validation_error = compute_error(validation_positions, &current_tuner_params, k);

      if (best_validation_error > current_validation_error) || (best_validation_error == current_validation_error && best_training_error > current_training_error) {
        best_training_error = current_training_error;
        best_validation_error = current_validation_error;
        best_tuner_params = current_tuner_params.clone();
      }

      println!();
      println!("Training error: {}", current_training_error);
      println!("Validation error: {}", current_validation_error);
      println!("Best training error: {best_training_error}");
      println!("Best validation error: {best_validation_error}");
      println!();
    }
  }

  let final_error = compute_error(&dataset, &best_tuner_params, k);
  let header_comments = format!(
    "// Dataset: {}\n// Algorithm: {}\n// Number of features: 2 * {} = {}\n// Number of iterations: {}\n// Dataset size: {}\n// Training set size: {}\n// Validation set size: {}\n// Final error: {}\n// Sigmoid scale (k): {}\n",
    dataset_path,
    "Adam",
    MAX_FEATURES,
    2 * MAX_FEATURES,
    max_iterations.separate_with_commas(),
    dataset.len().separate_with_commas(),
    training_size.separate_with_commas(),
    validation_positions.len().separate_with_commas(),
    final_error,
    k
  );
  best_tuner_params.save_to_file("params.rs", &header_comments)?;
  println!("Error of the best params on the entire dataset: {final_error}");

  Ok(())
}

pub fn optimize_k(dataset_path: &str, max_number_of_samples: usize) -> Result<f32, io::Error> {
  let games_infos = {
    let mut rng = rng();
    let mut dataset: Vec<(PositionIR, f32)> = extract_dataset(dataset_path, max_number_of_samples)?;
    let mut games_infos: Vec<(f32, f32)> = Vec::with_capacity(dataset.len());

    dataset.shuffle(&mut rng);
    let tuner_eval_params = TunerParams::from_eval_param(&EVAL_PARAMS_DEFAULT);
    for position in dataset {
      let score = tuner_evaluation(&position.0, &tuner_eval_params);
      games_infos.push((score, position.1));
    }

    games_infos
  };

  let mut errors: Vec<(f32, f32)> = Vec::new();
  let mut best_k = 0.0;
  let mut best_error = ZENO_INFINITY as f32;
  let mut k = 0.003;
  while k > 0.002 {
    let error = games_infos
      .iter()
      .map(|(score, result)| {
        let sigmoid = sigmoid(k, *score as f32);
        mse(sigmoid, *result)
      })
      .sum();

    errors.push((k, error));
    if error <= best_error {
      best_error = error;
      best_k = k;
      println!("Current best k: {best_k}");
    }

    k -= 0.00000001;
  }

  println!("\n{} training data used for optimizing K", games_infos.len().separate_with_commas());

  println!("Error Evolution");
  Chart::new(100, 40, 0.0020, 0.0027).lineplot(&Shape::Lines(&errors)).display();

  println!("Winnig Probability with {best_k}");
  Chart::new(100, 40, -20.0, 20.0).lineplot(&Shape::Continuous(Box::new(|x| sigmoid(best_k, x * 100.0)))).display();

  Ok(best_k)
}

fn extract_dataset(dataset_path: &str, max_number_of_samples: usize) -> Result<Vec<(PositionIR, f32)>, io::Error> {
  // Vector of (Position Intermediate Representation, result)
  let mut dataset: Vec<(PositionIR, f32)> = Vec::with_capacity(MAX_TUNER_BUFFER_SIZE);
  let file = File::open(dataset_path)?;
  let reader = BufReader::with_capacity(MAX_TUNER_BUFFER_SIZE, file);

  let mut number_of_samples = 0;
  let lines = reader.lines();
  for line in lines {
    number_of_samples += 1;
    if number_of_samples > max_number_of_samples {
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
      dataset.push((PositionIR::from_position(&Position::from_fen(&fen)), result));
    }
  }

  Ok(dataset)
}

fn compute_error(dataset: &[(PositionIR, f32)], tuner_params: &TunerParams, k: f32) -> f32 {
  let mut error = 0.0;

  for (position, y) in dataset {
    let score = tuner_evaluation(&position, &tuner_params);
    let y_pred = sigmoid(k, score);
    error += mse(*y, y_pred);
  }

  error *= 1.0 / dataset.len() as f32;

  error
}
