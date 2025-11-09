# Zeno Chess Engine

Zeno is a chess engine written in Rust. It uses bitboards, magic move generation, and supports standard chess rules
including castling and en passant.

## Features

- [x] Bitboard-based move generation
  - [ ] Staged move generation
- [x] Castling and en passant support
- [x] Magic bitboard lookup for sliding pieces
- [x] [UCI protocol support (partially)](https://backscattering.de/chess/uci/#engine-info-hashfull)
- [x] Perft testing
- [ ] Evaluation
    - [ ] Material evaluation
    - [ ] Piece-square tables evaluation
    - [ ] Piece Mobility
    - [ ] [Tapered evaluation](https://www.chessprogramming.org/Tapered_Eval)
    - [ ] King Safety
    - [ ] Pawn Structure
    - [ ] Draw Detection (Threefold repetition draws)
    - [ ] Passed Pawns
    - [ ] Bishop Pair
- [ ] Search
    - [ ] [Alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning)
    - [ ] Move Ordering
      - [ ] `PV_MOVE` → `TT_MOVE` → `GOOD_CAPTURES` → `KILLER_MOVES` → `GOOD_QUIET_MOVES` → `BAD_CAPTURES` → `BAD_QUIET_MOVES`
       - [ ] MVV_LVA
      - [ ] Static exchange evaluation
    - [ ] Iterative deepening
    - [ ] Transposition table
    - [ ] Killer moves
    - [ ] Quiescence search
    - [ ] Null move pruning
    - [ ] Late move reductions
    - [ ] Principal Variation Search (PVS)
    - [ ] Aspiration windows
    - [ ] Multi-threading
- [ ] Time management
- [ ] Opening book support
- [ ] Endgame tablebases

## Author

- [Toudonou](https://github.com/Toudonou)

## Useful Links

- https://www.chessprogramming.org
- https://www.dogeystamp.com/chess1/
- https://publish.obsidian.md/modern-uci-doc/UCI+Docs/Intro
- https://dev.to/larswaechter/zobrist-hashing-72n
- https://www.cs.cmu.edu/afs/cs/academic/class/15418-s12/www/competition/www.contrib.andrew.cmu.edu/~jvirdo/rasmussen-2004.pdf
- https://www.josherv.in/2021/03/19/chess-1/
- https://jdhwilkins.com/python-chess-efficient-move-generation-using-bitwise-operations/
- https://raytran.net/projects/protochess
- https://lichess.org/@/likeawizard/blog/review-of-different-board-representations-in-computer-chess/S9eQCAWa
- https://github.com/jhonnold/berserk
- https://markus7800.github.io/blog/AI/chess_engine.html
- https://joeyrobert.org/2016/01/06/optimizing-move-generation/
- https://billylevin.dev/posts/chess-engine-programming/
- https://www.chessprogramming.org/Mop-up_Evaluation
- https://cs.stackexchange.com/questions/1134/how-does-the-negascout-algorithm-work

## License

Zeno is free software, licensed under the GNU General Public License v3. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.