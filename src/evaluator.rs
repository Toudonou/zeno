use crate::bitboard::BitBoard;
use crate::eval_params::EvalParams;
use crate::params::{EG_BISHOP_PAIR, MG_BISHOP_PAIR};
use crate::piece::{PieceColor, PieceType};
use crate::position::Position;
use crate::square::Square;
use crate::{get_lsb, pop_lsb};
use std::ops::{Add, AddAssign, SubAssign};

pub static DRAW_VALUE: i32 = 0;
static LIGHT_SQUARES: BitBoard = 0x55AA55AA55AA55AAu64;

#[rustfmt::skip]
static ARR_CENTER_MANHATTAN_DISTANCE: [i32; 64] = [
  6, 5, 4, 3, 3, 4, 5, 6,
  5, 4, 3, 2, 2, 3, 4, 5,
  4, 3, 2, 1, 1, 2, 3, 4,
  3, 2, 1, 0, 0, 1, 2, 3,
  3, 2, 1, 0, 0, 1, 2, 3,
  4, 3, 2, 1, 1, 2, 3, 4,
  5, 4, 3, 2, 2, 3, 4, 5,
  6, 5, 4, 3, 3, 4, 5, 6
];

pub struct Score {
  pub mg: i32,
  pub eg: i32,
}

pub struct Evaluator {}

impl Evaluator {
  #[inline(always)]
  pub fn evaluate(position: &Position, eval_params: &EvalParams) -> i32 {
    Evaluator::static_evaluation(position, eval_params)
  }

  #[inline(always)]
  pub fn static_evaluation(position: &Position, eval_params: &EvalParams) -> i32 {
    let mut score = Score { mg: 0, eg: 0 };
    let phase = position.evaluate_phase();

    // The game is about 80% of the phase
    if phase >= 200 {
      // A draw by insufficient material can only occur during endgames
      if Evaluator::is_draw_by_insufficient_material(position) {
        return 0;
      }
      score.eg += Evaluator::king_cornering(position.get_king_square(PieceColor::Black));
      score.eg -= Evaluator::king_cornering(position.get_king_square(PieceColor::White));
    }

    score += Evaluator::material_eval(position, PieceColor::White, eval_params);
    score -= Evaluator::material_eval(position, PieceColor::Black, eval_params);

    if Evaluator::has_bishop_pair(position, PieceColor::White) {
      score.mg += MG_BISHOP_PAIR;
      score.eg += EG_BISHOP_PAIR;
    }

    if Evaluator::has_bishop_pair(position, PieceColor::Black) {
      score.mg -= MG_BISHOP_PAIR;
      score.eg -= EG_BISHOP_PAIR;
    }

    (((score.mg * (256 - phase)) + (score.eg * phase)) >> 8) * position.get_side().to_i32()
  }

  #[inline(always)]
  fn material_eval(position: &Position, side: PieceColor, eval_params: &EvalParams) -> Score {
    let mut score = Score { mg: 0, eg: 0 };

    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      let mut board = position.get_by_side_and_type(side, piece_type);
      while board != 0 {
        let square = get_lsb!(board);

        score.mg += eval_params.get_mg_piece_value(piece_type) + eval_params.get_mg_psqt_value(piece_type, side, square);
        score.eg += eval_params.get_eg_piece_value(piece_type) + eval_params.get_eg_psqt_value(piece_type, side, square);

        pop_lsb!(board);
      }
    }

    score
  }

  #[inline(always)]
  pub fn has_bishop_pair(position: &Position, side: PieceColor) -> bool {
    let bishops = position.get_by_side_and_type(side, PieceType::Bishop);
    (bishops & LIGHT_SQUARES != 0) && (bishops & !LIGHT_SQUARES != 0)
  }

  #[inline(always)]
  pub fn is_draw_by_insufficient_material(position: &Position) -> bool {
    if position.get_by_type(PieceType::Pawn).count_ones() != 0 || position.get_by_type(PieceType::Rook).count_ones() != 0 || position.get_by_type(PieceType::Queen).count_ones() != 0 {
      return false;
    }

    let knights_count = position.get_by_type(PieceType::Knight).count_ones();
    let white_bishops_count = position.get_by_side_and_type(PieceColor::White, PieceType::Bishop).count_ones();
    let black_bishops_count = position.get_by_side_and_type(PieceColor::Black, PieceType::Bishop).count_ones();
    let total_minors = knights_count + white_bishops_count + black_bishops_count;

    // King vs King
    if total_minors == 0 {
      return true;
    }

    // King + Minor vs King (Only 1 minor piece total on the board)
    if total_minors == 1 {
      return true;
    }

    // King + Bishop vs King + Bishop on the same square color
    if total_minors == 2 && white_bishops_count == 1 && black_bishops_count == 1 {
      let white_bishop_square = position.get_by_side_and_type(PieceColor::White, PieceType::Bishop);
      let black_bishop_square = position.get_by_side_and_type(PieceColor::Black, PieceType::Bishop);

      // If both bishops are on light squares or both are on dark squares, it's a draw.
      // They can either be one light squares (therefore not on the dark ones)
      // or they can be on the dark ones (therefore not on the light ones)
      let white_on_light = (white_bishop_square & LIGHT_SQUARES) != 0;
      let black_on_light = (black_bishop_square & LIGHT_SQUARES) != 0;

      if white_on_light == black_on_light {
        return true;
      }
    }

    false
  }

  #[inline(always)]
  fn king_cornering(opponent_square: Square) -> i32 {
    6 * ARR_CENTER_MANHATTAN_DISTANCE[opponent_square as usize]
  }
}

impl Add for Score {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Score { mg: self.mg + rhs.mg, eg: self.eg + rhs.eg }
  }
}

impl AddAssign for Score {
  fn add_assign(&mut self, rhs: Self) {
    self.mg += rhs.mg;
    self.eg += rhs.eg;
  }
}

impl SubAssign for Score {
  fn sub_assign(&mut self, rhs: Self) {
    self.mg -= rhs.mg;
    self.eg -= rhs.eg;
  }
}
