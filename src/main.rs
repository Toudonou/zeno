// https://www.chessprogramming.org
// https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
// https://dev.to/larswaechter/zobrist-hashing-72n
// https://www.cs.cmu.edu/afs/cs/academic/class/15418-s12/www/competition/www.contrib.andrew.cmu.edu/~jvirdo/rasmussen-2004.pdf
// https://www.josherv.in/2021/03/19/chess-1/
// https://jdhwilkins.com/python-chess-efficient-move-generation-using-bitwise-operations/
// https://raytran.net/projects/protochess
// https://lichess.org/@/likeawizard/blog/review-of-different-board-representations-in-computer-chess/S9eQCAWa
// https://github.com/jhonnold/berserk
// https://markus7800.github.io/blog/AI/chess_engine.html
// https://joeyrobert.org/2016/01/06/optimizing-move-generation/

use std::time::Instant;
use zeno::perft;
use zeno::position::Position;

fn main() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let p = Position::from_fen(fen);
    for i in 1..=6 {
        let start = Instant::now();
        print!(
            "Pertf({i}) = {} in ",
            perft::perft(i, &p)
        );
        let duration = start.elapsed();
        println!("{:?}s", duration);
    }
    return;
    zeno::uci::uci_loop()
}
