use crate::bitboard::BitBoard;
use crate::containers::ByColor;
use crate::lookup_tables::{get_bishop_attacks, get_bishop_square_to_square_ray, get_king_attacks, get_knight_attacks, get_rook_attacks, get_rook_square_to_square_ray};
use crate::moves::{Move, MoveList, MoveType};
use crate::piece::{PieceColor, PieceType};
use crate::position::Position;
use crate::square::Square;
use crate::utils::{BITBOARD_FILL_WITH_ONE, NOT_FILE_A, NOT_FILE_H, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7};
use crate::{get_lsb, pop_lsb, shift};

static PRE_PROMOTION_RANK: ByColor<BitBoard> = ByColor::new(RANK_7, RANK_2);
static RANK2_BY_PERSPECTIVE: ByColor<BitBoard> = ByColor::new(RANK_2, RANK_7);
static RANK3_BY_PERSPECTIVE: ByColor<BitBoard> = ByColor::new(RANK_3, RANK_6);
static RANK4_BY_PERSPECTIVE: ByColor<BitBoard> = ByColor::new(RANK_4, RANK_5);
static EN_PASSANT_RANK_OBLIGATION: ByColor<BitBoard> = ByColor::new(RANK_5, RANK_4);
static LEFT_OFFSET: ByColor<i8> = ByColor::new(7, -9);
static RIGHT_OFFSET: ByColor<i8> = ByColor::new(9, -7);
static FORWARD_OFFSET: ByColor<i8> = ByColor::new(8, -8);

#[inline(always)]
pub fn generate_legal_moves(position: &mut Position, move_list: &mut MoveList) {
  let side = position.get_side();
  let king_square = position.get_king_square(side);

  let our_board = position.get_by_side(side);
  let non_own_pieces: BitBoard = !our_board;
  let full_board = position.get_full_board();

  let obligation_board: BitBoard = generate_obligation_board(position, side, king_square);
  let pin_masks = generate_pin_masks(position, side, obligation_board);

  // Pawns
  let our_pawns = our_board & position.get_pawns_board();
  let enemy_board = full_board & non_own_pieces;
  generate_quiets_moves_for_pawns(side, our_pawns, !full_board, &pin_masks, move_list);
  generate_captures_moves_for_pawns(side, our_pawns, enemy_board, &pin_masks, move_list);
  generate_promotions_moves_for_pawns(side, our_pawns, enemy_board, !full_board, &pin_masks, move_list);
  generate_en_passant_moves(position, king_square, move_list);

  // Knights
  let knights_board = our_board & position.get_knights_board();
  generate_non_pawns_moves(PieceType::Knight, knights_board, non_own_pieces, full_board, &pin_masks, move_list);

  // Bishops and Queen
  let queens_board = position.get_queens_board();
  let bishops_board = our_board & (position.get_bishops_board() | queens_board);
  generate_non_pawns_moves(PieceType::Bishop, bishops_board, non_own_pieces, full_board, &pin_masks, move_list);

  // Rooks and Queen
  let rooks_board = our_board & (position.get_rooks_board() | queens_board);
  generate_non_pawns_moves(PieceType::Rook, rooks_board, non_own_pieces, full_board, &pin_masks, move_list);

  // King
  generate_king_moves(position, king_square, non_own_pieces, side, false, move_list);
}

