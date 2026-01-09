<div align="center">

# Zeno Chess Engine

Zeno is a chess engine written in Rust. It uses bitboards, magic move generation, and supports standard chess rules
including castling and en passant.
<br/>
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
    - [x] [Fifty-move rule](https://en.wikipedia.org/wiki/Fifty-move_rule)
- [x] Search
    - [x] [Alpha-beta pruning(Negamax)](https://en.wikipedia.org/wiki/Negamax#Negamax_with_alpha_beta_pruning)
    - [x] Move Ordering
      - [x] TT Move ordering
      - [x] [MVV_LVA](https://www.chessprogramming.org/MVV-LVA)
      - [x] Killer Heuristic
    - [x] Iterative deepening
    - [x] [Transposition table](https://en.wikipedia.org/wiki/Negamax#Negamax_with_alpha_beta_pruning_and_transposition_tables)
    - [x] Quiescence search
    - [x] [Principal Variation Search (PVS)](https://en.wikipedia.org/wiki/Principal_variation_search)
    - [x] Aspiration window
	- [x] [Null move pruning](https://web.archive.org/web/20040306081648/http://www.brucemo.com/compchess/programming/nullmove.htm)
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
- [Polyglot Format](http://hgm.nubati.net/book_format.html)
- [Polyglot Zobrist Key Generator](https://shinkarom.github.io/zobrist/)
- [Implementing Game Algorithms: Minimax and Alpha-Beta Pruning](https://algocademy.com/blog/implementing-game-algorithms-minimax-and-alpha-beta-pruning/)
- [AlphaDeepChess](https://docta.ucm.es/rest/api/core/bitstreams/4e289e34-0b84-4c1b-9d19-bc0911cfb48b/content)

## License

Zeno is free software, licensed under the GNU General Public License v3. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.
