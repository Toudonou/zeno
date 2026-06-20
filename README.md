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

![Rust](https://img.shields.io/badge/Language-Rust-red)
![Version](https://img.shields.io/badge/Version-2.0.0-blue)
![License](https://img.shields.io/badge/License-GPLv3-green)

</div>


## Quick Start

### Installation

1. **Clone the repository:**
    ```sh
    git clone https://github.com/Toudonou/zeno.git
    cd zeno
    ```

2. **Build the project:**
    ```sh
    cargo build --release
    ```

3. **Run the engine:**
    ```sh
    ./target/release/zeno
    ```

### Usage

Zeno supports the UCI (Universal Chess Interface) protocol. You can:

- Connect it to UCI-compatible GUIs (Arena, Chess.com, Lichess, etc.)
- Run it interactively from the command line
- Use it for analysis and engine matches

For UCI documentation, see the [UCI Protocol Reference](https://backscattering.de/chess/uci/).

## Documentation

- **[features.md](docs/features.md)** - Complete list of implemented features with references
- **[ratings.md](docs/ratings.md)** - Elo progression and feature-by-feature performance analysis
- **[testing.md](docs/testing.md)** - SPRT test results for major features

## Development

### Building for Development

```sh
cargo build --release # Always in release
```

### Running Tests

```sh
cargo test --release
```

### Testing the Engine

See [docs/testing.md](docs/testing.md) for information on:
- SPRT (Sequential Probability Ratio Test) methodology
- Feature testing procedures
- Performance benchmarking

### Engine Influences

- [Berserk](https://github.com/jhonnold/berserk)
- [Blunder](https://github.com/deanmchris/blunder)
- [Ethereal](https://github.com/AndyGrant/Ethereal)

### Additional resources:

- [Chess Programming Wiki](https://www.chessprogramming.org)
- [UCI Protocol Documentation](https://publish.obsidian.md/modern-uci-doc/UCI+Docs/Intro)
- [Zobrist Hashing](https://dev.to/larswaechter/zobrist-hashing-72n)
- [Board Representation](https://lichess.org/@/likeawizard/blog/review-of-different-board-representations-in-computer-chess/S9eQCAWa)
- [Minimax & Alpha-Beta Pruning](https://algocademy.com/blog/implementing-game-algorithms-minimax-and-alpha-beta-pruning/)
- [Polyglot Format](http://hgm.nubati.net/book_format.html)
- [Polyglot Zobrist Key Generator](https://shinkarom.github.io/zobrist/)
- https://www.dogeystamp.com/chess1/
- https://www.cs.vu.nl/~wanf/theses/bijl-tiet-bscthesis.pdf
- https://www.cs.cmu.edu/afs/cs/academic/class/15418-s12/www/competition/www.contrib.andrew.cmu.edu/~jvirdo/rasmussen-2004.pdf
- https://www.josherv.in/2021/03/19/chess-1/
- https://jdhwilkins.com/python-chess-efficient-move-generation-using-bitwise-operations/
- https://raytran.net/projects/protochess
- https://markus7800.github.io/blog/AI/chess_engine.html
- https://billylevin.dev/posts/chess-engine-programming/
- https://cs.stackexchange.com/questions/1134/how-does-the-negascout-algorithm-work

## Author

- [Toudonou](https://github.com/Toudonou)


## Contributing

Contributions are welcome! Please fork the repository and submit pull requests for any features, bug fixes, or improvements.

## License

Zeno is free software, licensed under the GNU General Public License v3. See [LICENSE](LICENSE) for details.