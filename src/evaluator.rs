use crate::eval_params::EvalParams;
use crate::piece::{PieceColor, PieceType};
use crate::position::Position;
use crate::square::{Square, SquareOps};
use crate::zobrist_hash::BoardHash;
use crate::{get_lsb, pop_lsb};

pub static DRAW_VALUE: i32 = 0;
static LIGHT_SQUARES: BoardHash = 0x55AA55AA55AA55AAu64;

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

pub struct Evaluator {}

impl Evaluator {
  #[inline(always)]
  pub fn evaluate(position: &Position, eval_params: &EvalParams) -> i32 {
    Evaluator::tapered_evaluation(position, eval_params)
  }

  #[inline(always)]
  fn tapered_evaluation(position: &Position, eval_params: &EvalParams) -> i32 {
    let mut mg_evaluation: i32 = 0;
    let mut eg_evaluation: i32 = 0;

    if position.is_endgame() {
      // A draw by insufficient material can only occur during endgames
      if Evaluator::is_draw_by_insufficient_material(position) {
        return 0;
      }
      eg_evaluation += Evaluator::king_cornering(position.get_king_square(PieceColor::White), position.get_king_square(PieceColor::Black));
      eg_evaluation -= Evaluator::king_cornering(position.get_king_square(PieceColor::Black), position.get_king_square(PieceColor::White));
    }

    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      let mut board = position.get_by_side_and_type(PieceColor::White, piece_type);
      while board != 0 {
        let square = get_lsb!(board);
        mg_evaluation += eval_params.get_mg_piece_value(piece_type) + eval_params.get_mg_psqt_value(piece_type, PieceColor::White, square);
        eg_evaluation += eval_params.get_eg_piece_value(piece_type) + eval_params.get_eg_psqt_value(piece_type, PieceColor::White, square);
        pop_lsb!(board);
      }

      let mut board = position.get_by_side_and_type(PieceColor::Black, piece_type);
      while board != 0 {
        let square = get_lsb!(board);
        mg_evaluation -= eval_params.get_mg_piece_value(piece_type) + eval_params.get_mg_psqt_value(piece_type, PieceColor::Black, square);
        eg_evaluation -= eval_params.get_eg_piece_value(piece_type) + eval_params.get_eg_psqt_value(piece_type, PieceColor::Black, square);
        pop_lsb!(board);
      }
    }

    let phase = position.get_phase();
    (((mg_evaluation * (256 - phase)) + (eg_evaluation * phase)) >> 8) * position.get_side().to_i32()
  }

  #[inline(always)]
  fn king_cornering(friendly_square: Square, opponent_square: Square) -> i32 {
    let mut evaluation: i32 = 0;
    let distance_between_kings: i32 = (friendly_square.get_file().abs_diff(opponent_square.get_file()) + friendly_square.get_rank().abs_diff(opponent_square.get_rank())) as i32;

    evaluation += 6 * ARR_CENTER_MANHATTAN_DISTANCE[opponent_square as usize];
    evaluation += 2 * (14 - distance_between_kings);

    evaluation
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
}
