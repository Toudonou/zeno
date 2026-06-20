#[cfg(test)]
mod evaluation_tests {
  use std::sync::Arc;
  use zeno::eval_params::EVAL_PARAMS_DEFAULT;
  use zeno::evaluator::Evaluator;
  use zeno::history::History;
  use zeno::moves::{Move, MoveType};
  use zeno::moves_picker::MovePicker;
  use zeno::piece::{PieceColor, PieceType};
  use zeno::pos_eval::{Evaluation, MATE_SCORE};
  use zeno::position::Position;
  use zeno::search_constants::SearchLimits;
  use zeno::search_pool::SearchPool;
  use zeno::square::{Square, SquareOps};
  use zeno::transposition_table::TranspositionTable;
  use zeno::utils::{MAX_PLY, START_POSITION};

  #[test]
  fn test_evaluation_conversion_from_i32_to_u32_and_reverse() {
    // Simple score
    for i in -(MATE_SCORE / 2)..=(MATE_SCORE / 2) {
      let initial_eval = Evaluation::CentiPawns(i);
      let value_after_double_transition = Evaluation::from_u32(initial_eval.to_u32());
      assert_eq!(value_after_double_transition, initial_eval);
    }

    // Mate score
    for i in -MAX_PLY..=MAX_PLY {
      let initial_eval = Evaluation::MateIn(i);
      let value_after_double_transition = Evaluation::from_u32(initial_eval.to_u32());
      assert_eq!(value_after_double_transition, initial_eval);
    }
  }

  #[test]
  fn is_draw_by_repetition() {
    // Game link: https://lichess.org/7WQb1tNl
    let mut history = History::new();
    let mut position = Position::from_fen(START_POSITION);
    history.save_hash(position.get_zobrist_hash());
    let moves = "d2d4 b7b6 c2c4 c8b7 b1c3 e7e6 e2e4 g8f6 f1d3 c7c5 d4d5 d7d6 d1c2 e6e5 g1e2 g7g6 e1g1 f8g7 f2f4 e8g8 f4f5 b8d7 e2g3 d8e7 c1g5 h7h6 g5e3 g6g5 h2h3 f6h5 g3h5 f7f6 h5g7 e7g7 g1f2 a7a6 h3h4 g8f7 h4g5 h6g5 f1g1 f8h8 f2e2 h8h4 e3f2 h4h2 f2g3 h2h5 e2d2 a8h8 d3e2 h5h6 a1f1 d7f8 c2d3 f8d7 d3e3 d7f8 e3f2 f8d7 f2e3 d7b8 c3a4 b8d7 a4c3".split_whitespace();
    moves.for_each(|move_string| match Move::from_uci_notation(move_string, &position) {
      Some(mov) => {
        position.make_move(mov);
        history.save_hash(position.get_zobrist_hash());
      }
      None => {}
    });
    let transposition_table = Arc::new(TranspositionTable::default());
    let search_pool = SearchPool::new(transposition_table, 1);
    let mov = search_pool.search(&mut position, &mut history, SearchLimits::ThinkingTime(5 * 1000));
    assert_eq!(mov, None);

    // Game link: https://lichess.org/hTK2QMUTul1t
    let mut history = History::new();
    let mut position = Position::from_fen(START_POSITION);
    history.save_hash(position.get_zobrist_hash());
    let moves = "d2d4 g8f6 b1c3 e7e6 d1d3 b8c6 a2a3 d7d5 g1f3 f8d6 h2h4 e8g8 h4h5 h7h6 c3b5 f6g4 h1h4 e6e5 b5d6 c7d6 d4e5 g4e5 f3e5 d8h4 e5c6 b7c6 d3f3 c8g4 f3f4 f8d8 c1e3 h4h5 f2f3 g4e6 g2g4 h5h4 e3f2 h4g5 f4g5 h6g5 e1d2 c6c5 f1g2 a8b8 b2b3 a7a5 f2g3 b8c8 a3a4 c5c4 b3c4 c8c4 e2e3 d8c8 a1c1 c8d8 c1a1 d8c8 a1c1 c8d8 c1a1".split_whitespace();
    moves.for_each(|move_string| match Move::from_uci_notation(move_string, &position) {
      Some(mov) => {
        position.make_move(mov);
        history.save_hash(position.get_zobrist_hash());
      }
      None => {}
    });
    let transposition_table = Arc::new(TranspositionTable::default());
    let search_pool = SearchPool::new(transposition_table, 1);
    let mov = search_pool.search(&mut position, &mut history, SearchLimits::ThinkingTime(5 * 1000));
    assert_eq!(mov, None);
  }

  #[test]
  fn is_draw_by_insufficient_material_k_vs_k() {
    assert_eq!(Evaluator::is_draw_by_insufficient_material(&Position::from_fen("8/7K/8/8/8/8/k7/8 w - - 0 1")), true);
  }

  #[test]
  fn is_draw_by_insufficient_material_kn_vs_k() {
    assert_eq!(Evaluator::is_draw_by_insufficient_material(&Position::from_fen("8/7K/8/4N3/8/8/k7/8 b - - 0 1")), true);
    assert_eq!(Evaluator::is_draw_by_insufficient_material(&Position::from_fen("8/7K/8/8/8/8/k2n4/8 w - - 0 1")), true);
  }

