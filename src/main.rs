use clap::Parser;
use std::time::Instant;
use thousands::Separable;
use zeno::moves::Move;
use zeno::piece::{PieceColor, PieceType};
use zeno::position::Position;
use zeno::utils::START_POSITION;
use zeno::{cmd, perft, position};

fn main() {
  let args = cmd::Cmd::parse();
  cmd::process_cmd(args);

  // let mut position = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  // for depth in 1..=6 {
  //   let start = Instant::now();
  //   let nodes = perft::perft(depth, &mut position);
  //   let duration = start.elapsed();
  //
  //   let seconds = duration.as_secs_f64();
  //   let nps = if seconds > 0.0 { (nodes as f64 / seconds) as u64 } else { 0 };
  //   println!("Perft({}) = {} in {:?}; Speed: {} NPS", depth, nodes.separate_with_commas(), duration, nps.separate_with_commas());
  // }
}
