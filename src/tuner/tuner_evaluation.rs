use crate::tuner::position_ir::PositionIR;
use crate::tuner::tuner_params::TunerParams;

#[inline(always)]
pub fn tuner_evaluation(position_ir: &PositionIR, tuner_params: &TunerParams) -> f32 {
  let mut mg_evaluation = 0.0;
  let mut eg_evaluation = 0.0;

  for (index, feature) in &position_ir.board {
    mg_evaluation += *feature as f32 * tuner_params.mg[*index];
    eg_evaluation += *feature as f32 * tuner_params.eg[*index];
  }

  position_ir.mg_factor * mg_evaluation + position_ir.eg_factor * eg_evaluation
}
