use std::cmp;

use crate::moves::{MOVE_LIST_MAX_SIZE, Move, MoveList, MoveType};
use crate::moves_generator::{generate_legal_moves, generate_quiescences_moves};
use crate::piece::{Piece, PieceType};
use crate::position::Position;
use crate::square::Square;
use crate::utils::{SEE_VALUES, ZENO_INFINITY};

// Move's order
// 1. TT move
// 2. Good captures + Promotions
// 3. Promotions
// 4. Good captures
// 5. Killers moves
// 6. Counters moves
// 7. Castles moves
// 8. Bad captures
// 8. The others quiet moves (history bonus)
static TT_MOVE_SCORE: i32 = 20_000_000;
static GOOD_CAPTURE_MOVE_SCORE: i32 = 15_000_000;
static KILLER_MOVE_SCORE: i32 = 7_000_000;
static COUNTER_MOVE_SCORE: i32 = 5_000_000;
static CASTLE_MOVE_SCORE: i32 = 2_000_000;
static BAD_CAPTURE_MOVE_SCORE: i32 = 1_000_000;

pub struct MovePicker {
  moves_list: MoveList,
  scores: [i32; MOVE_LIST_MAX_SIZE],
  start_index: usize,
}

impl MovePicker {
  #[inline(always)]
  pub fn new(
    position: &mut Position,
    tt_move: Option<Move>,
    killers: Option<(Move, Move)>,
    counter: Option<Move>,
    history_heuristic: Option<&[[i32; 64]; 64]>,
    is_quiescence_search: bool,
  ) -> Self {
    let tt_move = tt_move.unwrap_or_default();
    let killers = killers.unwrap_or_default();
    let counter = counter.unwrap_or_default();
    let history_heuristic = history_heuristic.unwrap_or(&[[0; 64]; 64]);

    let mut move_picker = MovePicker {
      moves_list: MoveList::new(),
      scores: [-ZENO_INFINITY; MOVE_LIST_MAX_SIZE],
      start_index: 0,
    };

    if is_quiescence_search {
      generate_quiescences_moves(position, &mut move_picker.moves_list);
    } else {
      generate_legal_moves(position, &mut move_picker.moves_list);
    }

    for i in 0..move_picker.moves_list.count {
      move_picker.scores[i] = MovePicker::evaluate_move(move_picker.moves_list.moves[i], position, tt_move, killers, counter, history_heuristic);
    }

    move_picker
  }

  #[inline(always)]
  pub fn pick_best_move(&mut self) -> Option<(Move, i32)> {
    if self.start_index >= self.moves_list.count {
      return None;
    }

    let mut best_index = self.start_index;
    let mut best_move = self.moves_list.moves[self.start_index];
    let mut best_score = self.scores[self.start_index];

    for i in (self.start_index + 1)..self.moves_list.count {
      if self.scores[i] > best_score {
        best_score = self.scores[i];
        best_index = i;
        best_move = self.moves_list.moves[i]
      }
    }

    // Swap the best move to the front (current_index)
    self.moves_list.moves.swap(self.start_index, best_index);
    self.scores.swap(self.start_index, best_index);

    // Move the starting index to ignore the picked move in the future
    self.start_index += 1;

    Some((best_move, (self.start_index - 1) as i32))
  }

  #[inline(always)]
  pub fn get_moves_count(&self) -> usize {
    self.moves_list.count
  }

  #[inline(always)]
  fn evaluate_move(mov: Move, position: &Position, tt_move: Move, killers: (Move, Move), counter: Move, history_heuristic: &[[i32; 64]; 64]) -> i32 {
    let destination_piece = position.get_piece_on_square(mov.destination());

    if mov == tt_move {
      TT_MOVE_SCORE
    } else if destination_piece.piece_type != PieceType::None {
      let see_value = Self::see_capture(position, mov);
      return if see_value >= 0 {
        GOOD_CAPTURE_MOVE_SCORE + see_value + if mov.is_promotion() { 9000 } else { 0 }
      } else {
        BAD_CAPTURE_MOVE_SCORE + see_value
      };
    } else if mov == killers.0 {
      KILLER_MOVE_SCORE + 50_000
    } else if mov == killers.1 {
      KILLER_MOVE_SCORE
    } else if mov == counter {
      COUNTER_MOVE_SCORE
    } else {
      match mov.move_type() {
        MoveType::ShortCastle | MoveType::LongCastle => CASTLE_MOVE_SCORE,
        MoveType::PawnToQueen => GOOD_CAPTURE_MOVE_SCORE + 6000,
        MoveType::PawnToRook => GOOD_CAPTURE_MOVE_SCORE + 5000,
        MoveType::PawnToBishop => GOOD_CAPTURE_MOVE_SCORE + 4000,
        MoveType::PawnToKnight => GOOD_CAPTURE_MOVE_SCORE + 3000,
        _ => history_heuristic[mov.source() as usize][mov.destination() as usize],
      }
    }
  }

  // https://www.chessprogramming.org/Static_Exchange_Evaluation#Implementation
  #[inline(always)]
  pub fn see_capture(position: &Position, mov: Move) -> i32 {
    let attacker = position.get_piece_on_square(mov.source());
    let victim = position.get_piece_on_square(mov.destination());

    let mut temp_position = position.clone();

    (if victim.piece_type != PieceType::None {
      temp_position.make_see_capture(attacker, victim, mov.source(), mov.destination());
      SEE_VALUES[victim.piece_type]
    } else {
      0
    }) - MovePicker::see(&mut temp_position, mov.destination(), attacker)
  }

  #[inline(always)]
  fn see(position: &mut Position, destination: Square, victim: Piece) -> i32 {
    let mut value: i32 = 0;
    let attacker_type_and_square = position.get_smallest_attacker_infos(destination, victim.color.opposite());
    let attacker = Piece {
      color: victim.color.opposite(),
      piece_type: attacker_type_and_square.0,
    };

    if attacker.piece_type != PieceType::None {
      position.make_see_capture(attacker, victim, attacker_type_and_square.1, destination);
      // Should be good, all captures are not forced
      value = cmp::max(0, SEE_VALUES[victim.piece_type] - Self::see(position, destination, attacker));
    }

    value
  }
}
