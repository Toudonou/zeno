use crate::containers::ByPieceType;
use crate::params::{EG_PIECES_SQUARES_TABLES, EG_PIECES_VALUES, MG_PIECES_SQUARES_TABLES, MG_PIECES_VALUES};
use crate::piece::{PieceColor, PieceType};
use crate::utils::get_psqt_index;

pub static EVAL_PARAMS_DEFAULT: EvalParams = EvalParams::default();

#[derive(Clone, Debug)]
pub struct EvalParams {
  mg_pieces_values: ByPieceType<i32>,
  eg_pieces_values: ByPieceType<i32>,
  mg_psqt_values: ByPieceType<[i32; 64]>,
  eg_psqt_values: ByPieceType<[i32; 64]>,
}

impl EvalParams {
  #[inline(always)]
  const fn default() -> Self {
    EvalParams { mg_pieces_values: MG_PIECES_VALUES, eg_pieces_values: EG_PIECES_VALUES, mg_psqt_values: MG_PIECES_SQUARES_TABLES, eg_psqt_values: EG_PIECES_SQUARES_TABLES }
  }

  #[inline(always)]
  pub fn get_mg_piece_value(&self, piece_type: PieceType) -> i32 {
    self.mg_pieces_values[piece_type]
  }

  #[inline(always)]
  pub fn get_eg_piece_value(&self, piece_type: PieceType) -> i32 {
    self.eg_pieces_values[piece_type]
  }

  #[inline(always)]
  pub fn get_mg_psqt_value(&self, piece_type: PieceType, side: PieceColor, square: u8) -> i32 {
    self.mg_psqt_values[piece_type][get_psqt_index(side, square)]
  }

  #[inline(always)]
  pub fn get_eg_psqt_value(&self, piece_type: PieceType, side: PieceColor, square: u8) -> i32 {
    self.eg_psqt_values[piece_type][get_psqt_index(side, square)]
  }
}
