use crate::bitboard::BitBoard;
use crate::containers::{ByColor, ByPieceType};
use crate::lookup_tables::{get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawns_attacks, get_rook_attacks};
use crate::moves::{Move, MoveType};
use crate::piece::{Piece, PieceColor, PieceType};
use crate::square::Square;
use crate::utils::{PAWNS_OCCUPANCY_OBLIGATION_FOR_EN_PASSANT, TOTAL_PHASE, get_phase};
use crate::zobrist_hash::{BoardHash, ZobristHash};
use crate::{get_lsb, pop_lsb};

static WHITE_CAN_SHORT_CASTLE: u8 = 1u8 << 0;
static WHITE_CAN_LONG_CASTLE: u8 = 1u8 << 1;
static BLACK_CAN_SHORT_CASTLE: u8 = 1u8 << 2;
static BLACK_CAN_LONG_CASTLE: u8 = 1u8 << 3;
pub static CAN_SHORT_CASTLE: ByColor<u8> = ByColor::new(WHITE_CAN_SHORT_CASTLE, BLACK_CAN_SHORT_CASTLE);
pub static CAN_LONG_CASTLE: ByColor<u8> = ByColor::new(WHITE_CAN_LONG_CASTLE, BLACK_CAN_LONG_CASTLE);

static SHORT_CASTLE_ROOK_MASK: ByColor<BitBoard> = ByColor::new(1u64 << 7 | 1u64 << 5, 1u64 << 63 | 1u64 << 61);
static LONG_CASTLE_ROOK_MASK: ByColor<BitBoard> = ByColor::new(1u64 << 0 | 1u64 << 3, 1u64 << 56 | 1u64 << 59);

static CASTLING_AVAILABILITY_TABLE: [u8; 64] = {
  let mut table: [u8; 64] = [0; 64];

  table[0] = WHITE_CAN_LONG_CASTLE;
  table[7] = WHITE_CAN_SHORT_CASTLE;

  table[4] = WHITE_CAN_SHORT_CASTLE | WHITE_CAN_LONG_CASTLE;
  table[60] = BLACK_CAN_SHORT_CASTLE | BLACK_CAN_LONG_CASTLE;

  table[56] = BLACK_CAN_LONG_CASTLE;
  table[63] = BLACK_CAN_SHORT_CASTLE;

  table
};

static KING_SQUARE_FROM_START_POSITION: ByColor<Square> = ByColor::new(4, 60);
static EN_PASSANT_OFFSET: ByColor<i8> = ByColor::new(-8, 8);

