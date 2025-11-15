#[cfg(test)]
mod zobrist_hash_test {
    use zeno::history::History;
    use zeno::moves::Move;
    use zeno::piece::PieceColor;
    use zeno::position::Position;

    #[test]
    fn test_zobrist_hash_normal_move_by_white() {
        let mut history = History::new();
        
        let mut position = Position::from_fen("3b1rk1/1bq3pp/5pn1/1p2rN2/2p1p3/2P1B2Q/1PB2PPP/R2R2K1 w - - 0 1", &mut history);
        assert_eq!(position.get_turn(), PieceColor::White);

        let mov = Move::from_uci_notation("d1d7", &position);
        position.make_move(&mov, &mut history);

        let expected_position = Position::from_fen("3b1rk1/1bqR2pp/5pn1/1p2rN2/2p1p3/2P1B2Q/1PB2PPP/R5K1 b - - 1 1", &mut history);

        assert_eq!(position.get_hash(), expected_position.get_hash());
        assert_eq!(position.get_turn(), PieceColor::Black);
    }

    #[test]
    fn test_zobrist_hash_short_castle_by_white() {
        let mut history = History::new();
        
        let mut position = Position::from_fen("r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4", &mut history);
        assert_eq!(position.get_turn(), PieceColor::White);

        let mov = Move::from_uci_notation("e1g1", &position);
        position.make_move(&mov, &mut history);

        let expected_position = Position::from_fen("r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4", &mut history);

        assert_eq!(position.get_hash(), expected_position.get_hash());
        assert_eq!(position.get_turn(), PieceColor::Black);
    }

    #[test]
    fn test_zobrist_hash_long_castle_by_white() {
        let mut history = History::new();
        
        let mut position = Position::from_fen("rnb1kb1r/pp3ppp/4p3/q2p4/3QnB2/2N5/PPP1PPPP/R3KBNR w KQkq - 0 8", &mut history);
        assert_eq!(position.get_turn(), PieceColor::White);

        let mov = Move::from_uci_notation("e1c1", &position);
        position.make_move(&mov, &mut history);

        let expected_position = Position::from_fen("rnb1kb1r/pp3ppp/4p3/q2p4/3QnB2/2N5/PPP1PPPP/2KR1BNR b kq - 1 8", &mut history);

        assert_eq!(position.get_hash(), expected_position.get_hash());
        assert_eq!(position.get_turn(), PieceColor::Black);
    }

    #[test]
    fn test_zobrist_hash_en_passant_move_by_white() {
        let mut history = History::new();
        
        let mut position = Position::from_fen("r1bqkbnr/ppp3pp/2n5/4Pp2/2Bp4/5N2/PP3PPP/RNBQK2R w KQkq f6 0 7", &mut history);
        assert_eq!(position.get_turn(), PieceColor::White);

        let mov = Move::from_uci_notation("e5f6", &position);
        position.make_move(&mov, &mut history);

        let expected_position = Position::from_fen("r1bqkbnr/ppp3pp/2n2P2/8/2Bp4/5N2/PP3PPP/RNBQK2R b KQkq - 0 7", &mut history);

        assert_eq!(position.get_hash(), expected_position.get_hash());
        assert_eq!(position.get_turn(), PieceColor::Black);
    }

