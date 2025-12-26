use crate::containers::ByPieceType;
use crate::piece::{PieceColor, PieceType};
use crate::pop_lsb;
use crate::position::Position;

// Values from https://github.com/jhonnold/berserk (Initial Release)
static PAWN_VALUE: i32 = 100;
static KNIGHT_VALUE: i32 = 554;
static BISHOP_VALUE: i32 = 557;
static ROOK_VALUE: i32 = 698;
static QUEEN_VALUE: i32 = 1578;
static KING_VALUE: i32 = 10_000;

pub static PIECES_VALUES: ByPieceType<i32> = ByPieceType::new(PAWN_VALUE, KNIGHT_VALUE, BISHOP_VALUE, ROOK_VALUE, QUEEN_VALUE, KING_VALUE);

pub struct Evaluator {}

impl Evaluator {
  #[inline(always)]
  pub fn evaluate(position: &Position) -> i32 {
    Evaluator::material_evaluation(position)
  }

  #[inline(always)]
  fn material_evaluation(position: &Position) -> i32 {
    let mut evaluation: i32 = 0;

    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      let mut board = position.get_by_side_and_type(PieceColor::White, piece_type);
      while board != 0 {
        evaluation += PIECES_VALUES[piece_type];
        pop_lsb!(board);
      }

      let mut board = position.get_by_side_and_type(PieceColor::Black, piece_type);
      while board != 0 {
        evaluation -= PIECES_VALUES[piece_type];
        pop_lsb!(board);
      }
    }

    evaluation
  }
}
