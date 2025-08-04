// https://www.chessprogramming.org
// https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
// https://dev.to/larswaechter/zobrist-hashing-72n
// https://www.cs.cmu.edu/afs/cs/academic/class/15418-s12/www/competition/www.contrib.andrew.cmu.edu/~jvirdo/rasmussen-2004.pdf
// https://www.josherv.in/2021/03/19/chess-1/
// https://jdhwilkins.com/python-chess-efficient-move-generation-using-bitwise-operations/
// https://raytran.net/projects/protochess
// https://lichess.org/@/likeawizard/blog/review-of-different-board-representations-in-computer-chess/S9eQCAWa
// https://github.com/jhonnold/berserk
// Use x & (x - 1) to clear the least significant bit: this is faster than x &= ~(1 << from)
// https://markus7800.github.io/blog/AI/chess_engine.html


fn main() {
    zeno::uci::uci_loop()
}
