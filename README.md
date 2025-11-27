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
- [x] [Zobrist hashing](https://en.wikipedia.org/wiki/Zobrist_hashing)
- [ ] Evaluation
    - [x] Material evaluation
    - [x] Piece-square tables evaluation
    - [ ] Piece Mobility
    - [x] [Tapered evaluation](https://www.chessprogramming.org/Tapered_Eval)
    - [ ] King Safety
    - [ ] Pawn Structure
    - [x] Draw by insufficient material
    - [x] [Draw by Threefold repetition](https://en.wikipedia.org/wiki/Threefold_repetition)
    - [ ] [Fifty-move rule](https://en.wikipedia.org/wiki/Fifty-move_rule)
    - [ ] Passed Pawns
    - [ ] Bishop Pair
- [ ] Search
    - [x] [Alpha-beta pruning(Negamax)](https://en.wikipedia.org/wiki/Negamax#Negamax_with_alpha_beta_pruning)
    - [ ] Move Ordering
      - [ ] `PV_MOVE` → `TT_MOVE` → `GOOD_CAPTURES` → `KILLER_MOVES` → `GOOD_QUIET_MOVES` → `BAD_CAPTURES` → `BAD_QUIET_MOVES`
      - [x] MVV_LVA
      - [x] [Static exchange evaluation](https://www.chessprogramming.org/Static_Exchange_Evaluation)
      - [x] Normals moves (quiets moves) ordering based on PSQT score
    - [x] Iterative deepening
    - [x] [Transposition table](https://en.wikipedia.org/wiki/Negamax#Negamax_with_alpha_beta_pruning_and_transposition_tables)
    - [x] Killer moves
    - [x] Quiescence search
    - [ ] Null move pruning
    - [ ] Late move reductions
    - [x] [Principal Variation Search (PVS)](https://en.wikipedia.org/wiki/Principal_variation_search)
    - [ ] Aspiration window
    - [ ] Multi-threading
- [x] Time management (partially)
- [x] Opening book support: [Polyglot book format](http://hgm.nubati.net/book_format.html)
- [ ] Endgame tablebases

## Author

- [Toudonou](https://github.com/Toudonou)

## Useful Links

- https://www.chessprogramming.org
- https://www.dogeystamp.com/chess1/
- https://www.cs.vu.nl/~wanf/theses/bijl-tiet-bscthesis.pdf
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
- [Polyglot Zobrist Key Generator](https://shinkarom.github.io/zobrist/)

## License

Zeno is free software, licensed under the GNU General Public License v3. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.