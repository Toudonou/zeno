// Value from https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function

static MG_PAWN_VALUE:   i32 = 100;
static MG_KNIGHT_VALUE: i32 = 320;
static MG_BISHOP_VALUE: i32 = 330;
static MG_ROOK_VALUE:   i32 = 500;
static MG_QUEEN_VALUE:  i32 = 900;

static EG_PAWN_VALUE:   i32 = 150;
static EG_KNIGHT_VALUE: i32 = 500;
static EG_BISHOP_VALUE: i32 = 560;
static EG_ROOK_VALUE:   i32 = 835;
static EG_QUEEN_VALUE:  i32 = 1520;

static KING_VALUE: i32 = 10_000;

pub static MG_PIECES_VALUES: [i32; 6] = [MG_PAWN_VALUE, MG_KNIGHT_VALUE, MG_BISHOP_VALUE, MG_ROOK_VALUE, MG_QUEEN_VALUE, KING_VALUE];
pub static EG_PIECES_VALUES: [i32; 6] = [EG_PAWN_VALUE, EG_KNIGHT_VALUE, EG_BISHOP_VALUE, EG_ROOK_VALUE, EG_QUEEN_VALUE, KING_VALUE];

pub static PAWN_PHASE: i32 = 0;
pub static KNIGHT_PHASE: i32 = 1;
pub static BISHOP_PHASE: i32 = 1;
pub static ROOK_PHASE: i32 = 2;
pub static QUEEN_PHASE: i32 = 4;
pub static TOTAL_PHASE: i32 = PAWN_PHASE * 16 + KNIGHT_PHASE * 4 + BISHOP_PHASE * 4 + ROOK_PHASE * 4 + QUEEN_PHASE * 2;
pub static PHASE_TABLE: [i32; 6] = [PAWN_PHASE, KNIGHT_PHASE, BISHOP_PHASE, ROOK_PHASE, QUEEN_PHASE, 0];

