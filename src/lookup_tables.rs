use std::collections::HashSet;
use std::sync::LazyLock;
use crate::utils::random_u64_few_bits;

pub static LOOK_UP_TABLE: LazyLock<LookUpTable> = LazyLock::new(|| LookUpTable::init());

#[derive(Debug, PartialEq)]
pub struct LookUpTable {
    pub knight_attacks: [u64; 64],
    pub king_attacks: [u64; 64],
    pub white_pawn_attacks: [u64; 64],
    pub black_pawn_attacks: [u64; 64],
    pub rook_blockers_masks: [u64; 64],
    pub rook_magics: [u64; 64],
    pub rook_attacks: Box<[[u64; 4096]; 64]>,
    pub bishop_blockers_masks: [u64; 64],
    pub bishop_magics: [u64; 64],
    pub bishop_attacks: Box<[[u64; 512]; 64]>,
}

impl LookUpTable {
    pub fn init() -> LookUpTable {
        let rook_magics_and_attacks = generate_rook_attacks();
        let bishop_magics_and_attacks = generate_bishop_attacks();
        let bishop_anti_diag_blockers_masks = generate_bishop_anti_diag_blockers_masks();
        let bishop_diag_blockers_masks = generate_bishop_diag_blockers_masks();

        let mut bishop_blockers_masks = [0u64; 64];
        for i in 0..64 {
            bishop_blockers_masks[i] =
                bishop_anti_diag_blockers_masks[i] | bishop_diag_blockers_masks[i];
        }

        LookUpTable {
            knight_attacks: generate_knight_mask(),
            king_attacks: generate_king_mask(),
            white_pawn_attacks: generate_white_pawn_attacks(),
            black_pawn_attacks: generate_black_pawn_attacks(),

            rook_blockers_masks: generate_rook_blockers_masks(),
            rook_magics: rook_magics_and_attacks.0,
            rook_attacks: rook_magics_and_attacks.1,

            bishop_blockers_masks,
            bishop_magics: bishop_magics_and_attacks.0,
            bishop_attacks: bishop_magics_and_attacks.1,
        }
    }
}


pub fn generate_rook_attacks() -> ([u64; 64], Box<[[u64; 4096]; 64]>) {
    let mut magic_numbers = [0u64; 64];
    let mut attacks = Box::new([[0u64; 4096]; 64]);

    for square in 0..64 {
        let occupancy_combos = generate_rook_occupancy_combos(&square);
        loop {
            let mut ok = true;
            let magic: u64 = random_u64_few_bits(); // Increase the chances of finding a magic number
            attacks[square as usize] = [0u64; 4096];

            for (occupancy, moves) in occupancy_combos {
                let index = occupancy.wrapping_mul(magic) >> (64 - 12);
                assert!(
                    index < 4096,
                    "Index {} out of range for square {}",
                    index,
                    square
                );
                if attacks[square as usize][index as usize] == 0 {
                    attacks[square as usize][index as usize] = moves;
                } else if attacks[square as usize][index as usize] != moves {
                    ok = false;
                    break;
                }
            }
            magic_numbers[square as usize] = magic;
            if ok {
                break;
            }
        }
    }
    (magic_numbers, attacks)
}

pub fn generate_bishop_attacks() -> ([u64; 64], Box<[[u64; 512]; 64]>) {
    let mut magic_numbers = [0u64; 64];
    let mut attacks = Box::new([[0u64; 512]; 64]);

    for square in 0..64 {
        let occupancy_combos = generate_bishop_occupancy_combos(&square);
        loop {
            let mut ok = true;
            let magic: u64 = random_u64_few_bits(); // Increase the chances of finding a magic number
            attacks[square as usize] = [0u64; 512];

            for (occupancy, moves) in occupancy_combos.clone() {
                let index = occupancy.wrapping_mul(magic) >> (64 - 9);
                assert!(
                    index < 512,
                    "Index {} out of range for square {}",
                    index,
                    square
                );
                if attacks[square as usize][index as usize] == 0 {
                    attacks[square as usize][index as usize] = moves;
                } else if attacks[square as usize][index as usize] != moves {
                    ok = false;
                    break;
                }
            }
            magic_numbers[square as usize] = magic;
            if ok {
                break;
            }
        }
    }
    (magic_numbers, attacks)
}