/*
    Directions and shifts
    +-----+-----+-----+
    | << 7| << 8| << 9|
    +-----+-----+-----+
    | >> 1|  0  | << 1|
    +-----+-----+-----+
    | >> 9| >> 8| >> 7|
    +-----+-----+-----+


     Bit Index (square):
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
#[derive(Clone)]
pub struct Position {
  side_occupancies: ByColor<BitBoard>,
  pieces_occupancies: ByPieceType<BitBoard>,

  // 0 0 0 0 0(q) 0(k) 0(Q) 0(K)
  castling_rights: u8,
  en_passant_file: u8,

  side: PieceColor,
  number_of_moves: u8,
  half_move_clock: u8,

  zobrist_hash: BoardHash,
}

impl Position {
  // https://www.freechess.org/Help/HelpFiles/fen.html
  pub fn from_fen(fen: &str) -> Position {
    let mut board_index: u64 = 56;

    let mut white_board: BitBoard = 0;
    let mut black_board: BitBoard = 0;
    let mut pawns_board: BitBoard = 0;
    let mut knights_board: BitBoard = 0;
    let mut bishops_board: BitBoard = 0;
    let mut rooks_board: BitBoard = 0;
    let mut queens_board: BitBoard = 0;
    let mut kings_board: BitBoard = 0;

    let mut parts = fen.split_whitespace();
    let board_part = parts.next().expect("Missing board part");
    let turn_part = parts.next().expect("Missing side part");
    let castling_part = parts.next().unwrap_or("-");
    let en_passant_part = parts.next().unwrap_or("-");
    let half_move_part = parts.next().unwrap_or("0");
    let number_of_moves_move_part = parts.next().unwrap_or("1");

    for ch in board_part.chars() {
      match ch {
        '/' => {
          board_index = board_index - 16;
          continue;
        }

        '1'..='8' => {
          let skip = ch.to_digit(10).unwrap() - 1;
          board_index += skip as u64;
        }
        'P' | 'p' => {
          pawns_board |= 1u64 << board_index;
          if ch == 'P' {
            white_board |= 1u64 << board_index;
          } else {
            black_board |= 1u64 << board_index;
          }
        }
        'N' | 'n' => {
          knights_board |= 1u64 << board_index;
          if ch == 'N' {
            white_board |= 1u64 << board_index;
          } else {
            black_board |= 1u64 << board_index;
          }
        }
        'B' | 'b' => {
          bishops_board |= 1u64 << board_index;
          if ch == 'B' {
            white_board |= 1u64 << board_index;
          } else {
            black_board |= 1u64 << board_index;
          }
        }
        'R' | 'r' => {
          rooks_board |= 1u64 << board_index;
          if ch == 'R' {
            white_board |= 1u64 << board_index;
          } else {
            black_board |= 1u64 << board_index;
          }
        }
        'Q' | 'q' => {
          queens_board |= 1u64 << board_index;
          if ch == 'Q' {
            white_board |= 1u64 << board_index;
          } else {
            black_board |= 1u64 << board_index;
          }
        }
        'K' | 'k' => {
          kings_board |= 1u64 << board_index;
          if ch == 'K' {
            white_board |= 1u64 << board_index;
          } else {
            black_board |= 1u64 << board_index;
          }
        }

        ' ' => break,
        _ => panic!("Invalid character in FEN: {}", ch),
      }
      board_index = board_index + 1;
    }

    let side = match turn_part {
      "w" => PieceColor::White,
      "b" => PieceColor::Black,
      _ => panic!("Invalid side character in FEN: {}", turn_part),
    };

    let mut castling_rights: u8 = 0;
    if castling_part != "-" {
      for ch in castling_part.chars() {
        match ch {
          'K' => castling_rights |= WHITE_CAN_SHORT_CASTLE,
          'Q' => castling_rights |= WHITE_CAN_LONG_CASTLE,
          'k' => castling_rights |= BLACK_CAN_SHORT_CASTLE,
          'q' => castling_rights |= BLACK_CAN_LONG_CASTLE,
          _ => {}
        }
      }
    }

    let mut en_passant_file: u8 = 8;
    if en_passant_part != "-" {
      for ch in en_passant_part.chars() {
        match ch {
          'a'..='h' => en_passant_file = ch as u8 - 'a' as u8,
          _ => {}
        }
      }
    }

    let mut zobrish_hash: u64 = 0;
    zobrish_hash ^= ZobristHash::get_castling_key(castling_rights);
    zobrish_hash ^= u64::from(side == PieceColor::White) * ZobristHash::get_side_key();

    let boards: ByPieceType<BitBoard> = ByPieceType::new(pawns_board, knights_board, bishops_board, rooks_board, queens_board, kings_board);
    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      let mut board = boards[piece_type] & white_board;
      while board != 0 {
        let square = get_lsb!(board);
        zobrish_hash ^= ZobristHash::get_piece_key(Piece { color: PieceColor::White, piece_type }, square);
        pop_lsb!(board);
      }

      board = boards[piece_type] & black_board;
      while board != 0 {
        let square = get_lsb!(board);
        zobrish_hash ^= ZobristHash::get_piece_key(Piece { color: PieceColor::Black, piece_type }, square);
        pop_lsb!(board);
      }
    }

    if en_passant_file < 8 {
      // En passant key is only applied if the opponent can take the en passant at his turn
      match side {
        PieceColor::White => {
          if pawns_board & white_board & PAWNS_OCCUPANCY_OBLIGATION_FOR_EN_PASSANT[PieceColor::White][en_passant_file as usize] != 0 {
            zobrish_hash ^= ZobristHash::get_en_passant_file_key(en_passant_file);
          } else {
            en_passant_file = 0;
          };
        }
        PieceColor::Black => {
          if pawns_board & black_board & PAWNS_OCCUPANCY_OBLIGATION_FOR_EN_PASSANT[PieceColor::Black][en_passant_file as usize] != 0 {
            zobrish_hash ^= ZobristHash::get_en_passant_file_key(en_passant_file);
          } else {
            en_passant_file = 0;
          };
        }
        PieceColor::None => {}
      }
    }

    let side_occupancies = ByColor::new(white_board, black_board);
    let pieces_occupancies = ByPieceType::new(pawns_board, knights_board, bishops_board, rooks_board, queens_board, kings_board);

    Position {
      side_occupancies,
      pieces_occupancies,
      castling_rights,
      en_passant_file,
      side,
      number_of_moves: number_of_moves_move_part.parse().unwrap_or(1),
      half_move_clock: half_move_part.parse().unwrap_or(0),
      zobrist_hash: zobrish_hash,
    }
  }

  #[inline(always)]
  pub fn make_move(&mut self, mov: Move) {
    let our_side = self.side;
    let enemy_side = our_side.opposite();

    let source: Square = mov.source();
    let destination: Square = mov.destination();
    let move_type = mov.move_type();

    let source_mask: BitBoard = 1u64 << source;
    let destination_mask: BitBoard = 1u64 << destination;
    let source_piece = self.get_piece_on_square(source);
    let destination_piece = self.get_piece_on_square(destination);

    // Putting 0 at the index of the destination
    if destination_piece.piece_type != PieceType::None {
      self.pieces_occupancies[destination_piece.piece_type] &= !destination_mask;
      self.zobrist_hash ^= ZobristHash::get_piece_key(destination_piece, destination);
      // If the destination is not empty, the half move clock will be reset
      self.half_move_clock = 0;
    } else {
      // Half move clock update
      // Pawn's moves are irreversible,
      self.half_move_clock = if source_piece.piece_type == PieceType::Pawn { 0 } else { self.half_move_clock + 1 }
    }

    // Update castling rights
    self.zobrist_hash ^= ZobristHash::get_castling_key(self.castling_rights);
    self.castling_rights &= !CASTLING_AVAILABILITY_TABLE[destination as usize];
    self.castling_rights &= !CASTLING_AVAILABILITY_TABLE[source as usize];
    self.zobrist_hash ^= ZobristHash::get_castling_key(self.castling_rights);

    // Putting 0 at the index of the source
    // And moving the piece at the destination by putting 1 at the destination for the corresponding piece
    self.pieces_occupancies[source_piece.piece_type] ^= source_mask | destination_mask;
    self.zobrist_hash ^= ZobristHash::get_piece_key(source_piece, source);
    self.zobrist_hash ^= ZobristHash::get_piece_key(source_piece, destination);

    // Updating the boards (for each color)
    self.side_occupancies[our_side] ^= source_mask | destination_mask;
    self.side_occupancies[enemy_side] &= !destination_mask;

    // Applying castling and promotions rules
    match move_type {
      MoveType::Normal => {}
      MoveType::ShortCastle => {
        static INITIALS_ROOKS_SQUARES: ByColor<Square> = ByColor::new(7, 63);
        let finals_rook_square = INITIALS_ROOKS_SQUARES[our_side] - 2;

        self.pieces_occupancies[PieceType::Rook] ^= SHORT_CASTLE_ROOK_MASK[our_side];
        self.side_occupancies[our_side] ^= SHORT_CASTLE_ROOK_MASK[our_side];

        self.zobrist_hash ^= ZobristHash::get_piece_key(Piece { color: our_side, piece_type: PieceType::Rook }, INITIALS_ROOKS_SQUARES[our_side]);
        self.zobrist_hash ^= ZobristHash::get_piece_key(Piece { color: our_side, piece_type: PieceType::Rook }, finals_rook_square);
      }
      MoveType::LongCastle => {
        static INITIALS_ROOKS_SQUARES: ByColor<Square> = ByColor::new(0, 56);
        let finals_rook_square = INITIALS_ROOKS_SQUARES[our_side] + 3;

        self.pieces_occupancies[PieceType::Rook] ^= LONG_CASTLE_ROOK_MASK[our_side];
        self.side_occupancies[our_side] ^= LONG_CASTLE_ROOK_MASK[our_side];

        self.zobrist_hash ^= ZobristHash::get_piece_key(Piece { color: our_side, piece_type: PieceType::Rook }, INITIALS_ROOKS_SQUARES[our_side]);
        self.zobrist_hash ^= ZobristHash::get_piece_key(Piece { color: our_side, piece_type: PieceType::Rook }, finals_rook_square);
      }
      MoveType::EnPassant => {
        let enemy_pawn_square = (destination as i8 + EN_PASSANT_OFFSET[our_side]) as Square;

        self.pieces_occupancies[PieceType::Pawn] &= !(1u64 << enemy_pawn_square);
        self.side_occupancies[enemy_side] &= !(1u64 << enemy_pawn_square);
        self.zobrist_hash ^= ZobristHash::get_key_after_en_passant_has_been_taken_by(our_side, enemy_pawn_square);
      }
      _ => {
        // MoveType::PawnToKnight | MoveType::PawnToBishop | MoveType::PawnToRook | MoveType::PawnToQueen
        static PROMOTION_INDEX_TO_PIECE_TYPE: [PieceType; 4] = [PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen];

        // IMPORTANT: This index calculation is heavily aligned with the value assigned to each type of move in the struct MoveType
        let promotion_index = move_type as usize - 3;
        let promotion_piece_type: PieceType = PROMOTION_INDEX_TO_PIECE_TYPE[promotion_index];

        self.pieces_occupancies[PieceType::Pawn] &= !destination_mask;
        self.pieces_occupancies[promotion_piece_type] |= destination_mask;

        self.zobrist_hash ^= ZobristHash::get_piece_key(Piece { color: our_side, piece_type: PieceType::Pawn }, destination);
        self.zobrist_hash ^= ZobristHash::get_piece_key(Piece { color: our_side, piece_type: promotion_piece_type }, destination);
      }
    }

    // Reset the en passant
    self.zobrist_hash ^= ZobristHash::get_en_passant_file_key(self.en_passant_file);
    self.en_passant_file = 8;
    if source_piece.piece_type == PieceType::Pawn && source.abs_diff(destination) == 16 {
      // En passant key is only applied if the opponent can take the en passant at his turn
      self.en_passant_file = (destination as i8 + EN_PASSANT_OFFSET[our_side]) as Square & 7;

      if self.pieces_occupancies[PieceType::Pawn] & self.side_occupancies[enemy_side] & PAWNS_OCCUPANCY_OBLIGATION_FOR_EN_PASSANT[enemy_side][self.en_passant_file as usize] != 0 {
        self.zobrist_hash ^= ZobristHash::get_en_passant_file_key(self.en_passant_file);
      } else {
        self.en_passant_file = 8;
      }
    }

    self.zobrist_hash ^= ZobristHash::get_side_key();
    self.number_of_moves += u8::from(self.side == PieceColor::Black);
    self.side = enemy_side;
  }

  #[inline(always)]
  pub fn check_king_move_safety(&mut self, source: Square, destination: Square, side: PieceColor) -> bool {
    let source_mask = 1u64 << source;
    let destination_mask = 1u64 << destination;
    let move_mask = source_mask | destination_mask;

    self.side_occupancies[side] ^= move_mask;
    self.pieces_occupancies[PieceType::King] ^= move_mask;

    let result = self.get_smallest_attacker_infos(destination, side.opposite()) == (PieceType::None, 64);

    self.side_occupancies[side] ^= move_mask;
    self.pieces_occupancies[PieceType::King] ^= move_mask;

    result
  }

  #[inline(always)]
  pub fn check_en_passant_move_for_king_safety(&mut self, king_square: Square, source: Square, destination: Square, side: PieceColor) -> bool {
    let source_mask = 1u64 << source;
    let destination_mask = 1u64 << destination;
    let move_mask = source_mask | destination_mask;

    let en_passant_mask = !(1u64 << (destination as i8 + EN_PASSANT_OFFSET[side]));

    let our_occupancies = self.side_occupancies[side];
    let enemy_occupancies = self.side_occupancies[side.opposite()];
    let pawns_occupancies = self.pieces_occupancies[PieceType::Pawn];

    self.side_occupancies[side] ^= move_mask;
    self.pieces_occupancies[PieceType::Pawn] ^= move_mask;
    self.pieces_occupancies[PieceType::Pawn] &= en_passant_mask;
    self.side_occupancies[side.opposite()] &= en_passant_mask;

    let result = self.get_smallest_attacker_infos(king_square, side.opposite()) == (PieceType::None, 64);

    self.side_occupancies[side] = our_occupancies;
    self.side_occupancies[side.opposite()] = enemy_occupancies;
    self.pieces_occupancies[PieceType::Pawn] = pawns_occupancies;

    result
  }

  #[inline(always)]
  pub fn make_null_move(&mut self) -> u8 {
    let previous_en_passant_file = self.en_passant_file;

    self.zobrist_hash ^= ZobristHash::get_en_passant_file_key(previous_en_passant_file);
    self.zobrist_hash ^= ZobristHash::get_side_key();
    self.en_passant_file = 8;
    self.side = self.side.opposite();

    previous_en_passant_file
  }

  #[inline(always)]
  pub fn unmake_null_move(&mut self, ancient_en_passant_file: u8) {
    self.zobrist_hash ^= ZobristHash::get_en_passant_file_key(ancient_en_passant_file);
    self.zobrist_hash ^= ZobristHash::get_side_key();
    self.en_passant_file = ancient_en_passant_file;
    self.side = self.side.opposite();
  }

  #[inline(always)]
  pub fn make_see_capture(&mut self, attacker: Piece, target: Piece, source: Square, destination: Square) {
    let source_mask = 1u64 << source;
    let destination_mask = 1u64 << destination;

    self.pieces_occupancies[target.piece_type] &= !destination_mask;
    self.pieces_occupancies[attacker.piece_type] ^= source_mask | destination_mask;

    self.side_occupancies[attacker.color] ^= source_mask | destination_mask;
    self.side_occupancies[target.color] &= !destination_mask;
  }

  #[inline(always)]
  pub fn get_smallest_attacker_infos(&self, target_square: Square, enemy_side: PieceColor) -> (PieceType, Square) {
    let full_board = self.get_full_board();

    let enemy_board = self.side_occupancies[enemy_side];
    let mut superior_mask;
    let mut result;

    superior_mask = get_pawns_attacks(enemy_side.opposite(), target_square);
    result = superior_mask & self.pieces_occupancies[PieceType::Pawn] & enemy_board;
    if result != 0 {
      return (PieceType::Pawn, get_lsb!(result));
    }

    superior_mask = get_knight_attacks(target_square);
    result = superior_mask & self.pieces_occupancies[PieceType::Knight] & enemy_board;
    if result != 0 {
      return (PieceType::Knight, get_lsb!(result));
    }

    let bishop_mask = get_bishop_attacks(full_board, target_square);
    result = bishop_mask & self.pieces_occupancies[PieceType::Bishop] & enemy_board;
    if result != 0 {
      return (PieceType::Bishop, get_lsb!(result));
    }

    let rook_mask = get_rook_attacks(full_board, target_square);
    result = rook_mask & self.pieces_occupancies[PieceType::Rook] & enemy_board;
    if result != 0 {
      return (PieceType::Rook, get_lsb!(result));
    }

    result = (rook_mask | bishop_mask) & self.pieces_occupancies[PieceType::Queen] & enemy_board;
    if result != 0 {
      return (PieceType::Queen, get_lsb!(result));
    }

    superior_mask = get_king_attacks(target_square);
    result = superior_mask & self.pieces_occupancies[PieceType::King] & enemy_board;
    if result != 0 {
      return (PieceType::King, get_lsb!(result));
    }

    (PieceType::None, 64)
  }

  #[inline(always)]
  pub fn get_checkers_for(&self, side: PieceColor) -> ([(PieceType, Square); 16], usize) {
    // A king can be attacked at most by 16 pieces
    // FEN: 8/8/2BNRNB1/2N3N1/2R1k1R1/2N3N1/2BNRNB1/K7 b - - 0 1
    let mut checkers: [(PieceType, Square); 16] = [(PieceType::None, 64); 16];
    let mut checkers_count: usize = 0;

    let full_board = self.get_full_board();
    let enemy_side = side.opposite();
    let king_square = self.get_king_square(side);

    let enemy_board = self.side_occupancies[enemy_side];
    let mut superior_mask;
    let mut result;

    superior_mask = get_pawns_attacks(side, king_square);
    result = superior_mask & self.pieces_occupancies[PieceType::Pawn] & enemy_board;
    while result != 0 {
      checkers[checkers_count] = (PieceType::Pawn, get_lsb!(result));
      checkers_count += 1;
      pop_lsb!(result);
    }

    superior_mask = get_knight_attacks(king_square);
    result = superior_mask & self.pieces_occupancies[PieceType::Knight] & enemy_board;
    while result != 0 {
      checkers[checkers_count] = (PieceType::Knight, get_lsb!(result));
      checkers_count += 1;
      pop_lsb!(result);
    }

    // For the sliding piece, the queen is embedded in the mask of either the bishop or the rook
    let bishop_mask = get_bishop_attacks(full_board, king_square);
    result = bishop_mask & (self.pieces_occupancies[PieceType::Bishop] | self.pieces_occupancies[PieceType::Queen]) & enemy_board;
    while result != 0 {
      checkers[checkers_count] = (PieceType::Bishop, get_lsb!(result));
      checkers_count += 1;
      pop_lsb!(result);
    }

    let rook_mask = get_rook_attacks(full_board, king_square);
    result = rook_mask & (self.pieces_occupancies[PieceType::Rook] | self.pieces_occupancies[PieceType::Queen]) & enemy_board;
    while result != 0 {
      checkers[checkers_count] = (PieceType::Rook, get_lsb!(result));
      checkers_count += 1;
      pop_lsb!(result);
    }

    (checkers, checkers_count)
  }

  #[inline(always)]
  pub fn is_square_attack_by(&self, square: Square, attacker_color: PieceColor) -> bool {
    self.get_smallest_attacker_infos(square, attacker_color) != (PieceType::None, 64)
  }

  #[inline(always)]
  pub fn is_check(&self, side: PieceColor) -> bool {
    self.is_square_attack_by(self.get_king_square(side), side.opposite())
  }

  #[inline(always)]
  pub fn get_side(&self) -> PieceColor {
    self.side
  }

  #[inline(always)]
  pub fn get_piece_on_square(&self, square: Square) -> Piece {
    let square_mask = 1u64 << square as usize;
    let color = if self.side_occupancies[PieceColor::White] & square_mask != 0 {
      PieceColor::White
    } else if self.side_occupancies[PieceColor::Black] & square_mask != 0 {
      PieceColor::Black
    } else {
      // If the square has no color, there is no piece on it
      return Piece { color: PieceColor::None, piece_type: PieceType::None };
    };

    let piece_type = if self.pieces_occupancies[PieceType::Pawn] & square_mask != 0 {
      PieceType::Pawn
    } else if self.pieces_occupancies[PieceType::Knight] & square_mask != 0 {
      PieceType::Knight
    } else if self.pieces_occupancies[PieceType::Bishop] & square_mask != 0 {
      PieceType::Bishop
    } else if self.pieces_occupancies[PieceType::Rook] & square_mask != 0 {
      PieceType::Rook
    } else if self.pieces_occupancies[PieceType::Queen] & square_mask != 0 {
      PieceType::Queen
    } else if self.pieces_occupancies[PieceType::King] & square_mask != 0 {
      PieceType::King
    } else {
      PieceType::None
    };

    Piece { color, piece_type }
  }

  #[inline(always)]
  pub fn get_king_square(&self, side: PieceColor) -> Square {
    get_lsb!(self.side_occupancies[side] & self.pieces_occupancies[PieceType::King])
  }

  #[inline(always)]
  pub fn get_full_board(&self) -> BitBoard {
    self.side_occupancies[PieceColor::White] | self.side_occupancies[PieceColor::Black]
  }

  #[inline(always)]
  pub fn get_by_side(&self, side: PieceColor) -> BitBoard {
    self.side_occupancies[side]
  }

  #[inline(always)]
  pub fn get_by_type(&self, piece_type: PieceType) -> BitBoard {
    self.pieces_occupancies[piece_type]
  }

  #[inline(always)]
  pub fn get_by_side_and_type(&self, side: PieceColor, piece_type: PieceType) -> BitBoard {
    self.side_occupancies[side] & self.pieces_occupancies[piece_type]
  }

  #[inline(always)]
  pub fn get_number_of_moves(&self) -> u8 {
    self.number_of_moves
  }

  #[inline(always)]
  pub fn get_half_move_clock(&self) -> u8 {
    self.half_move_clock
  }

  #[inline(always)]
  pub fn get_zobrist_hash(&self) -> BoardHash {
    self.zobrist_hash
  }

  #[inline(always)]
  pub fn can_short_castle(&self, side: PieceColor) -> bool {
    if self.castling_rights & CAN_SHORT_CASTLE[side] == 0 {
      return false;
    }

    static IMPORTANT_SQUARES_MASK_FOR_SHORT_CASTLE: ByColor<BitBoard> = ByColor::new(1u64 << 5 | 1u64 << 6, 1u64 << 61 | 1u64 << 62);

    let board = self.get_full_board();
    let enemy_side = side.opposite();
    let initial_king_square = KING_SQUARE_FROM_START_POSITION[side];

    (board & IMPORTANT_SQUARES_MASK_FOR_SHORT_CASTLE[side]) == 0
      && !self.is_square_attack_by(initial_king_square, enemy_side)
      && !self.is_square_attack_by(initial_king_square + 1, enemy_side)
      && !self.is_square_attack_by(initial_king_square + 2, enemy_side)
  }

  #[inline(always)]
  pub fn can_long_castle(&self, side: PieceColor) -> bool {
    if self.castling_rights & CAN_LONG_CASTLE[side] == 0 {
      return false;
    }

    static IMPORTANT_SQUARES_MASK_FOR_LONG_CASTLE: ByColor<BitBoard> = ByColor::new(1u64 << 3 | 1u64 << 2 | 1u64 << 1, 1u64 << 59 | 1u64 << 58 | 1u64 << 57);

    let board = self.get_full_board();
    let enemy_side = side.opposite();
    let initial_king_square = KING_SQUARE_FROM_START_POSITION[side];

    (board & IMPORTANT_SQUARES_MASK_FOR_LONG_CASTLE[side]) == 0
      && !self.is_square_attack_by(initial_king_square, enemy_side)
      && !self.is_square_attack_by(initial_king_square - 1, enemy_side)
      && !self.is_square_attack_by(initial_king_square - 2, enemy_side)
  }

  #[inline(always)]
  pub fn get_en_passant(&self) -> Square {
    static EN_PASSANT_FILE_TO_SQUARE_FOR_BLACK: [u8; 9] = [16, 17, 18, 19, 20, 21, 22, 23, 64];
    EN_PASSANT_FILE_TO_SQUARE_FOR_BLACK[self.en_passant_file as usize] + 3 * 8 * self.side as u8
  }

  #[inline(always)]
  pub fn has_non_pawn_material(&self) -> bool {
    self.pieces_occupancies[PieceType::Knight] != 0
      || self.pieces_occupancies[PieceType::Bishop] != 0
      || self.pieces_occupancies[PieceType::Rook] != 0
      || self.pieces_occupancies[PieceType::Queen] != 0
  }

  #[inline(always)]
  pub fn evaluate_phase(&self) -> i32 {
    let mut phase = TOTAL_PHASE;

    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      let mut board = self.get_by_type(piece_type);
      while board != 0 {
        phase -= get_phase(piece_type);
        pop_lsb!(board);
      }
    }

    phase.max(0) as i32
  }

  pub fn print_board(&self) {
    println!();
    for rank in (0..=7).rev() {
      print!("{} ", rank + 1);
      for file in 0..=7 {
        let index = (rank * 8 + file as usize) as Square;
        print!("{} ", self.piece_to_unicode(&self.get_piece_on_square(index)));
      }
      println!();
    }
    println!("\n  a b c d e f g h\n");
  }

  fn piece_to_unicode(&self, piece: &Piece) -> char {
    match (piece.color, piece.piece_type) {
      (PieceColor::White, PieceType::Pawn) => '♙',
      (PieceColor::White, PieceType::Knight) => '♘',
      (PieceColor::White, PieceType::Bishop) => '♗',
      (PieceColor::White, PieceType::Rook) => '♖',
      (PieceColor::White, PieceType::Queen) => '♕',
      (PieceColor::White, PieceType::King) => '♔',
      (PieceColor::Black, PieceType::Pawn) => '♟',
      (PieceColor::Black, PieceType::Knight) => '♞',
      (PieceColor::Black, PieceType::Bishop) => '♝',
      (PieceColor::Black, PieceType::Rook) => '♜',
      (PieceColor::Black, PieceType::Queen) => '♛',
      (PieceColor::Black, PieceType::King) => '♚',
      _ => '·',
    }
  }
}