#[inline(always)]
pub fn generate_quiescences_moves(position: &mut Position, move_list: &mut MoveList) {
  let side = position.get_side();
  let king_square = position.get_king_square(side);

  let our_board = position.get_by_side(side);
  let full_board = position.get_full_board();
  let enemy_board = full_board & !our_board;

  let obligation_board: BitBoard = generate_obligation_board(position, side, king_square);
  let pin_masks = generate_pin_masks(position, side, obligation_board);

  // Pawns
  let our_pawns = our_board & position.get_pawns_board();
  generate_captures_moves_for_pawns(side, our_pawns, enemy_board, &pin_masks, move_list);
  generate_promotions_moves_for_pawns(side, our_pawns, enemy_board, !full_board, &pin_masks, move_list);
  generate_en_passant_moves(position, king_square, move_list);

  // Knights
  let knights_board = our_board & position.get_knights_board();
  generate_non_pawns_moves(PieceType::Knight, knights_board, enemy_board, full_board, &pin_masks, move_list);

  // Bishops and Queen
  let queens_board = position.get_queens_board();
  let bishops_board = our_board & (position.get_bishops_board() | queens_board);
  generate_non_pawns_moves(PieceType::Bishop, bishops_board, enemy_board, full_board, &pin_masks, move_list);

  // Rooks and Queen
  let rooks_board = our_board & (position.get_rooks_board() | queens_board);
  generate_non_pawns_moves(PieceType::Rook, rooks_board, enemy_board, full_board, &pin_masks, move_list);

  // King
  generate_king_moves(position, king_square, enemy_board, side, true, move_list);
}

#[inline(always)]
fn generate_quiets_moves_for_pawns(side: PieceColor, our_pawns: BitBoard, empty_board: BitBoard, pin_masks: &[BitBoard; 64], move_list: &mut MoveList) {
  let non_promotion_pawns = our_pawns & !PRE_PROMOTION_RANK[side];

  let mut simple_push = shift!(non_promotion_pawns, FORWARD_OFFSET[side]) & empty_board;
  while simple_push != 0 {
    let destination = get_lsb!(simple_push);
    let source = (destination as i8 - FORWARD_OFFSET[side]) as Square;
    if (1u64 << destination) & pin_masks[source as usize] != 0 {
      move_list.push(Move::new(source, destination, MoveType::Normal));
    }
    pop_lsb!(simple_push);
  }

  let empty_rank_3_rank_4_by_perspective = shift!(!empty_board & RANK3_BY_PERSPECTIVE[side], FORWARD_OFFSET[side]) | (!empty_board & RANK4_BY_PERSPECTIVE[side]);
  let mut double_push = shift!(our_pawns & RANK2_BY_PERSPECTIVE[side], 2 * FORWARD_OFFSET[side]) & !empty_rank_3_rank_4_by_perspective;
  while double_push != 0 {
    let destination = get_lsb!(double_push);
    let source = (destination as i8 - 2 * FORWARD_OFFSET[side]) as Square;
    if (1u64 << destination) & pin_masks[source as usize] != 0 {
      move_list.push(Move::new(source, destination, MoveType::Normal));
    }
    pop_lsb!(double_push);
  }
}

#[inline(always)]
fn generate_captures_moves_for_pawns(side: PieceColor, our_pawns: BitBoard, enemy_board: BitBoard, pin_masks: &[BitBoard; 64], move_list: &mut MoveList) {
  let non_promotion_pawns = our_pawns & !PRE_PROMOTION_RANK[side];

  let mut left_push = shift!(non_promotion_pawns, LEFT_OFFSET[side]) & NOT_FILE_H & enemy_board;
  while left_push != 0 {
    let destination = get_lsb!(left_push);
    let source = (destination as i8 - LEFT_OFFSET[side]) as Square;
    if (1u64 << destination) & pin_masks[source as usize] != 0 {
      move_list.push(Move::new(source, destination, MoveType::Normal));
    }
    pop_lsb!(left_push);
  }

  let mut right_push = shift!(non_promotion_pawns, RIGHT_OFFSET[side]) & NOT_FILE_A & enemy_board;
  while right_push != 0 {
    let destination = get_lsb!(right_push);
    let source = (destination as i8 - RIGHT_OFFSET[side]) as Square;
    if (1u64 << destination) & pin_masks[source as usize] != 0 {
      move_list.push(Move::new(source, destination, MoveType::Normal));
    }
    pop_lsb!(right_push);
  }
}