    #[test]
    fn test_zobrist_hash_promotion_move_by_white() {
        let mut history = History::new();
        

        let mut position_for_knight_promotion = Position::from_fen("r1b1kbnr/ppp3Pp/3q4/4n3/2Bp4/5N2/PP3PPP/RNBQK2R w KQkq - 1 9", &mut history);
        assert_eq!(position_for_knight_promotion.get_turn(), PieceColor::White);

        let mov = Move::from_uci_notation("g7h8n", &position_for_knight_promotion);
        position_for_knight_promotion.make_move(&mov, &mut history);

        let mut position_for_bishop_promotion = Position::from_fen("r1b1kbnr/ppp3Pp/3q4/4n3/2Bp4/5N2/PP3PPP/RNBQK2R w KQkq - 1 9", &mut history);
        assert_eq!(position_for_bishop_promotion.get_turn(), PieceColor::White);

        let mov = Move::from_uci_notation("g7f8b", &position_for_bishop_promotion);
        position_for_bishop_promotion.make_move(&mov, &mut history);

        let mut position_for_rook_promotion = Position::from_fen("r3kbnr/1pP3pp/p7/3qn3/3pP1b1/5N2/P4PPP/RNBQ1RK1 w - - 1 12", &mut history);
        assert_eq!(position_for_rook_promotion.get_turn(), PieceColor::White);

        let mov = Move::from_uci_notation("c7c8r", &position_for_rook_promotion);
        position_for_rook_promotion.make_move(&mov, &mut history);

        let mut position_for_queen_promotion = Position::from_fen("r3kbnr/1pP3pp/p7/3qn3/3pP1b1/5N2/P4PPP/RNBQ1RK1 w - - 1 12", &mut history);
        assert_eq!(position_for_queen_promotion.get_turn(), PieceColor::White);

        let mov = Move::from_uci_notation("c7c8q", &position_for_queen_promotion);
        position_for_queen_promotion.make_move(&mov, &mut history);

        let expected_position_for_knight_promotion = Position::from_fen("r1b1kbnN/ppp4p/3q4/4n3/2Bp4/5N2/PP3PPP/RNBQK2R b KQq - 0 9", &mut history);
        let expected_position_for_bishop_promotion = Position::from_fen("r1b1kBnr/ppp4p/3q4/4n3/2Bp4/5N2/PP3PPP/RNBQK2R b KQkq - 0 9", &mut history);
        let expected_position_for_rook_promotion = Position::from_fen("r1R1kbnr/1p4pp/p7/3qn3/3pP1b1/5N2/P4PPP/RNBQ1RK1 b - - 0 12", &mut history);
        let expected_position_for_queen_promotion = Position::from_fen("r1Q1kbnr/1p4pp/p7/3qn3/3pP1b1/5N2/P4PPP/RNBQ1RK1 b - - 0 12", &mut history);

        assert_eq!(position_for_knight_promotion.get_hash(), expected_position_for_knight_promotion.get_hash());
        assert_eq!(position_for_bishop_promotion.get_hash(), expected_position_for_bishop_promotion.get_hash());
        assert_eq!(position_for_rook_promotion.get_hash(), expected_position_for_rook_promotion.get_hash());
        assert_eq!(position_for_queen_promotion.get_hash(), expected_position_for_queen_promotion.get_hash());

        assert_eq!(position_for_knight_promotion.get_turn(), PieceColor::Black);
        assert_eq!(position_for_bishop_promotion.get_turn(), PieceColor::Black);
        assert_eq!(position_for_rook_promotion.get_turn(), PieceColor::Black);
        assert_eq!(position_for_queen_promotion.get_turn(), PieceColor::Black);
    }

    #[test]
    fn test_zobrist_hash_normal_move_by_black() {
        let mut history = History::new();
        
        let mut position = Position::from_fen("r3r1k1/p1p2ppp/Q7/1p6/1P3p1b/5q1N/P4N1P/3R1K1R b - - 1 26", &mut history);
        assert_eq!(position.get_turn(), PieceColor::Black);

        let mov = Move::from_uci_notation("f3e2", &position);
        position.make_move(&mov, &mut history);

        let expected_position = Position::from_fen("r3r1k1/p1p2ppp/Q7/1p6/1P3p1b/7N/P3qN1P/3R1K1R w - - 2 27", &mut history);

        assert_eq!(position.get_hash(), expected_position.get_hash());
        assert_eq!(position.get_turn(), PieceColor::White);
    }

    #[test]
    fn test_zobrist_hash_short_castle_by_black() {
        let mut history = History::new();
        
        let mut position = Position::from_fen("rn2k2r/ppp1bppp/8/5q2/2NP1p2/2P2P1N/PP3K1P/R2Q1B1R b kq - 0 15", &mut history);
        assert_eq!(position.get_turn(), PieceColor::Black);

        let mov = Move::from_uci_notation("e8g8", &position);
        position.make_move(&mov, &mut history);

        let expected_position = Position::from_fen("rn3rk1/ppp1bppp/8/5q2/2NP1p2/2P2P1N/PP3K1P/R2Q1B1R w - - 1 16", &mut history);

        assert_eq!(position.get_hash(), expected_position.get_hash());
        assert_eq!(position.get_turn(), PieceColor::White);
    }