pub fn generate_rook_occupancy_combos(square: &u8) -> [(u64, u64); 4096] {
    let mut occupancy_combos = [(0, 0); 4096];
    let file = square % 8;
    let rank = square / 8;

    let file_occupancy = generate_rook_rank_1_occupancy(&rank);
    let rank_occupancy = generate_rook_rank_1_occupancy(&file);

    let mut index = 0;
    for (mut file_mask, mut file_moves) in file_occupancy {
        file_mask = rank_to_file(&file_mask);
        file_mask <<= file;

        file_moves = rank_to_file(&file_moves);
        file_moves <<= file;

        for (mut rank_mask, mut rank_moves) in rank_occupancy {
            rank_mask <<= rank * 8;
            rank_moves <<= rank * 8;
            occupancy_combos[index] = (file_mask | rank_mask, file_moves | rank_moves);
            index += 1;
        }
    }

    occupancy_combos
}

pub fn generate_bishop_occupancy_combos(square: &u8) -> HashSet<(u64, u64)> {
    let file = square % 8;
    let rank = square / 8;

    let reference = [0, 1, 2, 3, 4, 5, 6, 7];
    let distance_to_anti_diag: i8 = (file - reference[rank as usize]) as i8;
    let anti_diag_occupancy = generate_bishop_anti_diag_occupancy(&(rank));

    let mut final_anti_diag_occupancy = [(0, 0); 64];
    let anti_diag_blockers_masks = generate_bishop_anti_diag_blockers_masks()[*square as usize];
    let anti_diag_masks = generate_bishop_anti_diag_masks()[*square as usize];
    let mut i = 0;
    for (mut mask, mut moves) in anti_diag_occupancy {
        if distance_to_anti_diag < 0 {
            mask >>= -1 * distance_to_anti_diag; // Move to the left;
            moves >>= -1 * distance_to_anti_diag; // Move to the left;
        } else if distance_to_anti_diag > 0 {
            mask <<= distance_to_anti_diag; // Move to the right
            moves <<= distance_to_anti_diag; // Move to the right
        }
        mask &= anti_diag_blockers_masks;
        moves &= anti_diag_masks;
        final_anti_diag_occupancy[i] = (mask, moves);
        i += 1;
    }

    let distance_to_diag: i8 = (file - reference[(7 - rank) as usize]) as i8;
    let diag_occupancy = generate_bishop_diag_occupancy(&(7 - rank));

    let mut final_diag_occupancy = [(0, 0); 64];
    let diag_blockers_masks = generate_bishop_diag_blockers_masks()[*square as usize];
    let diag_masks = generate_bishop_diag_masks()[*square as usize];
    let mut i = 0;
    for (mut mask, mut moves) in diag_occupancy {
        if distance_to_diag < 0 {
            mask >>= -1 * distance_to_diag; // Move to the left
            moves >>= -1 * distance_to_diag; // Move to the left
        } else if distance_to_diag > 0 {
            mask <<= distance_to_diag; // Move to the right
            moves <<= distance_to_diag; // Move to the right
        }
        mask &= diag_blockers_masks;
        moves &= diag_masks;
        final_diag_occupancy[i] = (mask, moves);
        i += 1;
    }

    let mut hash_table: HashSet<(u64, u64)> = HashSet::with_capacity(512);
    for (anti_diag_mask, anti_diag_moves) in final_anti_diag_occupancy {
        for (diag_mask, diag_moves) in final_diag_occupancy {
            hash_table.insert((anti_diag_mask | diag_mask, anti_diag_moves | diag_moves));
        }
    }

    hash_table
}

pub fn generate_rook_rank_1_occupancy(square: &u8) -> [(u64, u64); 64] {
    let mut attacks: [(u64, u64); 64] = [(0, 0); 64];
    let index: u64 = *square as u64;

    // IMPORTANT: There will 2^6 = 64 different possibilities for the blockers disposition of the file;
    // that is because at least 2 bits are force to always be 0
    // And the larger number that will represent a blocker configuration is 126 = 01111110 (rook on A1 or H1)
    let mut masks_list = [0u64; 64];
    let mut j = 0;
    for i in 0..=127 {
        if (i >> index) & 1 != 0 || (i >> 0) & 1 != 0 {
            continue;
        }
        masks_list[j] = i as u64;
        j += 1;
    }

    j = 0;
    for mask in masks_list {
        // left side
        let mut cursor: u64 = index + 1;
        let mut moves = 0;
        let mut process = true;
        while process && (1 << cursor) <= 128 {
            if (mask >> cursor) & 1 == 1 {
                process = false;
            }
            moves |= 1 << cursor;
            cursor += 1;
        }
        // right side
        if index > 0 {
            cursor = index - 1;
            process = true;
            while process {
                if (mask >> cursor) & 1 != 0 {
                    process = false;
                }
                moves |= 1 << cursor;
                if cursor == 0 {
                    break;
                }
                cursor -= 1;
            }
        }
        attacks[j] = (mask, moves);
        j += 1;
    }
    attacks
}

