use crate::piece::PieceColor;
use crate::position::Position;

const MAX_PLY: usize = 1024;

#[derive(Clone)]
pub struct History {
    history: Vec<u64>,
}

impl History {
    #[inline(always)]
    pub fn new() -> History {
        History { history: Vec::with_capacity(MAX_PLY) }
    }

    #[inline(always)]
    pub fn save_position(&mut self, position: &Position) {
        self.history.push(position.get_hash());
    }

    #[inline(always)]
    pub fn pop_last_entry(&mut self) {
        self.history.pop();
    }

    #[inline(always)]
    pub fn get_position_occurrences_count(&self, position: &Position) -> usize {
        let hash = position.get_hash();
        // history.iter().rev().take(halfmove_clock).filter(|&&k| k == zobrist).count();
        self.history.iter().rev().filter(|k| **k == hash).count()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.history.clear()
    }
}

#[derive(Clone, Copy)]
pub struct UndoMove {
    pub white_board: u64,
    pub black_board: u64,

    pub pawns_board: u64,
    pub knights_board: u64,
    pub bishops_board: u64,
    pub rooks_board: u64,
    pub queens_board: u64,
    pub kings_board: u64,

    pub white_king_square: u8,
    pub black_king_square: u8,
    pub castling_rights: u8, // 0 0 0 0 0(q) 0(k) 0(Q) 0(K)
    pub en_passant_file: u8,

    pub turn: PieceColor,
    pub number_of_move: u8,
    pub half_move_clock: u8,

    // code: u64,
    pub hash: u64,
}

impl UndoMove {
    #[inline(always)]
    pub fn new(position: &Position) -> UndoMove {
        UndoMove {
            white_board: position.get_white_board(),
            black_board: position.get_black_board(),

            pawns_board: position.get_pawns_board(),
            knights_board: position.get_knights_board(),
            bishops_board: position.get_bishops_board(),
            rooks_board: position.get_rooks_board(),
            queens_board: position.get_queens_board(),
            kings_board: position.get_kings_board(),

            white_king_square: position.get_white_king_square(),
            black_king_square: position.get_black_king_square(),
            castling_rights: position.get_castling_rights(),
            en_passant_file: position.get_en_passant() & 7,

            turn: position.get_turn(),
            number_of_move: position.get_number_of_move(),
            half_move_clock: position.get_half_move_clock(),

            hash: position.get_hash(),
        }
    }

    #[inline(always)]
    pub fn default() -> UndoMove {
        UndoMove {
            white_board: 0,
            black_board: 0,

            pawns_board: 0,
            knights_board: 0,
            bishops_board: 0,
            rooks_board: 0,
            queens_board: 0,
            kings_board: 0,

            white_king_square: 0,
            black_king_square: 0,
            castling_rights: 0,
            en_passant_file: 0,

            turn: PieceColor::None,
            number_of_move: 0,
            half_move_clock: 0,

            hash: 0,
        }
    }
}

































