use std::collections::HashMap;
use std::sync::LazyLock;

pub static ROOK_RANK_MASK: LazyLock<[[u64; 255]; 8]> =
    LazyLock::new(|| generate_rook_rank_loop_up_mask());

pub static ANTI_DIAG_MASK: LazyLock<[u64; 8]> = LazyLock::new(|| generate_anti_diag_mask());

pub static DIAG_MASK: LazyLock<[u64; 8]> = LazyLock::new(|| generate_diag_mask());

pub static KNIGHT_MASK: LazyLock<[u64; 64]> = LazyLock::new(|| generate_knight_mask());
pub static KING_MASK: LazyLock<[u64; 64]> = LazyLock::new(|| generate_king_mask());

pub static PAWN_WHITE_DIAG_MASK: LazyLock<[u64; 64]> =
    LazyLock::new(|| generate_white_pawn_diag_mask());

pub static PAWN_BLACK_DIAG_MASK: LazyLock<[u64; 64]> =
    LazyLock::new(|| generate_black_pawn_diag_mask());

fn generate_rook_rank_loop_up_mask() -> [[u64; 255]; 8] {
    let mut attacks: [[u64; 255]; 8] = [[0; 255]; 8];

    for index in 0..8 {
        let mut masks_list = [0u64; 255];
        for i in 0..=255 {
            if (i >> index) & 1 != 0 {
                continue;
            }
            masks_list[i] = i as u64;
        }

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
            attacks[index as usize][mask as usize] = moves;
        }
    }
    attacks
}

fn generate_anti_diag_mask() -> [u64; 8] {
    /*
        To generate the anti-diagonal mask, you do +9 to move alongside the corresponding diagonal and -8 for the next step
        We only need to generate the upper anti-diagonal
        Anti-diag index in the lookup table: file - a + 8 - rank (for the upper anti-diagonal)
        Anti-diag index in the lookup table: 8 - (file - a + rank) (for the lower anti-diagonal)

        56 57 58 59 60 61 62 63   ← Rank 8
        48 49 50 51 52 53 54 55   ← Rank 7
        40 41 42 43 44 45 46 47   ← Rank 6
        32 33 34 35 36 37 38 39   ← Rank 5
        24 25 26 27 28 29 30 31   ← Rank 4
        16 17 18 19 20 21 22 23   ← Rank 3
        08 09 10 11 12 13 14 15   ← Rank 2
        00 01 02 03 04 05 06 07   ← Rank 1
        ↑
        File A
    */

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

    anti_diag_mask_list
}

fn generate_diag_mask() -> [u64; 8] {
    /*
        To generate the     diagonal mask, you do +7 to move alongside the corresponding diagonal and +1 for the next step
        We only need to generate the lower diagonal
        Diag index in the lookup table: file - a + rank - 1 (for the lower diagonal)
        Diag index in the lookup table: h - file + 8 - rank (for the upper diagonal)

        56 57 58 59 60 61 62 63   ← Rank 8
        48 49 50 51 52 53 54 55   ← Rank 7
        40 41 42 43 44 45 46 47   ← Rank 6
        32 33 34 35 36 37 38 39   ← Rank 5
        24 25 26 27 28 29 30 31   ← Rank 4
        16 17 18 19 20 21 22 23   ← Rank 3
        08 09 10 11 12 13 14 15   ← Rank 2
        00 01 02 03 04 05 06 07   ← Rank 1
        ↑
        File A
    */

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

    diag_mask_list
}

fn generate_knight_mask() -> [u64; 64] {
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

fn generate_king_mask() -> [u64; 64] {
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

fn generate_white_pawn_diag_mask() -> [u64; 64] {
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

fn generate_black_pawn_diag_mask() -> [u64; 64] {
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
