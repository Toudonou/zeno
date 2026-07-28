#[cfg(test)]
mod transposition_tests {
  use zeno::moves::{Move, MoveType};
  use zeno::pos_eval::Evaluation;
  use zeno::position::Position;
  use zeno::transposition_table::{AtomicTTEntry, TTFlag};
  use zeno::utils::START_POSITION;

  #[test]
  fn test_entry_encoding_and_decoding() {
    let position = Position::from_fen(START_POSITION);
    let best_move = Some(Move::new(25, 35, MoveType::EnPassant));
    let depth = 5;
    let flag = TTFlag::LowerBound;
    let evaluation = Evaluation::CentiPawns(25);
    let ply = 0;
    let tt_entry = AtomicTTEntry::new(position.get_zobrist_hash(), best_move, depth, flag, evaluation, ply).get_tt_entry();
    assert_eq!(tt_entry.get_hash(), position.get_zobrist_hash());
    assert_eq!(tt_entry.get_best_move(), best_move);
    assert_eq!(tt_entry.get_depth(), depth);
    assert_eq!(tt_entry.get_flag(), flag);
    assert_eq!(tt_entry.get_evaluation(ply), evaluation);

    let position = Position::from_fen(START_POSITION);
    let best_move = Some(Move::new(25, 35, MoveType::Normal));
    let depth = 5;
    let flag = TTFlag::LowerBound;
    let evaluation = Evaluation::MateIn(8);
    let ply = 5;
    let tt_entry = AtomicTTEntry::new(position.get_zobrist_hash(), best_move, depth, flag, evaluation, ply).get_tt_entry();
    assert_eq!(tt_entry.get_hash(), position.get_zobrist_hash());
    assert_eq!(tt_entry.get_best_move(), best_move);
    assert_eq!(tt_entry.get_depth(), depth);
    assert_eq!(tt_entry.get_flag(), flag);
    assert_eq!(tt_entry.get_evaluation(ply), evaluation);
    assert_eq!(tt_entry.get_evaluation(1), Evaluation::MateIn(4));

    let position = Position::from_fen(START_POSITION);
    let best_move = Some(Move::new(25, 35, MoveType::Normal));
    let depth = 5;
    let flag = TTFlag::LowerBound;
    let evaluation = Evaluation::MateIn(-6);
    let ply = 5;
    let tt_entry = AtomicTTEntry::new(position.get_zobrist_hash(), best_move, depth, flag, evaluation, ply).get_tt_entry();
    assert_eq!(tt_entry.get_hash(), position.get_zobrist_hash());
    assert_eq!(tt_entry.get_best_move(), best_move);
    assert_eq!(tt_entry.get_depth(), depth);
    assert_eq!(tt_entry.get_flag(), flag);
    assert_eq!(tt_entry.get_evaluation(ply), evaluation);
    assert_eq!(tt_entry.get_evaluation(1), Evaluation::MateIn(-2));

    let position = Position::from_fen(START_POSITION);
    let best_move = Some(Move::new(25, 35, MoveType::Normal));
    let depth = 5;
    let flag = TTFlag::LowerBound;
    let evaluation = Evaluation::CentiPawns(-6);
    let ply = 5;
    let tt_entry = AtomicTTEntry::new(position.get_zobrist_hash(), best_move, depth, flag, evaluation, ply).get_tt_entry();
    assert_eq!(tt_entry.get_hash(), position.get_zobrist_hash());
    assert_eq!(tt_entry.get_best_move(), best_move);
    assert_eq!(tt_entry.get_depth(), depth);
    assert_eq!(tt_entry.get_flag(), flag);
    assert_eq!(tt_entry.get_evaluation(ply), evaluation);
    assert_eq!(tt_entry.get_evaluation(1), evaluation);
  }
}
