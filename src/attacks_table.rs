use crate::utils::Coord;
use std::collections::HashMap;

pub fn pawns_attacks_table(coord: &Coord) -> u64 {
    4
}

pub fn knight_attacks_table(coord: &Coord) -> u64 {
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

    *knight_attacks.get(coord).unwrap()
}

pub fn bishop_attacks_table(coord: &Coord) -> u64 {
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

    *bishop_attacks.get(coord).unwrap()
}

pub fn rook_attacks_table(coord: &Coord) -> u64 {
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

    *rook_attacks.get(coord).unwrap()
}

pub fn queen_attacks_table(coord: &Coord) -> u64 {
    bishop_attacks_table(coord) | rook_attacks_table(coord)
}

// TODO: Generate castling for the king
pub fn king_attacks_table(coord: &Coord) -> u64 {
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

    *king_attacks.get(coord).unwrap()
}