#[rustfmt::skip]
pub static MG_PAWN_TABLE:[i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    98, 134,  61,  95,  95,  61, 134,  98,
    -6,   7,  26,  31,  31,  26,   7,  -6,
    -14,  13,   6,  21,  21,   6,  13, -14,
    -27,  -2,   5,  21,  21,   5,  -2, -27,
    -26,  -4,  -4,  10,  10,  -4,  -4, -26,
    -35,  -1, -20, -23, -23, -20,  -1, -35,
    0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
pub static EG_PAWN_TABLE:[i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    178, 173, 158, 134, 134, 158, 173, 178,
    94, 100,  85,  67,  67,  85, 100,  94,
    32,  24,  13,   5,   5,  13,  24,  32,
    13,   9,  -3,  -7,  -7,  -3,   9,  13,
    4,   7,  -6,   1,   1,  -6,   7,   4,
    13,   8,   8,  10,  10,   8,   8,  13,
    0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
pub static MG_KNIGHT_TABLE:[i32; 64] = [
    -167, -89, -34, -49, -49, -34, -89, -167,
    -73, -41,  72,  36,  36,  72, -41, -73,
    -47,  60,  37,  65,  65,  37,  60, -47,
    -9,  17,  19,  53,  53,  19,  17,  -9,
    -13,   4,  16,  13,  13,  16,   4, -13,
    -23,  -9,  25,  10,  10,  25,  -9, -23,
    -29, -53, -12,  -3,  -3, -12, -53, -29,
    -105, -21, -58, -33, -33, -58, -21, -105,
];

#[rustfmt::skip]
pub static EG_KNIGHT_TABLE:[i32; 64] = [
    -58, -38, -13, -28, -28, -13, -38, -58,
    -25,  -8, -25,  -2,  -2, -25,  -8, -25,
    -24, -20,  10,   9,   9,  10, -20, -24,
    -17,   3,  22,  22,  22,  22,   3, -17,
    -18,  -6,  16,  25,  25,  16,  -6, -18,
    -23,  -3,  -1,  15,  15,  -1,  -3, -23,
    -42, -20, -10,  -5,  -5, -10, -20, -42,
    -29, -51, -23, -15, -15, -23, -51, -29,
];

#[rustfmt::skip]
pub static MG_BISHOP_TABLE:[i32; 64] = [
    -29,   4, -82, -37, -37, -82,   4, -29,
    -26,  16, -18, -13, -13, -18,  16, -26,
    -16,  37,  43,  40,  40,  43,  37, -16,
    -4,   5,  19,  50,  50,  19,   5,  -4,
    -6,  13,  13,  26,  26,  13,  13,  -6,
    0,  15,  15,  15,  15,  15,  15,   0,
    4,  15,  16,   0,   0,  16,  15,   4,
    -33,  -3, -14, -21, -21, -14,  -3, -33,
];

#[rustfmt::skip]
pub static EG_BISHOP_TABLE:[i32; 64] = [
    -14, -21, -11,  -8,  -8, -11, -21, -14,
    -8,  -4,   7, -12, -12,   7,  -4,  -8,
    2,  -8,   0,  -1,  -1,   0,  -8,   2,
    -3,   9,  12,   9,   9,  12,   9,  -3,
    -6,   3,  13,  19,  19,  13,   3,  -6,
    -12,  -3,   8,  10,  10,   8,  -3, -12,
    -14, -18,  -7,  -1,  -1,  -7, -18, -14,
    -23,  -9, -23,  -5,  -5, -23,  -9, -23,
];

#[rustfmt::skip]
pub static MG_ROOK_TABLE:[i32; 64] = [
    32,  42,  32,  51,  51,  32,  42,  32,
    27,  32,  58,  62,  62,  58,  32,  27,
    -5,  19,  26,  36,  36,  26,  19,  -5,
    -24, -11,   7,  26,  26,   7, -11, -24,
    -36, -26, -12,  -1,  -1, -12, -26, -36,
    -45, -25, -16, -17, -17, -16, -25, -45,
    -44, -16, -20,  -9,  -9, -20, -16, -44,
    -19, -13,   1,  17,  17,   1, -13, -19,
];

#[rustfmt::skip]
pub static EG_ROOK_TABLE:[i32; 64] = [
    13,  10,  18,  15,  15,  18,  10,  13,
    11,  13,  13,  11,  11,  13,  13,  11,
    7,   7,   7,   5,   5,   7,   7,   7,
    4,   3,  13,   1,   1,  13,   3,   4,
    3,   5,   8,   4,   4,   8,   5,   3,
    -4,   0,  -5,  -1,  -1,  -5,   0,  -4,
    -6,  -6,   0,   2,   2,   0,  -6,  -6,
    -9,   2,   3,  -1,  -1,   3,   2,  -9,
];

#[rustfmt::skip]
pub static MG_QUEEN_TABLE:[i32; 64] = [
    -28,   0,  29,  12,  12,  29,   0, -28,
    -24, -39,  -5,   1,   1,  -5, -39, -24,
    -13, -17,   7,   8,   8,   7, -17, -13,
    -27, -27, -16, -16, -16, -16, -27, -27,
    -9, -26,  -9, -10, -10,  -9, -26,  -9,
    -14,   2, -11,  -2,  -2, -11,   2, -14,
    -35,  -8,  11,   2,   2,  11,  -8, -35,
    -1, -18,  -9,  10,  10,  -9, -18,  -1,
];

#[rustfmt::skip]
pub static EG_QUEEN_TABLE:[i32; 64] = [
    -9,  22,  22,  27,  27,  22,  22,  -9,
    -17,  20,  32,  41,  41,  32,  20, -17,
    -20,   6,   9,  49,  49,   9,   6, -20,
    3,  22,  24,  45,  45,  24,  22,   3,
    -18,  28,  19,  47,  47,  19,  28, -18,
    -16, -27,  15,   6,   6,  15, -27, -16,
    -22, -23, -30, -16, -16, -30, -23, -22,
    -33, -28, -22, -43, -43, -22, -28, -33,
];

#[rustfmt::skip]
pub static MG_KING_TABLE:[i32; 64] = [
    -65,  23,  16, -15, -15,  16,  23, -65,
    29,  -1, -20,  -7,  -7, -20,  -1,  29,
    -9,  24,   2, -16, -16,   2,  24,  -9,
    -17, -20, -12, -27, -27, -12, -20, -17,
    -49,  -1, -27, -39, -39, -27,  -1, -49,
    -14, -14, -22, -46, -46, -22, -14, -14,
    1,   7,  -8, -64, -64,  -8,   7,   1,
    -15,  50,  70, -54, -54,  12,  70, -15,
];

#[rustfmt::skip]
pub static EG_KING_TABLE:[i32; 64] = [
    -74, -35, -18, -18, -18, -18, -35, -74,
    -12,  17,  14,  17,  17,  14,  17, -12,
    10,  17,  23,  15,  15,  23,  17,  10,
    -8,  22,  24,  27,  27,  24,  22,  -8,
    -18,  -4,  21,  24,  24,  21,  -4, -18,
    -19,  -3,  11,  21,  21,  11,  -3, -19,
    -27, -11,   4,  13,  13,   4, -11, -27,
    -53, -34, -21, -11, -11, -21, -34, -53,
];

pub static MG_PIECES_SQUARES_TABLES: [[i32; 64]; 6] = [MG_PAWN_TABLE, MG_KNIGHT_TABLE, MG_BISHOP_TABLE, MG_ROOK_TABLE, MG_QUEEN_TABLE, MG_KING_TABLE];
pub static EG_PIECES_SQUARES_TABLES: [[i32; 64]; 6] = [EG_PAWN_TABLE, EG_KNIGHT_TABLE, EG_BISHOP_TABLE, EG_ROOK_TABLE, EG_QUEEN_TABLE, EG_KING_TABLE];
