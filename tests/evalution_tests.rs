#[cfg(test)]
mod evaluation_tests {
    use zeno::evaluation::evaluate;
    use zeno::history::History;
    use zeno::position::Position;
    use zeno::search::Searcher;
    use zeno::transposition_table::TranspositionTable;
    use zeno::uci::uci_move;
    use zeno::utils::START_POSITION;

    #[test]
    fn is_draw_by_insufficient_material_k_vs_k() {
        let position = Position::from_fen("8/7K/8/8/8/8/k7/8 w - - 0 1", None);
        let evaluation = evaluate(&position);

        assert_eq!(evaluation, 0);
    }

    #[test]
    fn is_draw_by_insufficient_material_kn_vs_k() {
        let position = Position::from_fen("8/7K/8/4N3/8/8/k7/8 b - - 0 1", None);
        let evaluation = evaluate(&position);

        assert_eq!(evaluation, 0);
    }

    #[test]
    fn is_draw_by_insufficient_material_k_vs_kb() {
        let position = Position::from_fen("8/7K/8/4b3/8/8/k7/8 w - - 0 1", None);
        let evaluation = evaluate(&position);

        assert_eq!(evaluation, 0);
    }

    #[test]
    fn is_draw_by_insufficient_material_kb_same_color_vs_kb_same_color() {
        let position = Position::from_fen("8/2B2K2/8/4b3/8/8/1k6/8 w - - 0 1", None);
        let evaluation = evaluate(&position);

        assert_eq!(evaluation, 0);
    }

    #[test]
    fn is_draw_by_repetition() {
        let mut history = History::new();
        let mut history = Some(&mut history);

        let mut position = Position::from_fen(START_POSITION, history.as_deref_mut());

        // Game link: https://lichess.org/7WQb1tNl
        let moves = "d2d4 b7b6 c2c4 c8b7 b1c3 e7e6 e2e4 g8f6 f1d3 c7c5 d4d5 d7d6 d1c2 e6e5 g1e2 g7g6 e1g1 f8g7 f2f4 e8g8 f4f5 b8d7 e2g3 d8e7 c1g5 h7h6 g5e3 g6g5 h2h3 f6h5 g3h5 f7f6 h5g7 e7g7 g1f2 a7a6 h3h4 g8f7 h4g5 h6g5 f1g1 f8h8 f2e2 h8h4 e3f2 h4h2 f2g3 h2h5 e2d2 a8h8 d3e2 h5h6 a1f1 d7f8 c2d3 f8d7 d3e3 d7f8 e3f2 f8d7 f2e3 d7b8 c3a4 b8d7 a4c3".split_whitespace();
        moves.for_each(|move_string| position.make_move(&uci_move(move_string, &position), history.as_deref_mut()));

        let mut searcher = Searcher::new();
        let mut transposition_table = Some(&mut TranspositionTable::new());
        let search_result = searcher.search(&position, history.as_deref_mut(), transposition_table.as_deref_mut(), 10 * 1000);

        assert_eq!(search_result, None);
    }
}