pub fn generate_bishop_anti_diag_occupancy(square: &u8) -> [(u64, u64); 64] {
    let temp_occupancy = generate_rook_rank_1_occupancy(square);
    let mut occupancy: [(u64, u64); 64] = [(0, 0); 64];

    let mut j = 0;
    for (mask, moves) in temp_occupancy {
        let final_mask = ((mask >> 0 & 1) << 0)
            | ((mask >> 1 & 1) << 9)
            | ((mask >> 2 & 1) << 18)
            | ((mask >> 3 & 1) << 27)
            | ((mask >> 4 & 1) << 36)
            | ((mask >> 5 & 1) << 45)
            | ((mask >> 6 & 1) << 54)
            | ((mask >> 7 & 1) << 63);

        let final_moves = ((moves >> 0 & 1) << 0)
            | ((moves >> 1 & 1) << 9)
            | ((moves >> 2 & 1) << 18)
            | ((moves >> 3 & 1) << 27)
            | ((moves >> 4 & 1) << 36)
            | ((moves >> 5 & 1) << 45)
            | ((moves >> 6 & 1) << 54)
            | ((moves >> 7 & 1) << 63);

        occupancy[j] = (final_mask, final_moves);
        j += 1;
    }
    occupancy
}

pub fn generate_bishop_diag_occupancy(square: &u8) -> [(u64, u64); 64] {
    let temp_occupancy = generate_rook_rank_1_occupancy(square);
    let mut occupancy: [(u64, u64); 64] = [(0, 0); 64];

    let mut j = 0;
    for (mask, moves) in temp_occupancy {
        let final_mask = ((mask >> 0 & 1) << 56)
            | ((mask >> 1 & 1) << 49)
            | ((mask >> 2 & 1) << 42)
            | ((mask >> 3 & 1) << 35)
            | ((mask >> 4 & 1) << 28)
            | ((mask >> 5 & 1) << 21)
            | ((mask >> 6 & 1) << 14)
            | ((mask >> 7 & 1) << 7);

        let final_moves = ((moves >> 0 & 1) << 56)
            | ((moves >> 1 & 1) << 49)
            | ((moves >> 2 & 1) << 42)
            | ((moves >> 3 & 1) << 35)
            | ((moves >> 4 & 1) << 28)
            | ((moves >> 5 & 1) << 21)
            | ((moves >> 6 & 1) << 14)
            | ((moves >> 7 & 1) << 7);

        occupancy[j] = (final_mask, final_moves);
        j += 1;
    }
    occupancy
}

pub fn rank_to_file(rank: &u64) -> u64 {
    ((rank >> 0) & 1u64) << 0
        | ((rank >> 1) & 1u64) << 8
        | ((rank >> 2) & 1u64) << 16
        | ((rank >> 3) & 1u64) << 24
        | ((rank >> 4) & 1u64) << 32
        | ((rank >> 5) & 1u64) << 40
        | ((rank >> 6) & 1u64) << 48
        | ((rank >> 7) & 1u64) << 56
}

pub fn generate_rook_blockers_masks() -> [u64; 64] {
    let mut masks = [0u64; 64];

    for square in 0..64 {
        let rank = square / 8;
        let file = square % 8;

        let mut rank_mask = (256 - 1) & !(1 << 0) & !(1 << 7) & !(1 << file); // 0111F110
        rank_mask <<= rank * 8;

        let mut file_mask = (256 - 1) & !(1 << 0) & !(1 << 7) & !(1 << rank);
        file_mask = rank_to_file(&file_mask);
        file_mask <<= file;

        masks[square as usize] = rank_mask | file_mask;
    }
    masks
}

pub fn generate_bishop_anti_diag_blockers_masks() -> [u64; 64] {
    let mut masks = [0u64; 64];

    let mut anti_diag_mask_list: [u64; 8] = [0u64; 8];
    for i in 0..8 {
        let mut anti_diag_mask: u64 = 0;
        let mut bit_index = 56 - 8 * i;
        while bit_index < 64 {
            anti_diag_mask |= 1u64 << bit_index;
            bit_index += 9;
        }
        anti_diag_mask &= !(1u64 << (56 - 8 * i));
        anti_diag_mask &= !(1u64 << (bit_index - 9));
        anti_diag_mask_list[i] = anti_diag_mask;
    }

    let anti_diagonal_reference = [0, 1, 2, 3, 4, 5, 6, 7];

    for square in 0..64 {
        let rank = square / 8;
        let file = square % 8;
        let distance_to_anti_diag: i8 = file - anti_diagonal_reference[rank as usize];

        masks[square as usize] = if distance_to_anti_diag < 0 {
            anti_diag_mask_list[file as usize + 7 - rank as usize]
        } else if distance_to_anti_diag > 0 {
            let index = 7 - file + rank;
            let mut temp_mask = anti_diag_mask_list[index as usize];
            temp_mask >>= (7 - index) * 8; // 7 - index times to the bottom
            temp_mask <<= 7 - index; // 7 - index times to the right
            temp_mask
        } else {
            // The anti-diag itself
            anti_diag_mask_list[7]
        };
        masks[square as usize] &= !(1u64 << square);
    }
    masks
}

