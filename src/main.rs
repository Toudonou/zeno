// https://www.chessprogramming.org
// https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
// https://dev.to/larswaechter/zobrist-hashing-72n
// https://www.cs.cmu.edu/afs/cs/academic/class/15418-s12/www/competition/www.contrib.andrew.cmu.edu/~jvirdo/rasmussen-2004.pdf
// https://www.josherv.in/2021/03/19/chess-1/

use crate::utils::Coord;
use std::collections::HashMap;
use crate::position::Position;

mod evaluation;
mod magic_bitboard_generator;
mod moves_generator;
mod position;
mod search;
mod uci;
mod utils;
mod zobrist_hash;

fn main() {
    let mut i: i32 = 0b0000000000000000000000000000000000000000111100110000000000000000;

    while i != 0 {
        let des = i.trailing_zeros();
        println!("{}", des);
        i = i & !(1 << des);
        println!("{:b}", i);
    }

    let mut position = Position::from_fen("5k2/1rn2p2/3pb1p1/7p/p3PP2/PnNBK2P/3N2P1/1R6 w - - 0 1");
    position.print_board();

    let mut knight_attacks: HashMap<Coord, u64> = HashMap::new();
    knight_attacks.insert(
        Coord { rank: 1, file: 'a' },
        0b0000000000000000000000000000000000000000000000100000010000000000,
    );

    knight_attacks.insert(
        Coord { rank: 1, file: 'b' },
        0b0000000000000000000000000000000000000000000001010000100000000000,
    );

    knight_attacks.insert(
        Coord { rank: 1, file: 'c' },
        0b0000000000000000000000000000000000000000000010100001000100000000,
    );

    knight_attacks.insert(
        Coord { rank: 1, file: 'd' },
        0b0000000000000000000000000000000000000000000101000010001000000000,
    );

    knight_attacks.insert(
        Coord { rank: 1, file: 'e' },
        0b0000000000000000000000000000000000000000001010000100010000000000,
    );

    knight_attacks.insert(
        Coord { rank: 1, file: 'f' },
        0b0000000000000000000000000000000000000000010100001000100000000000,
    );

    knight_attacks.insert(
        Coord { rank: 1, file: 'g' },
        0b0000000000000000000000000000000000000000101000000001000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 1, file: 'h' },
        0b0000000000000000000000000000000000000000010000000010000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 2, file: 'a' },
        0b0000000000000000000000000000000000000010000001000000000000000100,
    );

    knight_attacks.insert(
        Coord { rank: 2, file: 'b' },
        0b0000000000000000000000000000000000000101000010000000000000001000,
    );

    knight_attacks.insert(
        Coord { rank: 2, file: 'c' },
        0b0000000000000000000000000000000000001010000100010000000000010001,
    );

    knight_attacks.insert(
        Coord { rank: 2, file: 'd' },
        0b0000000000000000000000000000000000010100001000100000000000100010,
    );

    knight_attacks.insert(
        Coord { rank: 2, file: 'e' },
        0b0000000000000000000000000000000000101000010001000000000001000100,
    );

    knight_attacks.insert(
        Coord { rank: 2, file: 'f' },
        0b0000000000000000000000000000000001010000100010000000000010001000,
    );

    knight_attacks.insert(
        Coord { rank: 2, file: 'g' },
        0b0000000000000000000000000000000010100000000100000000000000010000,
    );

    knight_attacks.insert(
        Coord { rank: 2, file: 'h' },
        0b0000000000000000000000000000000001000000001000000000000000100000,
    );

    knight_attacks.insert(
        Coord { rank: 3, file: 'a' },
        0b0000000000000000000000000000001000000100000000000000010000000010,
    );

    knight_attacks.insert(
        Coord { rank: 3, file: 'b' },
        0b0000000000000000000000000000010100001000000000000000100000000101,
    );

    knight_attacks.insert(
        Coord { rank: 3, file: 'c' },
        0b0000000000000000000000000000101000010001000000000001000100001010,
    );

    knight_attacks.insert(
        Coord { rank: 3, file: 'd' },
        0b0000000000000000000000000001010000100010000000000010001000010100,
    );

    knight_attacks.insert(
        Coord { rank: 3, file: 'e' },
        0b0000000000000000000000000010100001000100000000000100010000101000,
    );

    knight_attacks.insert(
        Coord { rank: 3, file: 'f' },
        0b0000000000000000000000000101000010001000000000001000100001010000,
    );

    knight_attacks.insert(
        Coord { rank: 3, file: 'g' },
        0b0000000000000000000000001010000000010000000000000001000010100000,
    );

    knight_attacks.insert(
        Coord { rank: 3, file: 'h' },
        0b0000000000000000000000000100000000100000000000000010000001000000,
    );

    knight_attacks.insert(
        Coord { rank: 4, file: 'a' },
        0b0000000000000000000000100000010000000000000001000000001000000000,
    );

    knight_attacks.insert(
        Coord { rank: 4, file: 'b' },
        0b0000000000000000000001010000100000000000000010000000010100000000,
    );

    knight_attacks.insert(
        Coord { rank: 4, file: 'c' },
        0b0000000000000000000010100001000100000000000100010000101000000000,
    );

    knight_attacks.insert(
        Coord { rank: 4, file: 'd' },
        0b0000000000000000000101000010001000000000001000100001010000000000,
    );

    knight_attacks.insert(
        Coord { rank: 4, file: 'e' },
        0b0000000000000000001010000100010000000000010001000010100000000000,
    );

    knight_attacks.insert(
        Coord { rank: 4, file: 'f' },
        0b0000000000000000010100001000100000000000100010000101000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 4, file: 'g' },
        0b0000000000000000101000000001000000000000000100001010000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 4, file: 'h' },
        0b0000000000000000010000000010000000000000001000000100000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 5, file: 'a' },
        0b0000000000000010000001000000000000000100000000100000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 5, file: 'b' },
        0b0000000000000101000010000000000000001000000001010000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 5, file: 'c' },
        0b0000000000001010000100010000000000010001000010100000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 5, file: 'd' },
        0b0000000000010100001000100000000000100010000101000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 5, file: 'e' },
        0b0000000000101000010001000000000001000100001010000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 5, file: 'f' },
        0b0000000001010000100010000000000010001000010100000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 5, file: 'g' },
        0b0000000010100000000100000000000000010000101000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 5, file: 'h' },
        0b0000000001000000001000000000000000100000010000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 6, file: 'a' },
        0b0000001000000100000000000000010000000010000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 6, file: 'b' },
        0b0000010100001000000000000000100000000101000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 6, file: 'c' },
        0b0000101000010001000000000001000100001010000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 6, file: 'd' },
        0b0001010000100010000000000010001000010100000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 6, file: 'e' },
        0b0010100001000100000000000100010000101000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 6, file: 'f' },
        0b0101000010001000000000001000100001010000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 6, file: 'g' },
        0b1010000000010000000000000001000010100000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 6, file: 'h' },
        0b0100000000100000000000000010000001000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 7, file: 'a' },
        0b0000010000000000000001000000001000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 7, file: 'b' },
        0b0000100000000000000010000000010100000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 7, file: 'c' },
        0b0001000100000000000100010000101000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 7, file: 'd' },
        0b0010001000000000001000100001010000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 7, file: 'e' },
        0b0100010000000000010001000010100000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 7, file: 'f' },
        0b1000100000000000100010000101000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 7, file: 'g' },
        0b0001000000000000000100001010000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 7, file: 'h' },
        0b0010000000000000001000000100000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 8, file: 'a' },
        0b0000000000000100000000100000000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 8, file: 'b' },
        0b0000000000001000000001010000000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 8, file: 'c' },
        0b0000000000010001000010100000000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 8, file: 'd' },
        0b0000000000100010000101000000000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 8, file: 'e' },
        0b0000000001000100001010000000000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 8, file: 'f' },
        0b0000000010001000010100000000000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 8, file: 'g' },
        0b0000000000010000101000000000000000000000000000000000000000000000,
    );

    knight_attacks.insert(
        Coord { rank: 8, file: 'h' },
        0b0000000000100000010000000000000000000000000000000000000000000000,
    );

    let mut bishop_attacks: HashMap<Coord, u64> = HashMap::new();
    bishop_attacks.insert(
        Coord { rank: 1, file: 'a' },
        0b1000000001000000001000000001000000001000000001000000001000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 1, file: 'b' },
        0b0000000010000000010000000010000000010000000010000000010100000000,
    );

    bishop_attacks.insert(
        Coord { rank: 1, file: 'c' },
        0b0000000000000000100000000100000000100000000100010000101000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 1, file: 'd' },
        0b0000000000000000000000001000000001000001001000100001010000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 1, file: 'e' },
        0b0000000000000000000000000000000110000010010001000010100000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 1, file: 'f' },
        0b0000000000000000000000010000001000000100100010000101000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 1, file: 'g' },
        0b0000000000000001000000100000010000001000000100001010000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 1, file: 'h' },
        0b0000000100000010000001000000100000010000001000000100000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 2, file: 'a' },
        0b0100000000100000000100000000100000000100000000100000000000000010,
    );

    bishop_attacks.insert(
        Coord { rank: 2, file: 'b' },
        0b1000000001000000001000000001000000001000000001010000000000000101,
    );

    bishop_attacks.insert(
        Coord { rank: 2, file: 'c' },
        0b0000000010000000010000000010000000010001000010100000000000001010,
    );

    bishop_attacks.insert(
        Coord { rank: 2, file: 'd' },
        0b0000000000000000100000000100000100100010000101000000000000010100,
    );

    bishop_attacks.insert(
        Coord { rank: 2, file: 'e' },
        0b0000000000000000000000011000001001000100001010000000000000101000,
    );

    bishop_attacks.insert(
        Coord { rank: 2, file: 'f' },
        0b0000000000000001000000100000010010001000010100000000000001010000,
    );

    bishop_attacks.insert(
        Coord { rank: 2, file: 'g' },
        0b0000000100000010000001000000100000010000101000000000000010100000,
    );

    bishop_attacks.insert(
        Coord { rank: 2, file: 'h' },
        0b0000001000000100000010000001000000100000010000000000000001000000,
    );

    bishop_attacks.insert(
        Coord { rank: 3, file: 'a' },
        0b0010000000010000000010000000010000000010000000000000001000000100,
    );

    bishop_attacks.insert(
        Coord { rank: 3, file: 'b' },
        0b0100000000100000000100000000100000000101000000000000010100001000,
    );

    bishop_attacks.insert(
        Coord { rank: 3, file: 'c' },
        0b1000000001000000001000000001000100001010000000000000101000010001,
    );

    bishop_attacks.insert(
        Coord { rank: 3, file: 'd' },
        0b0000000010000000010000010010001000010100000000000001010000100010,
    );

    bishop_attacks.insert(
        Coord { rank: 3, file: 'e' },
        0b0000000000000001100000100100010000101000000000000010100001000100,
    );

    bishop_attacks.insert(
        Coord { rank: 3, file: 'f' },
        0b0000000100000010000001001000100001010000000000000101000010001000,
    );

    bishop_attacks.insert(
        Coord { rank: 3, file: 'g' },
        0b0000001000000100000010000001000010100000000000001010000000010000,
    );

    bishop_attacks.insert(
        Coord { rank: 3, file: 'h' },
        0b0000010000001000000100000010000001000000000000000100000000100000,
    );

    bishop_attacks.insert(
        Coord { rank: 4, file: 'a' },
        0b0001000000001000000001000000001000000000000000100000010000001000,
    );

    bishop_attacks.insert(
        Coord { rank: 4, file: 'b' },
        0b0010000000010000000010000000010100000000000001010000100000010000,
    );

    bishop_attacks.insert(
        Coord { rank: 4, file: 'c' },
        0b0100000000100000000100010000101000000000000010100001000100100000,
    );

    bishop_attacks.insert(
        Coord { rank: 4, file: 'd' },
        0b1000000001000001001000100001010000000000000101000010001001000001,
    );

    bishop_attacks.insert(
        Coord { rank: 4, file: 'e' },
        0b0000000110000010010001000010100000000000001010000100010010000010,
    );

    bishop_attacks.insert(
        Coord { rank: 4, file: 'f' },
        0b0000001000000100100010000101000000000000010100001000100000000100,
    );

    bishop_attacks.insert(
        Coord { rank: 4, file: 'g' },
        0b0000010000001000000100001010000000000000101000000001000000001000,
    );

    bishop_attacks.insert(
        Coord { rank: 4, file: 'h' },
        0b0000100000010000001000000100000000000000010000000010000000010000,
    );

    bishop_attacks.insert(
        Coord { rank: 5, file: 'a' },
        0b0000100000000100000000100000000000000010000001000000100000010000,
    );

    bishop_attacks.insert(
        Coord { rank: 5, file: 'b' },
        0b0001000000001000000001010000000000000101000010000001000000100000,
    );

    bishop_attacks.insert(
        Coord { rank: 5, file: 'c' },
        0b0010000000010001000010100000000000001010000100010010000001000000,
    );

    bishop_attacks.insert(
        Coord { rank: 5, file: 'd' },
        0b0100000100100010000101000000000000010100001000100100000110000000,
    );

    bishop_attacks.insert(
        Coord { rank: 5, file: 'e' },
        0b1000001001000100001010000000000000101000010001001000001000000001,
    );

    bishop_attacks.insert(
        Coord { rank: 5, file: 'f' },
        0b0000010010001000010100000000000001010000100010000000010000000010,
    );

    bishop_attacks.insert(
        Coord { rank: 5, file: 'g' },
        0b0000100000010000101000000000000010100000000100000000100000000100,
    );

    bishop_attacks.insert(
        Coord { rank: 5, file: 'h' },
        0b0001000000100000010000000000000001000000001000000001000000001000,
    );

    bishop_attacks.insert(
        Coord { rank: 6, file: 'a' },
        0b0000010000000010000000000000001000000100000010000001000000100000,
    );

    bishop_attacks.insert(
        Coord { rank: 6, file: 'b' },
        0b0000100000000101000000000000010100001000000100000010000001000000,
    );

    bishop_attacks.insert(
        Coord { rank: 6, file: 'c' },
        0b0001000100001010000000000000101000010001001000000100000010000000,
    );

    bishop_attacks.insert(
        Coord { rank: 6, file: 'd' },
        0b0010001000010100000000000001010000100010010000011000000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 6, file: 'e' },
        0b0100010000101000000000000010100001000100100000100000000100000000,
    );

    bishop_attacks.insert(
        Coord { rank: 6, file: 'f' },
        0b1000100001010000000000000101000010001000000001000000001000000001,
    );

    bishop_attacks.insert(
        Coord { rank: 6, file: 'g' },
        0b0001000010100000000000001010000000010000000010000000010000000010,
    );

    bishop_attacks.insert(
        Coord { rank: 6, file: 'h' },
        0b0010000001000000000000000100000000100000000100000000100000000100,
    );

    bishop_attacks.insert(
        Coord { rank: 7, file: 'a' },
        0b0000001000000000000000100000010000001000000100000010000001000000,
    );

    bishop_attacks.insert(
        Coord { rank: 7, file: 'b' },
        0b0000010100000000000001010000100000010000001000000100000010000000,
    );

    bishop_attacks.insert(
        Coord { rank: 7, file: 'c' },
        0b0000101000000000000010100001000100100000010000001000000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 7, file: 'd' },
        0b0001010000000000000101000010001001000001100000000000000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 7, file: 'e' },
        0b0010100000000000001010000100010010000010000000010000000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 7, file: 'f' },
        0b0101000000000000010100001000100000000100000000100000000100000000,
    );

    bishop_attacks.insert(
        Coord { rank: 7, file: 'g' },
        0b1010000000000000101000000001000000001000000001000000001000000001,
    );

    bishop_attacks.insert(
        Coord { rank: 7, file: 'h' },
        0b0100000000000000010000000010000000010000000010000000010000000010,
    );

    bishop_attacks.insert(
        Coord { rank: 8, file: 'a' },
        0b0000000000000010000001000000100000010000001000000100000010000000,
    );

    bishop_attacks.insert(
        Coord { rank: 8, file: 'b' },
        0b0000000000000101000010000001000000100000010000001000000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 8, file: 'c' },
        0b0000000000001010000100010010000001000000100000000000000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 8, file: 'd' },
        0b0000000000010100001000100100000110000000000000000000000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 8, file: 'e' },
        0b0000000000101000010001001000001000000001000000000000000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 8, file: 'f' },
        0b0000000001010000100010000000010000000010000000010000000000000000,
    );

    bishop_attacks.insert(
        Coord { rank: 8, file: 'g' },
        0b0000000010100000000100000000100000000100000000100000000100000000,
    );

    bishop_attacks.insert(
        Coord { rank: 8, file: 'h' },
        0b0000000001000000001000000001000000001000000001000000001000000001,
    );

    let mut rook_attacks: HashMap<Coord, u64> = HashMap::new();
    rook_attacks.insert(
        Coord { rank: 1, file: 'a' },
        0b0000000100000001000000010000000100000001000000010000000111111110,
    );

    rook_attacks.insert(
        Coord { rank: 1, file: 'b' },
        0b0000001000000010000000100000001000000010000000100000001011111101,
    );

    rook_attacks.insert(
        Coord { rank: 1, file: 'c' },
        0b0000010000000100000001000000010000000100000001000000010011111011,
    );

    rook_attacks.insert(
        Coord { rank: 1, file: 'd' },
        0b0000100000001000000010000000100000001000000010000000100011110111,
    );

    rook_attacks.insert(
        Coord { rank: 1, file: 'e' },
        0b0001000000010000000100000001000000010000000100000001000011101111,
    );

    rook_attacks.insert(
        Coord { rank: 1, file: 'f' },
        0b0010000000100000001000000010000000100000001000000010000011011111,
    );

    rook_attacks.insert(
        Coord { rank: 1, file: 'g' },
        0b0100000001000000010000000100000001000000010000000100000010111111,
    );

    rook_attacks.insert(
        Coord { rank: 1, file: 'h' },
        0b1000000010000000100000001000000010000000100000001000000001111111,
    );

    rook_attacks.insert(
        Coord { rank: 2, file: 'a' },
        0b0000000100000001000000010000000100000001000000011111111000000001,
    );

    rook_attacks.insert(
        Coord { rank: 2, file: 'b' },
        0b0000001000000010000000100000001000000010000000101111110100000010,
    );

    rook_attacks.insert(
        Coord { rank: 2, file: 'c' },
        0b0000010000000100000001000000010000000100000001001111101100000100,
    );

    rook_attacks.insert(
        Coord { rank: 2, file: 'd' },
        0b0000100000001000000010000000100000001000000010001111011100001000,
    );

    rook_attacks.insert(
        Coord { rank: 2, file: 'e' },
        0b0001000000010000000100000001000000010000000100001110111100010000,
    );

    rook_attacks.insert(
        Coord { rank: 2, file: 'f' },
        0b0010000000100000001000000010000000100000001000001101111100100000,
    );

    rook_attacks.insert(
        Coord { rank: 2, file: 'g' },
        0b0100000001000000010000000100000001000000010000001011111101000000,
    );

    rook_attacks.insert(
        Coord { rank: 2, file: 'h' },
        0b1000000010000000100000001000000010000000100000000111111110000000,
    );

    rook_attacks.insert(
        Coord { rank: 3, file: 'a' },
        0b0000000100000001000000010000000100000001111111100000000100000001,
    );

    rook_attacks.insert(
        Coord { rank: 3, file: 'b' },
        0b0000001000000010000000100000001000000010111111010000001000000010,
    );

    rook_attacks.insert(
        Coord { rank: 3, file: 'c' },
        0b0000010000000100000001000000010000000100111110110000010000000100,
    );

    rook_attacks.insert(
        Coord { rank: 3, file: 'd' },
        0b0000100000001000000010000000100000001000111101110000100000001000,
    );

    rook_attacks.insert(
        Coord { rank: 3, file: 'e' },
        0b0001000000010000000100000001000000010000111011110001000000010000,
    );

    rook_attacks.insert(
        Coord { rank: 3, file: 'f' },
        0b0010000000100000001000000010000000100000110111110010000000100000,
    );

    rook_attacks.insert(
        Coord { rank: 3, file: 'g' },
        0b0100000001000000010000000100000001000000101111110100000001000000,
    );

    rook_attacks.insert(
        Coord { rank: 3, file: 'h' },
        0b1000000010000000100000001000000010000000011111111000000010000000,
    );

    rook_attacks.insert(
        Coord { rank: 4, file: 'a' },
        0b0000000100000001000000010000000111111110000000010000000100000001,
    );

    rook_attacks.insert(
        Coord { rank: 4, file: 'b' },
        0b0000001000000010000000100000001011111101000000100000001000000010,
    );

    rook_attacks.insert(
        Coord { rank: 4, file: 'c' },
        0b0000010000000100000001000000010011111011000001000000010000000100,
    );

    rook_attacks.insert(
        Coord { rank: 4, file: 'd' },
        0b0000100000001000000010000000100011110111000010000000100000001000,
    );

    rook_attacks.insert(
        Coord { rank: 4, file: 'e' },
        0b0001000000010000000100000001000011101111000100000001000000010000,
    );

    rook_attacks.insert(
        Coord { rank: 4, file: 'f' },
        0b0010000000100000001000000010000011011111001000000010000000100000,
    );

    rook_attacks.insert(
        Coord { rank: 4, file: 'g' },
        0b0100000001000000010000000100000010111111010000000100000001000000,
    );

    rook_attacks.insert(
        Coord { rank: 4, file: 'h' },
        0b1000000010000000100000001000000001111111100000001000000010000000,
    );

    rook_attacks.insert(
        Coord { rank: 5, file: 'a' },
        0b0000000100000001000000011111111000000001000000010000000100000001,
    );

    rook_attacks.insert(
        Coord { rank: 5, file: 'b' },
        0b0000001000000010000000101111110100000010000000100000001000000010,
    );

    rook_attacks.insert(
        Coord { rank: 5, file: 'c' },
        0b0000010000000100000001001111101100000100000001000000010000000100,
    );

    rook_attacks.insert(
        Coord { rank: 5, file: 'd' },
        0b0000100000001000000010001111011100001000000010000000100000001000,
    );

    rook_attacks.insert(
        Coord { rank: 5, file: 'e' },
        0b0001000000010000000100001110111100010000000100000001000000010000,
    );

    rook_attacks.insert(
        Coord { rank: 5, file: 'f' },
        0b0010000000100000001000001101111100100000001000000010000000100000,
    );

    rook_attacks.insert(
        Coord { rank: 5, file: 'g' },
        0b0100000001000000010000001011111101000000010000000100000001000000,
    );

    rook_attacks.insert(
        Coord { rank: 5, file: 'h' },
        0b1000000010000000100000000111111110000000100000001000000010000000,
    );

    rook_attacks.insert(
        Coord { rank: 6, file: 'a' },
        0b0000000100000001111111100000000100000001000000010000000100000001,
    );

    rook_attacks.insert(
        Coord { rank: 6, file: 'b' },
        0b0000001000000010111111010000001000000010000000100000001000000010,
    );

    rook_attacks.insert(
        Coord { rank: 6, file: 'c' },
        0b0000010000000100111110110000010000000100000001000000010000000100,
    );

    rook_attacks.insert(
        Coord { rank: 6, file: 'd' },
        0b0000100000001000111101110000100000001000000010000000100000001000,
    );

    rook_attacks.insert(
        Coord { rank: 6, file: 'e' },
        0b0001000000010000111011110001000000010000000100000001000000010000,
    );

    rook_attacks.insert(
        Coord { rank: 6, file: 'f' },
        0b0010000000100000110111110010000000100000001000000010000000100000,
    );

    rook_attacks.insert(
        Coord { rank: 6, file: 'g' },
        0b0100000001000000101111110100000001000000010000000100000001000000,
    );

    rook_attacks.insert(
        Coord { rank: 6, file: 'h' },
        0b1000000010000000011111111000000010000000100000001000000010000000,
    );

    rook_attacks.insert(
        Coord { rank: 7, file: 'a' },
        0b0000000111111110000000010000000100000001000000010000000100000001,
    );

    rook_attacks.insert(
        Coord { rank: 7, file: 'b' },
        0b0000001011111101000000100000001000000010000000100000001000000010,
    );

    rook_attacks.insert(
        Coord { rank: 7, file: 'c' },
        0b0000010011111011000001000000010000000100000001000000010000000100,
    );

    rook_attacks.insert(
        Coord { rank: 7, file: 'd' },
        0b0000100011110111000010000000100000001000000010000000100000001000,
    );

    rook_attacks.insert(
        Coord { rank: 7, file: 'e' },
        0b0001000011101111000100000001000000010000000100000001000000010000,
    );

    rook_attacks.insert(
        Coord { rank: 7, file: 'f' },
        0b0010000011011111001000000010000000100000001000000010000000100000,
    );

    rook_attacks.insert(
        Coord { rank: 7, file: 'g' },
        0b0100000010111111010000000100000001000000010000000100000001000000,
    );

    rook_attacks.insert(
        Coord { rank: 7, file: 'h' },
        0b1000000001111111100000001000000010000000100000001000000010000000,
    );

    rook_attacks.insert(
        Coord { rank: 8, file: 'a' },
        0b1111111000000001000000010000000100000001000000010000000100000001,
    );

    rook_attacks.insert(
        Coord { rank: 8, file: 'b' },
        0b1111110100000010000000100000001000000010000000100000001000000010,
    );

    rook_attacks.insert(
        Coord { rank: 8, file: 'c' },
        0b1111101100000100000001000000010000000100000001000000010000000100,
    );

    rook_attacks.insert(
        Coord { rank: 8, file: 'd' },
        0b1111011100001000000010000000100000001000000010000000100000001000,
    );

    rook_attacks.insert(
        Coord { rank: 8, file: 'e' },
        0b1110111100010000000100000001000000010000000100000001000000010000,
    );

    rook_attacks.insert(
        Coord { rank: 8, file: 'f' },
        0b1101111100100000001000000010000000100000001000000010000000100000,
    );

    rook_attacks.insert(
        Coord { rank: 8, file: 'g' },
        0b1011111101000000010000000100000001000000010000000100000001000000,
    );

    rook_attacks.insert(
        Coord { rank: 8, file: 'h' },
        0b0111111110000000100000001000000010000000100000001000000010000000,
    );

    let mut queen_attacks: HashMap<Coord, u64> = HashMap::new();
    queen_attacks.insert(
        Coord { rank: 1, file: 'a' },
        0b1000000101000001001000010001000100001001000001010000001111111110,
    );

    queen_attacks.insert(
        Coord { rank: 1, file: 'b' },
        0b0000001010000010010000100010001000010010000010100000011111111101,
    );

    queen_attacks.insert(
        Coord { rank: 1, file: 'c' },
        0b0000010000000100100001000100010000100100000101010000111011111011,
    );

    queen_attacks.insert(
        Coord { rank: 1, file: 'd' },
        0b0000100000001000000010001000100001001001001010100001110011110111,
    );

    queen_attacks.insert(
        Coord { rank: 1, file: 'e' },
        0b0001000000010000000100000001000110010010010101000011100011101111,
    );

    queen_attacks.insert(
        Coord { rank: 1, file: 'f' },
        0b0010000000100000001000010010001000100100101010000111000011011111,
    );

    queen_attacks.insert(
        Coord { rank: 1, file: 'g' },
        0b0100000001000001010000100100010001001000010100001110000010111111,
    );

    queen_attacks.insert(
        Coord { rank: 1, file: 'h' },
        0b1000000110000010100001001000100010010000101000001100000001111111,
    );

    queen_attacks.insert(
        Coord { rank: 2, file: 'a' },
        0b0100000100100001000100010000100100000101000000111111111000000011,
    );

    queen_attacks.insert(
        Coord { rank: 2, file: 'b' },
        0b1000001001000010001000100001001000001010000001111111110100000111,
    );

    queen_attacks.insert(
        Coord { rank: 2, file: 'c' },
        0b0000010010000100010001000010010000010101000011101111101100001110,
    );

    queen_attacks.insert(
        Coord { rank: 2, file: 'd' },
        0b0000100000001000100010000100100100101010000111001111011100011100,
    );

    queen_attacks.insert(
        Coord { rank: 2, file: 'e' },
        0b0001000000010000000100011001001001010100001110001110111100111000,
    );

    queen_attacks.insert(
        Coord { rank: 2, file: 'f' },
        0b0010000000100001001000100010010010101000011100001101111101110000,
    );

    queen_attacks.insert(
        Coord { rank: 2, file: 'g' },
        0b0100000101000010010001000100100001010000111000001011111111100000,
    );

    queen_attacks.insert(
        Coord { rank: 2, file: 'h' },
        0b1000001010000100100010001001000010100000110000000111111111000000,
    );

    queen_attacks.insert(
        Coord { rank: 3, file: 'a' },
        0b0010000100010001000010010000010100000011111111100000001100000101,
    );

    queen_attacks.insert(
        Coord { rank: 3, file: 'b' },
        0b0100001000100010000100100000101000000111111111010000011100001010,
    );

    queen_attacks.insert(
        Coord { rank: 3, file: 'c' },
        0b1000010001000100001001000001010100001110111110110000111000010101,
    );

    queen_attacks.insert(
        Coord { rank: 3, file: 'd' },
        0b0000100010001000010010010010101000011100111101110001110000101010,
    );

    queen_attacks.insert(
        Coord { rank: 3, file: 'e' },
        0b0001000000010001100100100101010000111000111011110011100001010100,
    );

    queen_attacks.insert(
        Coord { rank: 3, file: 'f' },
        0b0010000100100010001001001010100001110000110111110111000010101000,
    );

    queen_attacks.insert(
        Coord { rank: 3, file: 'g' },
        0b0100001001000100010010000101000011100000101111111110000001010000,
    );

    queen_attacks.insert(
        Coord { rank: 3, file: 'h' },
        0b1000010010001000100100001010000011000000011111111100000010100000,
    );

    queen_attacks.insert(
        Coord { rank: 4, file: 'a' },
        0b0001000100001001000001010000001111111110000000110000010100001001,
    );

    queen_attacks.insert(
        Coord { rank: 4, file: 'b' },
        0b0010001000010010000010100000011111111101000001110000101000010010,
    );

    queen_attacks.insert(
        Coord { rank: 4, file: 'c' },
        0b0100010000100100000101010000111011111011000011100001010100100100,
    );

    queen_attacks.insert(
        Coord { rank: 4, file: 'd' },
        0b1000100001001001001010100001110011110111000111000010101001001001,
    );

    queen_attacks.insert(
        Coord { rank: 4, file: 'e' },
        0b0001000110010010010101000011100011101111001110000101010010010010,
    );

    queen_attacks.insert(
        Coord { rank: 4, file: 'f' },
        0b0010001000100100101010000111000011011111011100001010100000100100,
    );

    queen_attacks.insert(
        Coord { rank: 4, file: 'g' },
        0b0100010001001000010100001110000010111111111000000101000001001000,
    );

    queen_attacks.insert(
        Coord { rank: 4, file: 'h' },
        0b1000100010010000101000001100000001111111110000001010000010010000,
    );

    queen_attacks.insert(
        Coord { rank: 5, file: 'a' },
        0b0000100100000101000000111111111000000011000001010000100100010001,
    );

    queen_attacks.insert(
        Coord { rank: 5, file: 'b' },
        0b0001001000001010000001111111110100000111000010100001001000100010,
    );

    queen_attacks.insert(
        Coord { rank: 5, file: 'c' },
        0b0010010000010101000011101111101100001110000101010010010001000100,
    );

    queen_attacks.insert(
        Coord { rank: 5, file: 'd' },
        0b0100100100101010000111001111011100011100001010100100100110001000,
    );

    queen_attacks.insert(
        Coord { rank: 5, file: 'e' },
        0b1001001001010100001110001110111100111000010101001001001000010001,
    );

    queen_attacks.insert(
        Coord { rank: 5, file: 'f' },
        0b0010010010101000011100001101111101110000101010000010010000100010,
    );

    queen_attacks.insert(
        Coord { rank: 5, file: 'g' },
        0b0100100001010000111000001011111111100000010100000100100001000100,
    );

    queen_attacks.insert(
        Coord { rank: 5, file: 'h' },
        0b1001000010100000110000000111111111000000101000001001000010001000,
    );

    queen_attacks.insert(
        Coord { rank: 6, file: 'a' },
        0b0000010100000011111111100000001100000101000010010001000100100001,
    );

    queen_attacks.insert(
        Coord { rank: 6, file: 'b' },
        0b0000101000000111111111010000011100001010000100100010001001000010,
    );

    queen_attacks.insert(
        Coord { rank: 6, file: 'c' },
        0b0001010100001110111110110000111000010101001001000100010010000100,
    );

    queen_attacks.insert(
        Coord { rank: 6, file: 'd' },
        0b0010101000011100111101110001110000101010010010011000100000001000,
    );

    queen_attacks.insert(
        Coord { rank: 6, file: 'e' },
        0b0101010000111000111011110011100001010100100100100001000100010000,
    );

    queen_attacks.insert(
        Coord { rank: 6, file: 'f' },
        0b1010100001110000110111110111000010101000001001000010001000100001,
    );

    queen_attacks.insert(
        Coord { rank: 6, file: 'g' },
        0b0101000011100000101111111110000001010000010010000100010001000010,
    );

    queen_attacks.insert(
        Coord { rank: 6, file: 'h' },
        0b1010000011000000011111111100000010100000100100001000100010000100,
    );

    queen_attacks.insert(
        Coord { rank: 7, file: 'a' },
        0b0000001111111110000000110000010100001001000100010010000101000001,
    );

    queen_attacks.insert(
        Coord { rank: 7, file: 'b' },
        0b0000011111111101000001110000101000010010001000100100001010000010,
    );

    queen_attacks.insert(
        Coord { rank: 7, file: 'c' },
        0b0000111011111011000011100001010100100100010001001000010000000100,
    );

    queen_attacks.insert(
        Coord { rank: 7, file: 'd' },
        0b0001110011110111000111000010101001001001100010000000100000001000,
    );

    queen_attacks.insert(
        Coord { rank: 7, file: 'e' },
        0b0011100011101111001110000101010010010010000100010001000000010000,
    );

    queen_attacks.insert(
        Coord { rank: 7, file: 'f' },
        0b0111000011011111011100001010100000100100001000100010000100100000,
    );

    queen_attacks.insert(
        Coord { rank: 7, file: 'g' },
        0b1110000010111111111000000101000001001000010001000100001001000001,
    );

    queen_attacks.insert(
        Coord { rank: 7, file: 'h' },
        0b1100000001111111110000001010000010010000100010001000010010000010,
    );

    queen_attacks.insert(
        Coord { rank: 8, file: 'a' },
        0b1111111000000011000001010000100100010001001000010100000110000001,
    );

    queen_attacks.insert(
        Coord { rank: 8, file: 'b' },
        0b1111110100000111000010100001001000100010010000101000001000000010,
    );

    queen_attacks.insert(
        Coord { rank: 8, file: 'c' },
        0b1111101100001110000101010010010001000100100001000000010000000100,
    );

    queen_attacks.insert(
        Coord { rank: 8, file: 'd' },
        0b1111011100011100001010100100100110001000000010000000100000001000,
    );

    queen_attacks.insert(
        Coord { rank: 8, file: 'e' },
        0b1110111100111000010101001001001000010001000100000001000000010000,
    );

    queen_attacks.insert(
        Coord { rank: 8, file: 'f' },
        0b1101111101110000101010000010010000100010001000010010000000100000,
    );

    queen_attacks.insert(
        Coord { rank: 8, file: 'g' },
        0b1011111111100000010100000100100001000100010000100100000101000000,
    );

    queen_attacks.insert(
        Coord { rank: 8, file: 'h' },
        0b0111111111000000101000001001000010001000100001001000001010000001,
    );

    let mut king_attacks: HashMap<Coord, u64> = HashMap::new();
    king_attacks.insert(
        Coord { rank: 1, file: 'a' },
        0b0000000000000000000000000000000000000000000000000000001100000010,
    );

    king_attacks.insert(
        Coord { rank: 1, file: 'b' },
        0b0000000000000000000000000000000000000000000000000000011100000101,
    );

    king_attacks.insert(
        Coord { rank: 1, file: 'c' },
        0b0000000000000000000000000000000000000000000000000000111000001010,
    );

    king_attacks.insert(
        Coord { rank: 1, file: 'd' },
        0b0000000000000000000000000000000000000000000000000001110000010100,
    );

    king_attacks.insert(
        Coord { rank: 1, file: 'e' },
        0b0000000000000000000000000000000000000000000000000011100000101000,
    );

    king_attacks.insert(
        Coord { rank: 1, file: 'f' },
        0b0000000000000000000000000000000000000000000000000111000001010000,
    );

    king_attacks.insert(
        Coord { rank: 1, file: 'g' },
        0b0000000000000000000000000000000000000000000000001110000010100000,
    );

    king_attacks.insert(
        Coord { rank: 1, file: 'h' },
        0b0000000000000000000000000000000000000000000000001100000001000000,
    );

    king_attacks.insert(
        Coord { rank: 2, file: 'a' },
        0b0000000000000000000000000000000000000000000000110000001000000011,
    );

    king_attacks.insert(
        Coord { rank: 2, file: 'b' },
        0b0000000000000000000000000000000000000000000001110000010100000111,
    );

    king_attacks.insert(
        Coord { rank: 2, file: 'c' },
        0b0000000000000000000000000000000000000000000011100000101000001110,
    );

    king_attacks.insert(
        Coord { rank: 2, file: 'd' },
        0b0000000000000000000000000000000000000000000111000001010000011100,
    );

    king_attacks.insert(
        Coord { rank: 2, file: 'e' },
        0b0000000000000000000000000000000000000000001110000010100000111000,
    );

    king_attacks.insert(
        Coord { rank: 2, file: 'f' },
        0b0000000000000000000000000000000000000000011100000101000001110000,
    );

    king_attacks.insert(
        Coord { rank: 2, file: 'g' },
        0b0000000000000000000000000000000000000000111000001010000011100000,
    );

    king_attacks.insert(
        Coord { rank: 2, file: 'h' },
        0b0000000000000000000000000000000000000000110000000100000011000000,
    );

    king_attacks.insert(
        Coord { rank: 3, file: 'a' },
        0b0000000000000000000000000000000000000011000000100000001100000000,
    );

    king_attacks.insert(
        Coord { rank: 3, file: 'b' },
        0b0000000000000000000000000000000000000111000001010000011100000000,
    );

    king_attacks.insert(
        Coord { rank: 3, file: 'c' },
        0b0000000000000000000000000000000000001110000010100000111000000000,
    );

    king_attacks.insert(
        Coord { rank: 3, file: 'd' },
        0b0000000000000000000000000000000000011100000101000001110000000000,
    );

    king_attacks.insert(
        Coord { rank: 3, file: 'e' },
        0b0000000000000000000000000000000000111000001010000011100000000000,
    );

    king_attacks.insert(
        Coord { rank: 3, file: 'f' },
        0b0000000000000000000000000000000001110000010100000111000000000000,
    );

    king_attacks.insert(
        Coord { rank: 3, file: 'g' },
        0b0000000000000000000000000000000011100000101000001110000000000000,
    );

    king_attacks.insert(
        Coord { rank: 3, file: 'h' },
        0b0000000000000000000000000000000011000000010000001100000000000000,
    );

    king_attacks.insert(
        Coord { rank: 4, file: 'a' },
        0b0000000000000000000000000000001100000010000000110000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 4, file: 'b' },
        0b0000000000000000000000000000011100000101000001110000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 4, file: 'c' },
        0b0000000000000000000000000000111000001010000011100000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 4, file: 'd' },
        0b0000000000000000000000000001110000010100000111000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 4, file: 'e' },
        0b0000000000000000000000000011100000101000001110000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 4, file: 'f' },
        0b0000000000000000000000000111000001010000011100000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 4, file: 'g' },
        0b0000000000000000000000001110000010100000111000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 4, file: 'h' },
        0b0000000000000000000000001100000001000000110000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 5, file: 'a' },
        0b0000000000000000000000110000001000000011000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 5, file: 'b' },
        0b0000000000000000000001110000010100000111000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 5, file: 'c' },
        0b0000000000000000000011100000101000001110000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 5, file: 'd' },
        0b0000000000000000000111000001010000011100000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 5, file: 'e' },
        0b0000000000000000001110000010100000111000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 5, file: 'f' },
        0b0000000000000000011100000101000001110000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 5, file: 'g' },
        0b0000000000000000111000001010000011100000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 5, file: 'h' },
        0b0000000000000000110000000100000011000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 6, file: 'a' },
        0b0000000000000011000000100000001100000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 6, file: 'b' },
        0b0000000000000111000001010000011100000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 6, file: 'c' },
        0b0000000000001110000010100000111000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 6, file: 'd' },
        0b0000000000011100000101000001110000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 6, file: 'e' },
        0b0000000000111000001010000011100000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 6, file: 'f' },
        0b0000000001110000010100000111000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 6, file: 'g' },
        0b0000000011100000101000001110000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 6, file: 'h' },
        0b0000000011000000010000001100000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 7, file: 'a' },
        0b0000001100000010000000110000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 7, file: 'b' },
        0b0000011100000101000001110000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 7, file: 'c' },
        0b0000111000001010000011100000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 7, file: 'd' },
        0b0001110000010100000111000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 7, file: 'e' },
        0b0011100000101000001110000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 7, file: 'f' },
        0b0111000001010000011100000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 7, file: 'g' },
        0b1110000010100000111000000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 7, file: 'h' },
        0b1100000001000000110000000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 8, file: 'a' },
        0b0000001000000011000000000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 8, file: 'b' },
        0b0000010100000111000000000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 8, file: 'c' },
        0b0000101000001110000000000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 8, file: 'd' },
        0b0001010000011100000000000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 8, file: 'e' },
        0b0010100000111000000000000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 8, file: 'f' },
        0b0101000001110000000000000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 8, file: 'g' },
        0b1010000011100000000000000000000000000000000000000000000000000000,
    );

    king_attacks.insert(
        Coord { rank: 8, file: 'h' },
        0b0100000011000000000000000000000000000000000000000000000000000000,
    );

    return;
    uci::uci_loop();
}