  #[test]
  fn is_draw_by_insufficient_material_k_vs_kb() {
    assert_eq!(Evaluator::is_draw_by_insufficient_material(&Position::from_fen("8/7K/8/4b3/8/8/k7/8 w - - 0 1")), true);
    assert_eq!(Evaluator::is_draw_by_insufficient_material(&Position::from_fen("8/7K/8/3B4/8/8/k7/8 b - - 0 1")), true);
  }

  #[test]
  fn is_draw_by_insufficient_material_kb_same_square_color_vs_kb_same_square_color() {
    assert_eq!(Evaluator::is_draw_by_insufficient_material(&Position::from_fen("8/5K2/8/4b3/8/8/1k3B2/8 b - - 0 1")), true);
    assert_eq!(Evaluator::is_draw_by_insufficient_material(&Position::from_fen("8/5K2/8/2B1b3/8/3b3B/1k6/8 w - - 0 1")), false, "There should only one bishop one each side");
  }

  #[test]
  fn test_see_capture() {
    let position = Position::from_fen("3r1rk1/pnpqb1p1/1p1p1n1p/4p3/P3P3/2PPBN1P/1P3RPN/R2Q2K1 w - - 0 1");
    let source = Square::from_algebraic_notation("f3");
    let destination = Square::from_algebraic_notation("e5");
    let see_value = MovePicker::see_capture(&position, Move::new(source, destination, MoveType::Normal));
    assert_eq!(see_value, EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Pawn) - EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Knight));
    assert!(see_value < 0, "This capture is bad for white");

    let position = Position::from_fen("3r1rk1/pnpqb1p1/1p1p1n1p/8/P3p1p1/2PPBN1P/1P2BRPN/R2Q2K1 b - - 0 1");
    let source = Square::from_algebraic_notation("e4");
    let destination = Square::from_algebraic_notation("f3");
    let see_value = MovePicker::see_capture(&position, Move::new(source, destination, MoveType::Normal));
    assert_eq!(
      see_value,
      EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Knight) + EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Pawn) - 2 * EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Pawn)
    );
    assert!(see_value > 0, "This capture is good for back");

    let position = Position::from_fen("3r1rk1/pnpqb1p1/1p1p1n1p/8/P5p1/2PPBp1P/1P2BRPN/R2Q2K1 w - - 0 1");
    let source = Square::from_algebraic_notation("g2");
    let destination = Square::from_algebraic_notation("f3");
    let see_value = MovePicker::see_capture(&position, Move::new(source, destination, MoveType::Normal));
    assert_eq!(see_value, 2 * EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Pawn) - EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Pawn));
    assert!(see_value > 0, "This capture is good for back");

    let position = Position::from_fen("3r1rk1/p1pq2p1/1p1pb2p/n7/P1n1p1p1/1P1PB1NP/1P2BR1N/R2Q2K1 b - - 0 1");
    let source = Square::from_algebraic_notation("b3");
    let destination = Square::from_algebraic_notation("c4");
    let see_value = MovePicker::see_capture(&position, Move::new(source, destination, MoveType::Normal));
    assert_eq!(see_value, EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Knight), "It's better for black to just give up the knight");

    let position = Position::from_fen("1k1r3q/1ppn3p/p4b2/4p3/8/P2N2P1/1PP1R1BP/2K1Q3 w - - 0 1");
    let source = Square::from_algebraic_notation("d3");
    let destination = Square::from_algebraic_notation("e5");
    let see_value = MovePicker::see_capture(&position, Move::new(source, destination, MoveType::Normal));
    assert_eq!(see_value, EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Pawn) - EVAL_PARAMS_DEFAULT.get_mg_piece_value(PieceType::Knight));
  }

  #[test]
  fn test_has_bishop_pair() {
    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), PieceColor::White), true);
    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), PieceColor::Black), true);

    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("rnBqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RN1QKBNR b KQkq - 0 1"), PieceColor::White), false);
    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("r2qr1k1/pp1n2pp/2pb1np1/3p4/3P2P1/7P/PP1NPPB1/R1BQ1RK1 w - - 0 13"), PieceColor::White), true);

    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("r2q1rk1/2p1ppbp/p1n3p1/1p1nP3/1P1P4/P1Nb1N1P/1BQ2PP1/3RK2R w K - 0 18"), PieceColor::Black), true);
    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("r2r2k1/1R1b1pbp/p3p1p1/3pPn2/2pP4/2P5/P1NNBPPP/5RK1 w - - 2 20"), PieceColor::Black), true);

    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("2b4k/1p5p/1q3n2/pP3pQ1/3P3R/P7/2P2PP1/2RK4 w - - 0 1"), PieceColor::White), false);
    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("2b4k/1p5p/1q3n2/pP3pQ1/3P3R/P7/2P2PP1/2RK4 w - - 0 1"), PieceColor::Black), false);

    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("r6r/1pp2pk1/p1n3p1/2Np4/3P2Pq/P2P3P/1P1Q1PK1/R4R2 w - - 4 20"), PieceColor::White), false);
    assert_eq!(Evaluator::has_bishop_pair(&Position::from_fen("r6r/1pp2pk1/p1n3p1/2Np4/3P2Pq/P2P3P/1P1Q1PK1/R4R2 w - - 4 20"), PieceColor::Black), false);
  }
}