    #[test]
    fn test_zobrist_hash_long_castle_by_black() {
        let mut history = History::new();
        
        let mut position = Position::from_fen("r3k2r/pRp2ppn/2nqp2p/3p4/3P2P1/P1P1PN1P/2P2P2/2BQK2R b Kkq - 0 13", &mut history);
        assert_eq!(position.get_turn(), PieceColor::Black);

        let mov = Move::from_uci_notation("e8c8", &position);
        position.make_move(&mov, &mut history);

        let expected_position = Position::from_fen("2kr3r/pRp2ppn/2nqp2p/3p4/3P2P1/P1P1PN1P/2P2P2/2BQK2R w K - 1 14", &mut history);

        assert_eq!(position.get_hash(), expected_position.get_hash());
        assert_eq!(position.get_turn(), PieceColor::White);
    }

    #[test]
    fn test_zobrist_hash_en_passant_move_by_black() {
        let mut history = History::new();
        
        let mut position = Position::from_fen("2rq1r1k/1B2b3/p3p1P1/1p6/3P1pP1/PP1Q4/4n2P/R4KR1 b - g3 0 24", &mut history);
        assert_eq!(position.get_turn(), PieceColor::Black);

        let mov = Move::from_uci_notation("f4g3", &position);
        position.make_move(&mov, &mut history);

        let expected_position = Position::from_fen("2rq1r1k/1B2b3/p3p1P1/1p6/3P4/PP1Q2p1/4n2P/R4KR1 w - - 0 25", &mut history);

        assert_eq!(position.get_hash(), expected_position.get_hash());
        assert_eq!(position.get_turn(), PieceColor::White);
    }

    #[test]
    fn test_zobrist_hash_promotion_move_by_black() {
        let mut history = History::new();
        

        let mut position_for_knight_promotion = Position::from_fen("2rq3k/8/p3p1P1/1p6/3PB2b/PP5Q/5r1p/R2K2R1 b - - 3 28", &mut history);
        assert_eq!(position_for_knight_promotion.get_turn(), PieceColor::Black);

        let mov = Move::from_uci_notation("h2h1n", &position_for_knight_promotion);
        position_for_knight_promotion.make_move(&mov, &mut history);

        let mut position_for_bishop_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/p4r1p/1R1K4 b - - 4 33", &mut history);
        assert_eq!(position_for_bishop_promotion.get_turn(), PieceColor::Black);

        let mov = Move::from_uci_notation("a2b1b", &position_for_bishop_promotion);
        position_for_bishop_promotion.make_move(&mov, &mut history);

        let mut position_for_rook_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/p4r1p/1R1K4 b - - 4 33", &mut history);
        assert_eq!(position_for_rook_promotion.get_turn(), PieceColor::Black);

        let mov = Move::from_uci_notation("a2a1r", &position_for_rook_promotion);
        position_for_rook_promotion.make_move(&mov, &mut history);

        let mut position_for_queen_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/p4r1p/1R1K4 b - - 4 33", &mut history);
        assert_eq!(position_for_queen_promotion.get_turn(), PieceColor::Black);

        let mov = Move::from_uci_notation("h2h1q", &position_for_queen_promotion);
        position_for_queen_promotion.make_move(&mov, &mut history);

        let expected_position_for_knight_promotion = Position::from_fen("2rq3k/8/p3p1P1/1p6/3PB2b/PP5Q/5r2/R2K2Rn w - - 0 29", &mut history);
        let expected_position_for_bishop_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/5r1p/1b1K4 w - - 0 34", &mut history);
        let expected_position_for_rook_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/5r1p/rR1K4 w - - 0 34", &mut history);
        let expected_position_for_queen_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/p4r2/1R1K3q w - - 0 34", &mut history);

        assert_eq!(position_for_knight_promotion.get_hash(), expected_position_for_knight_promotion.get_hash());
        assert_eq!(position_for_bishop_promotion.get_hash(), expected_position_for_bishop_promotion.get_hash());
        assert_eq!(position_for_rook_promotion.get_hash(), expected_position_for_rook_promotion.get_hash());
        assert_eq!(position_for_queen_promotion.get_hash(), expected_position_for_queen_promotion.get_hash());

        assert_eq!(position_for_knight_promotion.get_turn(), PieceColor::White);
        assert_eq!(position_for_bishop_promotion.get_turn(), PieceColor::White);
        assert_eq!(position_for_rook_promotion.get_turn(), PieceColor::White);
        assert_eq!(position_for_queen_promotion.get_turn(), PieceColor::White);
    }
}