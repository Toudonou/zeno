use crate::piece::{PieceColor, PieceType};
use crate::position::Position;
use crate::psqt::{TOTAL_PHASE, get_eg_piece_value, get_eg_psqt_value, get_mg_piece_value, get_mg_psqt_value, get_phase};
use crate::square::Square;
use crate::{get_lsb, pop_lsb};

pub struct Evaluator {}

impl Evaluator {
  #[inline(always)]
  pub fn evaluate(position: &Position) -> i32 {
    Evaluator::tapered_evaluation(position)
  }

  #[inline(always)]
  fn tapered_evaluation(position: &Position) -> i32 {
    let mut phase = TOTAL_PHASE;
    let mut mg_evaluation: i32 = 0;
    let mut eg_evaluation: i32 = 0;

    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      let mut board = position.get_by_side_and_type(PieceColor::White, piece_type);
      while board != 0 {
        let square = get_lsb!(board);

        phase -= get_phase(piece_type);
        mg_evaluation += get_mg_piece_value(piece_type) + get_mg_psqt_value(piece_type, PieceColor::White, square);
        eg_evaluation += get_eg_piece_value(piece_type) + get_eg_psqt_value(piece_type, PieceColor::White, square);

        pop_lsb!(board);
      }

      let mut board = position.get_by_side_and_type(PieceColor::Black, piece_type);
      while board != 0 {
        let square = get_lsb!(board);

        phase -= get_phase(piece_type);
        mg_evaluation -= get_mg_piece_value(piece_type) + get_mg_psqt_value(piece_type, PieceColor::Black, square);
        eg_evaluation -= get_eg_piece_value(piece_type) + get_eg_psqt_value(piece_type, PieceColor::Black, square);

        pop_lsb!(board);
      }
    }

    phase = phase.max(0); // If we have a custom setup with more pieces than a normal chess board start position
    phase = (phase * 256 + (TOTAL_PHASE / 2)) / TOTAL_PHASE; // phase from [0, 24] to [0, 256]

    ((mg_evaluation * (256 - phase)) + (eg_evaluation * phase)) / 256
  }
}