#[inline(always)]
fn generate_promotions_moves_for_pawns(side: PieceColor, our_pawns: BitBoard, enemy_board: BitBoard, empty_board: BitBoard, pin_masks: &[BitBoard; 64], move_list: &mut MoveList) {
  let promotion_pawns = our_pawns & PRE_PROMOTION_RANK[side];

  let mut simple_push = shift!(promotion_pawns, FORWARD_OFFSET[side]) & empty_board;
  while simple_push != 0 {
    let destination = get_lsb!(simple_push);
    let source = (destination as i8 - FORWARD_OFFSET[side]) as Square;
    if (1u64 << destination) & pin_masks[source as usize] != 0 {
      add_promotions_moves(source, destination, move_list);
    }
    pop_lsb!(simple_push);
  }

  let mut left_push = shift!(promotion_pawns, LEFT_OFFSET[side]) & NOT_FILE_H & enemy_board;
  while left_push != 0 {
    let destination = get_lsb!(left_push);
    let source = (destination as i8 - LEFT_OFFSET[side]) as Square;
    if (1u64 << destination) & pin_masks[source as usize] != 0 {
      add_promotions_moves(source, destination, move_list);
    }
    pop_lsb!(left_push);
  }

  let mut right_push = shift!(promotion_pawns, RIGHT_OFFSET[side]) & NOT_FILE_A & enemy_board;
  while right_push != 0 {
    let destination = get_lsb!(right_push);
    let source = (destination as i8 - RIGHT_OFFSET[side]) as Square;
    if (1u64 << destination) & pin_masks[source as usize] != 0 {
      add_promotions_moves(source, destination, move_list);
    }
    pop_lsb!(right_push);
  }
}

#[inline(always)]
fn generate_en_passant_moves(position: &mut Position, king_square: Square, move_list: &mut MoveList) {
  let en_passant = position.get_en_passant();
  if en_passant < 64 {
    let side = position.get_side();
    let en_passant_mask = 1u64 << en_passant;
    let white_pawns = position.get_pawns_board() & position.get_by_side(side);
    let en_passant_obligation_mask = EN_PASSANT_RANK_OBLIGATION[side];

    let en_passant_left_push = shift!((white_pawns & en_passant_obligation_mask), LEFT_OFFSET[side]) & NOT_FILE_H & en_passant_mask;
    if en_passant_left_push != 0 {
      let source = (en_passant as i8 - LEFT_OFFSET[side]) as Square;
      if position.check_en_passant_move_for_king_safety(king_square, source, en_passant, side) {
        move_list.push(Move::new(source, en_passant, MoveType::EnPassant));
      }
    }

    let en_passant_right_push = shift!((white_pawns & en_passant_obligation_mask), RIGHT_OFFSET[side]) & NOT_FILE_A & en_passant_mask;
    if en_passant_right_push != 0 {
      let source = (en_passant as i8 - RIGHT_OFFSET[side]) as Square;
      if position.check_en_passant_move_for_king_safety(king_square, source, en_passant, side) {
        move_list.push(Move::new(source, en_passant, MoveType::EnPassant));
      }
    }
  }
}

#[inline(always)]
fn generate_non_pawns_moves(piece_type: PieceType, mut piece_board: BitBoard, target_board: BitBoard, full_board: BitBoard, pin_masks: &[BitBoard; 64], move_list: &mut MoveList) {
  while piece_board != 0 {
    let square = get_lsb!(piece_board);
    let attacks = target_board
        & pin_masks[square as usize]
        & match piece_type {
      PieceType::Knight => get_knight_attacks(square),
      PieceType::Bishop => get_bishop_attacks(full_board, square),
      PieceType::Rook => get_rook_attacks(full_board, square),
      _ => 0,
    };
    extract_move_from_mask(attacks, square, MoveType::Normal, move_list);
    pop_lsb!(piece_board);
  }
}

#[inline(always)]
fn generate_king_moves(position: &mut Position, king_square: Square, target_board: BitBoard, side: PieceColor, capture_only: bool, move_list: &mut MoveList) {
  if !capture_only && position.can_short_castle(side) {
    move_list.push(Move::new(king_square, king_square + 2, MoveType::ShortCastle));
  }
  if !capture_only && position.can_long_castle(side) {
    move_list.push(Move::new(king_square, king_square - 2, MoveType::LongCastle));
  }
  let mut attacks = target_board & get_king_attacks(king_square);
  while attacks != 0 {
    let destination = get_lsb!(attacks);
    if position.check_king_move_safety(king_square, destination, side) {
      move_list.push(Move::new(king_square, destination, MoveType::Normal));
    }
    pop_lsb!(attacks);
  }
}