pub fn generate_bishop_diag_blockers_masks() -> [u64; 64] {
    let mut masks = [0u64; 64];

    let mut diag_mask_list: [u64; 8] = [0u64; 8];
    for i in 0..8 {
        let mut diag_mask: u64 = 0;
        let mut bit_index: u64 = i;
        while bit_index <= i * 8 {
            diag_mask |= 1u64 << bit_index;
            bit_index += 7;
        }
        diag_mask &= !(1u64 << i);
        diag_mask &= !(1u64 << (bit_index - 7));
        diag_mask_list[i as usize] = diag_mask;
    }

    let diagonal_reference = [0, 1, 2, 3, 4, 5, 6, 7];

    for square in 0..64 {
        let rank = square / 8;
        let file = square % 8;
        let distance_to_diag: i8 = file - diagonal_reference[(7 - rank) as usize];

        masks[square as usize] = if distance_to_diag < 0 {
            diag_mask_list[file as usize + rank as usize]
        } else if distance_to_diag > 0 {
            let index = 7 - file + 7 - rank;
            let mut temp_mask = diag_mask_list[index as usize];
            temp_mask <<= (7 - index) * 8; // 7 - index times to the up
            temp_mask <<= 7 - index; // 7 - index times to the right
            temp_mask
        } else {
            // The diag itself
            diag_mask_list[7]
        };
        masks[square as usize] &= !(1u64 << square);
    }
    masks
}

pub fn generate_bishop_anti_diag_masks() -> [u64; 64] {
    let mut masks = [0u64; 64];

    let mut anti_diag_mask_list: [u64; 8] = [0u64; 8];
    for i in 0..8 {
        let mut anti_diag_mask: u64 = 0;
        let mut bit_index = 56 - 8 * i;
        while bit_index < 64 {
            anti_diag_mask |= 1u64 << bit_index;
            bit_index += 9;
        }
        anti_diag_mask_list[i] = anti_diag_mask;
    }

    let anti_diagonal_reference = [0, 1, 2, 3, 4, 5, 6, 7];

    for square in 0..64 {
        let rank = square / 8;
        let file = square % 8;
        let distance_to_anti_diag: i8 = file - anti_diagonal_reference[rank as usize];

        masks[square as usize] = if distance_to_anti_diag < 0 {
            anti_diag_mask_list[file as usize + 7 - rank as usize]
        } else if distance_to_anti_diag > 0 {
            let index = 7 - file + rank;
            let mut temp_mask = anti_diag_mask_list[index as usize];
            temp_mask >>= (7 - index) * 8; // 7 - index times to the bottom
            temp_mask <<= 7 - index; // 7 - index times to the right
            temp_mask
        } else {
            // The anti-diag itself
            anti_diag_mask_list[7]
        };
    }
    masks
}

pub fn generate_bishop_diag_masks() -> [u64; 64] {
    let mut masks = [0u64; 64];

    let mut diag_mask_list: [u64; 8] = [0u64; 8];
    for i in 0..8 {
        let mut diag_mask: u64 = 0;
        let mut bit_index: u64 = i;
        while bit_index <= i * 8 {
            diag_mask |= 1u64 << bit_index;
            bit_index += 7;
        }
        diag_mask_list[i as usize] = diag_mask;
    }

    let diagonal_reference = [0, 1, 2, 3, 4, 5, 6, 7];

    for square in 0..64 {
        let rank = square / 8;
        let file = square % 8;
        let distance_to_diag: i8 = file - diagonal_reference[(7 - rank) as usize];

        masks[square as usize] = if distance_to_diag < 0 {
            diag_mask_list[file as usize + rank as usize]
        } else if distance_to_diag > 0 {
            let index = 7 - file + 7 - rank;
            let mut temp_mask = diag_mask_list[index as usize];
            temp_mask <<= (7 - index) * 8; // 7 - index times to the up
            temp_mask <<= 7 - index; // 7 - index times to the right
            temp_mask
        } else {
            // The diag itself
            diag_mask_list[7]
        };
    }
    masks
}

