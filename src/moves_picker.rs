use crate::containers::ByPieceType;
use crate::moves::{MOVE_LIST_MAX_SIZE, Move, MoveList, MoveType};
use crate::moves_generator::{generate_legal_moves, generate_quiescences_moves};
use crate::piece::PieceType;
use crate::position::Position;
use crate::utils::ZENO_INFINITY;

// Move's order
// 1. TT Move
// 2. Captures + Promotions
// 3. Promotions
// 4. Captures
// 5. Killers moves
// 6. Castles moves
// 7. The others Quiets moves
static TT_MOVE_SCORE: i32 = 600_000;
static PROMOTION_MOVE_SCORE: i32 = 250_000;
static CAPTURE_MOVE_SCORE: i32 = 200_000;
static KILLER_MOVE_SCORE: i32 = 150_000;
static CASTLE_MOVE_SCORE: i32 = 100_000;
static EN_PASSANT_MOVE_SCORE: i32 = 6002;

// https://open-chess.org/viewtopic.php?t=3058
static MVV_LVA: ByPieceType<ByPieceType<i32>> = ByPieceType::new(
  /*P*/ ByPieceType::new(6002, 20225, 20250, 20400, 20800, 26900),
  /*N*/ ByPieceType::new(4775, 06004, 20025, 20175, 20575, 26675),
  /*B*/ ByPieceType::new(4750, 04975, 06006, 20150, 20550, 26650),
  /*R*/ ByPieceType::new(4600, 04825, 04850, 06008, 20400, 26500),
  /*Q*/ ByPieceType::new(4200, 04425, 04450, 04600, 06010, 26100),
  /*K*/ ByPieceType::new(3100, 03325, 03350, 03500, 03900, 26000),
);

pub struct MovePicker {
  moves_list: MoveList,
  scores: [i32; MOVE_LIST_MAX_SIZE],
  start_index: usize,
  tt_move: Move,
  killers: (Move, Move),
}

impl MovePicker {
  #[inline(always)]
  pub fn new(position: &mut Position, tt_move: Option<Move>, killers: Option<(Move, Move)>, is_quiescence_search: bool) -> Self {
    let tt_move = tt_move.unwrap_or_default();
    let killers = killers.unwrap_or_default();

    let mut moves = MoveList::new();
    if is_quiescence_search {
      generate_quiescences_moves(position, &mut moves);
    } else {
      generate_legal_moves(position, &mut moves);
    }

    Self { moves_list: moves, scores: [-ZENO_INFINITY; MOVE_LIST_MAX_SIZE], start_index: 0, tt_move, killers }
  }

  #[inline(always)]
  pub fn pick_best_move(&mut self, position: &mut Position) -> Option<Move> {
    if self.start_index >= self.moves_list.count {
      return None;
    }

    let mut best_index = self.start_index;
    let mut best_move = self.moves_list.moves[self.start_index];
    let mut best_score = self.scores[self.start_index];

    if best_score == -ZENO_INFINITY {
      best_score = self.evaluate_move(best_move, position);
    }

    for i in (self.start_index + 1)..self.moves_list.count {
      if self.scores[i] == -ZENO_INFINITY {
        self.scores[i] = self.evaluate_move(self.moves_list.moves[i], position);
      }

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

    Some(best_move)
  }

  #[inline(always)]
  pub fn get_moves_count(&self) -> usize {
    self.moves_list.count
  }

  #[inline(always)]
  fn evaluate_move(&self, mov: Move, position: &Position) -> i32 {
    let mut score = 0;

    let source_piece = position.get_piece_on_square(mov.source());
    let destination_piece = position.get_piece_on_square(mov.destination());

    if mov == self.tt_move {
      score = TT_MOVE_SCORE;
    }

    if destination_piece.piece_type != PieceType::None {
      score += CAPTURE_MOVE_SCORE + MVV_LVA[source_piece.piece_type][destination_piece.piece_type];
    } else {
      if mov == self.killers.0 {
        score += KILLER_MOVE_SCORE + 500;
      } else if mov == self.killers.1 {
        score += KILLER_MOVE_SCORE;
      }
    }

    // Promotion bonus
    score += match mov.move_type() {
      MoveType::Normal => 0,
      MoveType::ShortCastle => CASTLE_MOVE_SCORE,
      MoveType::LongCastle => CASTLE_MOVE_SCORE,
      MoveType::EnPassant => EN_PASSANT_MOVE_SCORE,
      MoveType::PawnToKnight => PROMOTION_MOVE_SCORE + 300,
      MoveType::PawnToBishop => PROMOTION_MOVE_SCORE + 400,
      MoveType::PawnToRook => PROMOTION_MOVE_SCORE + 500,
      MoveType::PawnToQueen => PROMOTION_MOVE_SCORE + 600,
    };
    score
  }
}