#[inline(always)]
fn generate_pin_masks(position: &Position, side: PieceColor, obligation_board: BitBoard) -> [BitBoard; 64] {
  let mut pin_masks: [BitBoard; 64] = [obligation_board; 64];

  let enemy_side = side.opposite();
  let king_square = position.get_king_square(side);

  let full_board = position.get_full_board();

  let our_board = position.get_by_side(side);
  let enemy_board = position.get_by_side(enemy_side);
  let board_without_friendlies = enemy_board;
  let queens_board = position.get_by_type(PieceType::Queen);

  let superior_king_bishop_mask = get_bishop_attacks(full_board, king_square) & our_board;
  let bishop_mask = get_bishop_attacks(board_without_friendlies, king_square);
  let mut result = bishop_mask & (position.get_by_type(PieceType::Bishop) | queens_board) & enemy_board;
  while result != 0 {
    let bishop_square = get_lsb!(result);
    let real_pinned_piece = superior_king_bishop_mask & get_bishop_attacks(full_board, bishop_square);

    // It should be only piece
    if real_pinned_piece != 0 {
      pin_masks[real_pinned_piece.trailing_zeros() as usize] &= get_bishop_square_to_square_ray(king_square, bishop_square);
    }
    pop_lsb!(result);
  }

  let superior_king_rook_mask = get_rook_attacks(full_board, king_square) & our_board;
  let rook_mask = get_rook_attacks(board_without_friendlies, king_square);
  let mut result = rook_mask & (position.get_by_type(PieceType::Rook) | queens_board) & enemy_board;
  while result != 0 {
    let rook_square = get_lsb!(result);
    let real_pinned_piece = superior_king_rook_mask & get_rook_attacks(full_board, rook_square);

    // It should be only piece
    if real_pinned_piece != 0 {
      pin_masks[real_pinned_piece.trailing_zeros() as usize] &= get_rook_square_to_square_ray(king_square, rook_square);
    }
    pop_lsb!(result);
  }

  pin_masks
}

#[inline(always)]
fn generate_obligation_board(position: &Position, side: PieceColor, king_square: Square) -> BitBoard {
  let checkers = position.get_checkers_for(side);
  if checkers.1 == 0 {
    BITBOARD_FILL_WITH_ONE
  } else if checkers.1 == 1 {
    let checker = &checkers.0[0];
    match checker.0 {
      // In case of knight or pawn as checker, the king should either move or the checker must be taken, therefore all legals moves should end up on the checker's square
      PieceType::Pawn | PieceType::Knight => 1u64 << checker.1,

      // In case of a sliding piece as checker,
      // - the king should also either move
      // - or the checker must be taken
      // - or a friendly piece should be place between our king and the checker
      PieceType::Bishop => get_bishop_square_to_square_ray(king_square, checker.1),
      PieceType::Rook => get_rook_square_to_square_ray(king_square, checker.1),
      _ => 0,
    }
  } else {
    0
  }
}

#[inline(always)]
fn extract_move_from_mask(mut attacks: BitBoard, source: Square, move_type: MoveType, move_list: &mut MoveList) {
  while attacks != 0 {
    let destination = get_lsb!(attacks);
    move_list.push(Move::new(source, destination, move_type));
    pop_lsb!(attacks);
  }
}

#[inline(always)]
fn add_promotions_moves(source: Square, destination: Square, move_list: &mut MoveList) {
  move_list.push(Move::new(source, destination, MoveType::PawnToQueen));
  move_list.push(Move::new(source, destination, MoveType::PawnToRook));
  move_list.push(Move::new(source, destination, MoveType::PawnToBishop));
  move_list.push(Move::new(source, destination, MoveType::PawnToKnight));
}
