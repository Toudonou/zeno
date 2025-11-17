#[cfg(test)]
mod position_tests {
    use zeno::position::Position;
    use zeno::uci::uci_move;
    use zeno::utils::START_POSITION;

    #[test]
    fn correct_half_move_clock_and_number_of_move() {
        let mut position = Position::from_fen(START_POSITION, None);

        // Game link: https://lichess.org/L84X4njg/black
        let moves = "d2d4 d7d5 c1f4 g8f6 g1f3 e7e6 b1c3 f8b4 a2a3 b4c3 b2c3 e8g8 d1d3 d8e7 c3c4 d5c4 d3c4 f6d5 e2e3 d5f4 e3f4 e7d6 g2g3 b8c6 f1d3 c8d7 e1g1 f7f5 f1b1 b7b6 a3a4 c6a5 c4a6 d6d5 d3e2 d5c6 a6d3 a8d8 b1b4 c6e4 d3e4 f5e4 f3e5 f8e8 c2c3 g8f8 b4b1 f8e7 e5d7 e7d7 g1g2 d7d6 f2f3 e4e3 e2d3 c7c5 d4c5 d6c5 b1b5 c5d6 d3h7 d6e7 h7e4 d8d2 g2g1 e7d6 b5g5 e8e7 a1e1 a5c4 g5h5 d2a2 e1b1 a2a4 b1d1 c4d2 h5b5 a4a3 c3c4 e7c7 b5g5 a3a2 g1g2 c7c4 e4b1 e3e2 d1h1 d2b1 h1e1 b1c3 g5g7 c4d4 g2f2 c3d1 f2g1 d4c4 g7g8 c4c2 h2h3 a7a5 g1h2 d1e3 h2g1 c2d2 g1f2 e3d1 f2g1 d6c5 g8e8 d1b2 e1c1 c5b5 e8c8 d2d1 g1g2 e2e1q c1c5 b6c5 c8b8 b5c4 h3h4 e1f1 g2h2 d1d2".split_whitespace();
        moves.for_each(|move_string| position.make_move(&uci_move(move_string, &position), None));

        assert_eq!(position.get_half_move_clock(), 3);
        assert_eq!(position.get_number_of_move(), 63);

        // Game link: https://lichess.org/qHgExmpR/black
        let mut position = Position::from_fen(START_POSITION, None);
        let moves = "b1c3 d7d5 g1f3 d5d4 c3e4 f7f5 e4c5 e7e5 c5d3 e5e4 b2b4 e4d3 c2c3 d4c3 d2c3 d8f6 d1d3 f8b4 d3b5 b8c6 a2a4 f6c3 f3d2 c3a1 e1d1 g8f6 d2b3 a1a2 b3d4 f6e4 c1e3 a2b1 e3c1 e4f2".split_whitespace();
        moves.for_each(|move_string| position.make_move(&uci_move(move_string, &position), None));

        assert_eq!(position.get_half_move_clock(), 0);
        assert_eq!(position.get_number_of_move(), 18);

        // Game link: https://lichess.org/aZVJVPLI/white
        let mut position = Position::from_fen(START_POSITION, None);
        let moves = "b1c3 g8f6 e2e4 d7d6 d1f3 a7a6 d2d4 b8c6 d4d5 c6e5 f3g3 c7c6 c1e3 c6d5 e4d5 e5g4 e3d2 d8a5 f1e2 h7h5 f2f3 g4e5 g3h4 f6d5 e2b5 a6b5 c3d5 a5d8 g1e2 e5c4 d2c1 e7e6 c1g5 d8d7 d5c3 c4b2 a1b1 f7f6 g5f6 g7f6 b1b2 f8e7 b2b5 f6f5 h4g3 a8a2 e2d4 a2a1 c3d1 e6e5 e1g1 e5d4 g3g7 d7b5 g7h8 e8d7 h8h7 b5e5 f3f4 e5e4 h7g7 b7b6 c2c4 c8a6 g7g3 e4e2 g3f3 e2f3 g2f3 a6c4 d1e3 c4f1 e3f1 d7e6 g1f2 d6d5 f1g3 h5h4 g3e2 e7c5 f2g2 d4d3 e2c3 d5d4 c3b5 d3d2 b5c7 e6f7 g2h3 d2d1q h3h4 d1f3 h4g5 f3g4 g5h6 g4g6".split_whitespace();
        moves.for_each(|move_string| position.make_move(&uci_move(move_string, &position), None));

        assert_eq!(position.get_half_move_clock(), 4);
        assert_eq!(position.get_number_of_move(), 49);

        // Game link: https://lichess.org/7WQb1tNl
        let mut position = Position::from_fen(START_POSITION, None);
        let moves = "d2d4 b7b6 c2c4 c8b7 b1c3 e7e6 e2e4 g8f6 f1d3 c7c5 d4d5 d7d6 d1c2 e6e5 g1e2 g7g6 e1g1 f8g7 f2f4 e8g8 f4f5 b8d7 e2g3 d8e7 c1g5 h7h6 g5e3 g6g5 h2h3 f6h5 g3h5 f7f6 h5g7 e7g7 g1f2 a7a6 h3h4 g8f7 h4g5 h6g5 f1g1 f8h8 f2e2 h8h4 e3f2 h4h2 f2g3 h2h5 e2d2 a8h8 d3e2 h5h6 a1f1 d7f8 c2d3 f8d7 d3e3 d7f8 e3f2 f8d7 f2e3 d7b8 c3a4 b8d7 a4c3".split_whitespace();
        moves.for_each(|move_string| position.make_move(&uci_move(move_string, &position), None));

        assert_eq!(position.get_half_move_clock(), 25);
        assert_eq!(position.get_number_of_move(), 33);
    }
}