// https://www.chessprogramming.org
// https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
// https://dev.to/larswaechter/zobrist-hashing-72n
mod position;
mod uci;
mod utils;
mod search;
mod moves_generator;
mod zobrist_hash;
mod evaluation;

fn main() {
    uci::uci_loop();
}
