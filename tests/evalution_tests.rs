#[cfg(test)]
mod evaluation_tests {
    use zeno::evaluation::evaluate;
    use zeno::history::History;
    use zeno::position::Position;

    #[test]
    fn is_draw_by_insufficient_material_k_vs_k() {
        let position = Position::from_fen("8/7K/8/8/8/8/k7/8 w - - 0 1");
        let evaluation = evaluate(&position);

        assert_eq!(evaluation, 0);
    }

    #[test]
    fn is_draw_by_insufficient_material_kn_vs_k() {
        let position = Position::from_fen("8/7K/8/4N3/8/8/k7/8 b - - 0 1");
        let evaluation = evaluate(&position);

        assert_eq!(evaluation, 0);
    }

    #[test]
    fn is_draw_by_insufficient_material_k_vs_kb() {
        let position = Position::from_fen("8/7K/8/4b3/8/8/k7/8 w - - 0 1");
        let evaluation = evaluate(&position);

        assert_eq!(evaluation, 0);
    }

    #[test]
    fn is_draw_by_insufficient_material_kb_same_color_vs_kb_same_color() {
        let position = Position::from_fen("8/2B2K2/8/4b3/8/8/1k6/8 w - - 0 1");
        let evaluation = evaluate(&position);

        assert_eq!(evaluation, 0);
    }
}