pub fn generate_knight_mask() -> [u64; 64] {
    let mut knight_mask_list: [u64; 64] = [0; 64];

    for rank in 1..=8 {
        for file in 'a'..='h' {
            let r = rank - 1;
            let f = (file as u8 - 'a' as u8) as i8;
            let index = r * 8 + f as i8;
            let mut knight_mask: u64 = 0;

            if 0 <= f - 2 {
                if 0 <= r - 1 {
                    knight_mask |= 1u64 << ((r - 1) * 8 + f - 2);
                }
                if r + 1 <= 7 {
                    knight_mask |= 1u64 << ((r + 1) * 8 + f - 2);
                }
            }
            if f + 2 <= 7 {
                if 0 <= r - 1 {
                    knight_mask |= 1u64 << ((r - 1) * 8 + f + 2);
                }
                if r + 1 <= 7 {
                    knight_mask |= 1u64 << ((r + 1) * 8 + f + 2);
                }
            }

            if 0 <= f - 1 {
                if 0 <= r - 2 {
                    knight_mask |= 1u64 << ((r - 2) * 8 + f - 1);
                }
                if r + 2 <= 7 {
                    knight_mask |= 1u64 << ((r + 2) * 8 + f - 1);
                }
            }
            if f + 1 <= 7 {
                if 0 <= r - 2 {
                    knight_mask |= 1u64 << ((r - 2) * 8 + f + 1);
                }
                if r + 2 <= 7 {
                    knight_mask |= 1u64 << ((r + 2) * 8 + f + 1);
                }
            }
            knight_mask_list[index as usize] = knight_mask;
        }
    }

    knight_mask_list
}

pub fn generate_king_mask() -> [u64; 64] {
    let mut king_mask_list: [u64; 64] = [0; 64];

    for rank in 1..=8 {
        for file in 'a'..='h' {
            let r = rank - 1;
            let f = (file as u8 - 'a' as u8) as i8;
            let index = r * 8 + f as i8;
            let mut king_mask: u64 = 0;

            if 0 <= f - 1 {
                king_mask |= 1u64 << (r * 8 + f - 1);
                if 0 <= r - 1 {
                    king_mask |= 1u64 << ((r - 1) * 8 + f - 1);
                }
                if r + 1 <= 7 {
                    king_mask |= 1u64 << ((r + 1) * 8 + f - 1);
                }
            }
            if f + 1 <= 7 {
                king_mask |= 1u64 << (r * 8 + f + 1);
                if 0 <= r - 1 {
                    king_mask |= 1u64 << ((r - 1) * 8 + f + 1);
                }
                if r + 1 <= 7 {
                    king_mask |= 1u64 << ((r + 1) * 8 + f + 1);
                }
            }

            if 0 <= r - 1 {
                king_mask |= 1u64 << ((r - 1) * 8 + f);
            }
            if r + 1 <= 7 {
                king_mask |= 1u64 << ((r + 1) * 8 + f);
            }
            king_mask_list[index as usize] = king_mask;
        }
    }

    king_mask_list
}

pub fn generate_white_pawn_attacks() -> [u64; 64] {
    let mut white_pawn_mask_list: [u64; 64] = [0; 64];

    for rank in 1..=8 {
        for file in 'a'..='h' {
            let r = rank - 1;
            let f = (file as u8 - 'a' as u8) as i8;
            let index = r * 8 + f as i8;
            let mut mask: u64 = 0;
            if rank < 8 {
                // No upper left for the file a
                if 0 <= f - 1 {
                    mask |= 1 << (index + 7);
                }
                // No upper right for the file h
                if f + 1 <= 7 {
                    mask |= 1 << (index + 9);
                }
            }
            white_pawn_mask_list[index as usize] = mask;
        }
    }
    white_pawn_mask_list
}

pub fn generate_black_pawn_attacks() -> [u64; 64] {
    let mut black_pawn_mask_list: [u64; 64] = [0; 64];

    for rank in 1..=8 {
        for file in 'a'..='h' {
            let r = rank - 1;
            let f = (file as u8 - 'a' as u8) as i8;
            let index = r * 8 + f as i8;
            let mut mask: u64 = 0;
            if rank > 1 {
                // No lower right for the file 'h'
                if f + 1 <= 7 {
                    mask |= 1 << (index - 7);
                }
                // No lower left for the file a
                if 0 <= f - 1 {
                    mask |= 1 << (index - 9);
                }
            }
            black_pawn_mask_list[index as usize] = mask;
        }
    }

    black_pawn_mask_list
}
