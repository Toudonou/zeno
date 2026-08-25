use crate::containers::ByPieceType;
use crate::params::{
  EG_BISHOP_PAIR, EG_DOUBLED_PAWNS, EG_PASSED_PAWNS, EG_PIECES_SQUARES_TABLES, EG_PIECES_VALUES, MG_BISHOP_PAIR, MG_DOUBLED_PAWNS, MG_PASSED_PAWNS, MG_PIECES_SQUARES_TABLES, MG_PIECES_VALUES,
};
use crate::piece::{PieceColor, PieceType};
use crate::square::Square;
use crate::utils::get_psqt_index;

pub static EVAL_PARAMS_DEFAULT: EvalParams = EvalParams::default();

#[derive(Clone, Debug)]
pub struct EvalParams {
  mg_pieces_values: ByPieceType<i32>,
  eg_pieces_values: ByPieceType<i32>,

  mg_psqt_values: ByPieceType<[i32; 64]>,
  eg_psqt_values: ByPieceType<[i32; 64]>,

  mg_bishop_pair_value: i32,
  eg_bishop_pair_value: i32,

  mg_doubled_pawns_value: [i32; 8],
  eg_doubled_pawns_value: [i32; 8],
  mg_passed_pawns_value: [i32; 8],
  eg_passed_pawns_value: [i32; 8],
}

impl EvalParams {
  #[inline(always)]
  const fn default() -> Self {
    EvalParams {
      mg_pieces_values: MG_PIECES_VALUES,
      eg_pieces_values: EG_PIECES_VALUES,
      mg_psqt_values: MG_PIECES_SQUARES_TABLES,
      eg_psqt_values: EG_PIECES_SQUARES_TABLES,
      mg_bishop_pair_value: MG_BISHOP_PAIR,
      eg_bishop_pair_value: EG_BISHOP_PAIR,
      mg_doubled_pawns_value: MG_DOUBLED_PAWNS,
      eg_doubled_pawns_value: EG_DOUBLED_PAWNS,
      mg_passed_pawns_value: MG_PASSED_PAWNS,
      eg_passed_pawns_value: EG_PASSED_PAWNS,
    }
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
  pub fn get_mg_psqt_value(&self, piece_type: PieceType, side: PieceColor, square: Square) -> i32 {
    self.mg_psqt_values[piece_type][get_psqt_index(side, square)]
  }

  #[inline(always)]
  pub fn get_eg_psqt_value(&self, piece_type: PieceType, side: PieceColor, square: Square) -> i32 {
    self.eg_psqt_values[piece_type][get_psqt_index(side, square)]
  }

  #[inline(always)]
  pub fn get_mg_bishop_pair_value(&self) -> i32 {
    self.mg_bishop_pair_value
  }

  #[inline(always)]
  pub fn get_eg_bishop_pair_value(&self) -> i32 {
    self.eg_bishop_pair_value
  }

  #[inline(always)]
  pub fn get_mg_doubled_pawns_value(&self, file: u8) -> i32 {
    self.mg_doubled_pawns_value[file as usize]
  }

  #[inline(always)]
  pub fn get_eg_doubled_pawns_value(&self, file: u8) -> i32 {
    self.eg_doubled_pawns_value[file as usize]
  }

  #[inline(always)]
  pub fn get_mg_passed_pawns_value(&self, rank: u8, side: PieceColor) -> i32 {
    self.mg_passed_pawns_value[if side == PieceColor::White { rank } else { 7 - rank } as usize]
  }

  #[inline(always)]
  pub fn get_eg_passed_pawns_value(&self, rank: u8, side: PieceColor) -> i32 {
    self.eg_passed_pawns_value[if side == PieceColor::White { rank } else { 7 - rank } as usize]
  }
}
