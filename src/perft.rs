use crate::moves::MoveList;
use crate::moves_generator::generate_legal_moves;
use crate::position::Position;
use std::time::Instant;
use thousands::Separable;

pub fn perft(depth: i32, position: &mut Position) -> u64 {
  if depth == 0 {
    return 1;
  }

  let mut move_list = MoveList::new();
  generate_legal_moves(position, &mut move_list);

  if depth == 1 {
    return move_list.count as u64;
  }

  let mut nodes: u64 = 0;
  for idx in 0..move_list.count {
    let mut temp_position = position.clone();
    temp_position.make_move(move_list.moves[idx]);
    nodes += perft(depth - 1, &mut temp_position);
  }

  nodes
}

pub fn perft_divide(depth: i32, position: &mut Position) {
  let mut total_nodes = 0;
  let start = Instant::now();

  let mut move_list = MoveList::new();
  generate_legal_moves(position, &mut move_list);
  for idx in 0..move_list.count {
    let mut temp_position = position.clone();
    temp_position.make_move(move_list.moves[idx]);
    let nodes = perft(depth - 1, &mut temp_position);

    println!("{}: {}", move_list.moves[idx], nodes);
    total_nodes += nodes;
  }

  let duration = start.elapsed();
  let speed = total_nodes as f32 / duration.as_secs_f32();

  println!("Total number of moves: {}", total_nodes.separate_with_commas());
  println!("Search finished in {:?}", duration);
  println!("Speed: {} NPS", speed.separate_with_commas());
  println!()
}
