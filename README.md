<div align="center">

# Zeno Chess Engine

Zeno is a chess engine written in Rust. It uses bitboards, magic move generation, and supports standard chess rules
including castling and en passant.
<br/>
<br/>
![](https://img.shields.io/badge/Lichess-ratings)
![Bullet](https://img.shields.io/badge/dynamic/json?label=Bullet&color=black&query=$.perfs.bullet.rating&url=https://lichess.org/api/user/zeno-bot)
![Blitz](https://img.shields.io/badge/dynamic/json?label=Blitz&color=darkred&query=$.perfs.blitz.rating&url=https://lichess.org/api/user/zeno-bot)
![Rapid](https://img.shields.io/badge/dynamic/json?label=Rapid&color=darkblue&query=$.perfs.rapid.rating&url=https://lichess.org/api/user/zeno-bot)
![Classical](https://img.shields.io/badge/dynamic/json?label=Classical&color=green&query=$.perfs.classical.rating&url=https://lichess.org/api/user/zeno-bot)
</div>


## Features

- [x] Bitboard-based move generation
  - [x] Legal move generator
- [x] Castling and en passant support
- [x] Magic bitboard lookup for sliding pieces
- [x] [UCI protocol support (partially)](https://backscattering.de/chess/uci/#engine-info-hashfull)
- [x] Perft testing
- [x] [Zobrist hashing](https://en.wikipedia.org/wiki/Zobrist_hashing)
- [x] Evaluation
    - [x] Material evaluation
    - [x] Piece-square tables evaluation
    - [x] [Tapered evaluation](https://www.chessprogramming.org/Tapered_Eval)
    - [x] Draw by insufficient material
    - [x] [Draw by Threefold repetition](https://en.wikipedia.org/wiki/Threefold_repetition)
    - [x] [Fifty-move rule (partially)](https://en.wikipedia.org/wiki/Fifty-move_rule)
- [x] Search
    - [x] [Alpha-beta pruning(Negamax)](https://en.wikipedia.org/wiki/Negamax#Negamax_with_alpha_beta_pruning)
    - [x] Move Ordering
      - [x] TT Move ordering
      - [x] [MVV_LVA](https://www.chessprogramming.org/MVV-LVA)
      - [x] [Killer Heuristic](https://www.chessprogramming.org/Killer_Heuristic)
      - [x] [Countermove Heuristic](https://www.chessprogramming.org/Countermove_Heuristic)
      - [x] [History Heuristic](https://www.chessprogramming.org/History_Heuristic)
      - [x] [Static exchange evaluation](https://www.chessprogramming.org/Static_Exchange_Evaluation)
      - [x] `TT_MOVE` → `GOOD_CAPTURES + PROMOTIONS` → `PROMOTIONS` → `GOOD_CAPTURES` → `KILLERS_MOVES` → `COUNTER_MOVE` → `CASTLES` → `BAD_CAPTURES` → `QUIET_MOVES`
    - [x] Iterative deepening
    - [x] [Transposition table](https://en.wikipedia.org/wiki/Negamax#Negamax_with_alpha_beta_pruning_and_transposition_tables)
    - [x] [Quiescence search](https://www.chessprogramming.org/Quiescence_Search)
    - [x] [Principal Variation Search (PVS)](https://en.wikipedia.org/wiki/Principal_variation_search)
    - [x] Aspiration window
- [x] Time management (partially)


## Installation
1. Clone the repository:
    ```sh
    git clone https://github.com/Toudonou/zeno.git
    ```
2. Navigate to the project directory:
    ```sh
    cd zeno
    ```
3. Build the project using `cargo`:
    ```sh
    cargo build --release
    ```
   ```sh
    ./target/release/zeno
    ```

## Author

- [Toudonou](https://github.com/Toudonou)


## Engine Influences

- [Berserk](https://github.com/jhonnold/berserk)
- [Blunder](https://github.com/deanmchris/blunder)


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
- https://markus7800.github.io/blog/AI/chess_engine.html
- https://billylevin.dev/posts/chess-engine-programming/
- https://cs.stackexchange.com/questions/1134/how-does-the-negascout-algorithm-work
- [Polyglot Format](http://hgm.nubati.net/book_format.html)
- [Polyglot Zobrist Key Generator](https://shinkarom.github.io/zobrist/)
- [Implementing Game Algorithms: Minimax and Alpha-Beta Pruning](https://algocademy.com/blog/implementing-game-algorithms-minimax-and-alpha-beta-pruning/)
- [AlphaDeepChess](https://docta.ucm.es/rest/api/core/bitstreams/4e289e34-0b84-4c1b-9d19-bc0911cfb48b/content)

## License

Zeno is free software, licensed under the GNU General Public License v3. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.
