#[cfg(test)]
mod evaluation_tests {
  use zeno::pos_eval::{Evaluation, MATE_SCORE};
  use zeno::search::MAX_PLY;

  #[test]
  fn test_evaluation_conversion_from_i32_to_u32_and_reverse() {
    // Simple score
    for i in -(MATE_SCORE / 2)..=(MATE_SCORE / 2) {
      let initial_eval = Evaluation::Score(i);
      let value_after_double_transition = Evaluation::from_u32(initial_eval.to_u32());
      assert_eq!(value_after_double_transition, initial_eval);
    }

    // Mate score
    for i in -(MAX_PLY as i32)..=MAX_PLY as i32 {
      let initial_eval = Evaluation::MateIn(i);
      let value_after_double_transition = Evaluation::from_u32(initial_eval.to_u32());
      assert_eq!(value_after_double_transition, initial_eval);
    }
  }
}
