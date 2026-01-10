#[cfg(test)]
mod transposition_tests {
  use zeno::moves::{Move, MoveType};
  use zeno::pos_eval::Evaluation;
  use zeno::position::Position;
  use zeno::transposition_table::{TTEntry, TTFlag};
  use zeno::utils::START_POSITION;

  #[test]
  fn test_entry_encoding_and_decoding() {
    let position = Position::from_fen(START_POSITION);
    let best_move = Some(Move::new(25, 35, MoveType::EnPassant));
    let depth = 5;
    let flag = TTFlag::LowerBound;
    let evaluation = Evaluation::MateIn(25);

    let tt_entry = TTEntry::new(position.get_zobrist_hash(), best_move, depth, flag, evaluation);

    assert_eq!(tt_entry.get_hash(), position.get_zobrist_hash());
    assert_eq!(tt_entry.get_best_move(), best_move);
    assert_eq!(tt_entry.get_depth(), depth);
    assert_eq!(tt_entry.get_flag(), flag);
    assert_eq!(tt_entry.get_evaluation(), evaluation);
  }
}
