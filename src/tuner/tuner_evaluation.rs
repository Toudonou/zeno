use crate::tuner::features::MAX_FEATURES;
use crate::tuner::position_ir::PositionIR;
use crate::tuner::tuner_params::TunerParams;

#[inline(always)]
pub fn tuner_evaluation(position_ir: &PositionIR, eval_params: &TunerParams) -> f32 {
  let mut mg_evaluation = 0.0;
  let mut eg_evaluation = 0.0;

  for i in 0..MAX_FEATURES {
    mg_evaluation += position_ir.board[i] as f32 * eval_params.mg[i];
    eg_evaluation += position_ir.board[i] as f32 * eval_params.eg[i];
  }

  position_ir.mg_factor * mg_evaluation + position_ir.eg_factor * eg_evaluation
}
