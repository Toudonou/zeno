#[cfg(test)]
mod zobrist_hash_test {
  use zeno::moves::Move;
  use zeno::piece::PieceColor;
  use zeno::position::Position;
  use zeno::utils::START_POSITION;

  #[test]
  fn test_zobrist_hash_polyglot_values() {
    // For hash verification: https://shinkarom.github.io/zobrist/

    // En passant for black
    let position = Position::from_fen("rnbqkbnr/8/8/8/pppppppp/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0x0b667f4815ea3b0e);

    let position = Position::from_fen("rnbqkbnr/8/8/8/Pppppppp/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0xba07ee13cf46d018);

    let position = Position::from_fen("rnbqkbnr/8/8/8/pPpppppp/8/2PPPPPP/RNBQKBNR b KQkq b3 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0xda9843864982ee80);

    let position = Position::from_fen("rnbqkbnr/8/8/8/pPPppppp/8/3PPPPP/RNBQKBNR b KQkq c3 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0xcb9f05ae49f1012c);

    let position = Position::from_fen("rnbqkbnr/8/8/8/pPPPpppp/8/4PPPP/RNBQKBNR b KQkq d3 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0xf32c86f4aace7a45);

    let position = Position::from_fen("rnbqkbnr/8/8/8/pPPPPppp/8/5PPP/RNBQKBNR b KQkq e3 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0xa8fe06ebbabd0ad1);

    let position = Position::from_fen("rnbqkbnr/8/8/8/pPPPpPpp/8/6PP/RNBQKBNR b KQkq f3 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0xfafa6872c7e9c138);

    let position = Position::from_fen("rnbqkbnr/8/8/8/pPPPpPPp/8/7P/RNBQKBNR b KQkq g3 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0xcf06001050152e80);

    // En passant for white
    let position = Position::from_fen("rbnqknbr/pppppppp/8/PPPPPPPP/8/8/8/RBNQKNBR b KQkq - 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0x2fb2c51fed28a34e);

    let position = Position::from_fen("rbnqknbr/1ppppppp/8/pPPPPPPP/8/8/8/RBNQKNBR w KQkq a6 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0x45723021c8c25d73);

    let position = Position::from_fen("rbnqknbr/2pppppp/8/ppPPPPPP/8/8/8/RBNQKNBR w KQkq b6 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0xfc84484f238d0959);

    let position = Position::from_fen("rbnqknbr/3ppppp/8/pPpPPPPP/8/8/8/RBNQKNBR w KQkq c6 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0x9e59bbf864faf07f);

    let position = Position::from_fen("rbnqknbr/4pppp/8/pPBpPPPP/8/8/8/RBNQKN1R w KQkq d6 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0x87973947f59667fd);

    let position = Position::from_fen("rbnqknbr/5ppp/8/pPpppPPP/8/8/8/RBNQKN1R w KQkq e6 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0xf3773c3b91e55bd6);

    let position = Position::from_fen("rbnqknbr/6pp/8/pPppppPP/8/8/8/RBNQKN1R w KQkq f6 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0x1a6fd715bb64c649);

    let position = Position::from_fen("rbnqknbr/7p/8/pPpppppP/8/8/8/RBNQKN1R w KQkq g6 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0x8a5895747f8a183f);

    let position = Position::from_fen("rbnqknbr/8/8/pPpppppp/8/8/8/RBNQKN1R w KQkq h6 0 1", None);
    assert_eq!(position.get_zobrish_hash(), 0x7b12dcf34309a006);

    // Tests from http://hgm.nubati.net/book_format.html
    let mut position = Position::from_fen(START_POSITION, None);
    assert_eq!(position.get_zobrish_hash(), 0x463b96181691fc9c);

    let mov = Move::from_uci_notation("e2e4", &position).unwrap();
    position.make_move(mov, None);
    assert_eq!(position.get_zobrish_hash(), 0x823c9b50fd114196);

    let mov = Move::from_uci_notation("d7d5", &position).unwrap();
    position.make_move(mov, None);
    assert_eq!(position.get_zobrish_hash(), 0x0756b94461c50fb0);

    let mov = Move::from_uci_notation("e4e5", &position).unwrap();
    position.make_move(mov, None);
    assert_eq!(position.get_zobrish_hash(), 0x662fafb965db29d4);

    let mov = Move::from_uci_notation("f7f5", &position).unwrap();
    position.make_move(mov, None);
    assert_eq!(position.get_zobrish_hash(), 0x22a48b5a8e47ff78);

    let mov = Move::from_uci_notation("e1e2", &position).unwrap();
    position.make_move(mov, None);
    assert_eq!(position.get_zobrish_hash(), 0x652a607ca3f242c1);

    let mov = Move::from_uci_notation("e8f7", &position).unwrap();
    position.make_move(mov, None);
    assert_eq!(position.get_zobrish_hash(), 0x00fdd303c946bdd9);

    let mut position = Position::from_fen(START_POSITION, None);
    let moves = "a2a4 b7b5 h2h4 b5b4 c2c4".split_whitespace();
    moves.for_each(|move_string| match Move::from_uci_notation(move_string, &position) {
      Some(mov) => position.make_move(mov, None),
      None => {}
    });
    assert_eq!(position.get_zobrish_hash(), 0x3c8123ea7b067637);

    let mov = Move::from_uci_notation("b4c3", &position).unwrap();
    position.make_move(mov, None);
    let mov = Move::from_uci_notation("a1a3", &position).unwrap();
    position.make_move(mov, None);
    assert_eq!(position.get_zobrish_hash(), 0x5c3f9b829b279560);
  }

  #[test]
  fn test_zobrist_hash_normal_move_by_white() {
    let mut position = Position::from_fen("3b1rk1/1bq3pp/5pn1/1p2rN2/2p1p3/2P1B2Q/1PB2PPP/R2R2K1 w - - 0 1", None);
    assert_eq!(position.get_side(), PieceColor::White);

    let mov = Move::from_uci_notation("d1d7", &position).unwrap();
    position.make_move(mov, None);
    let expected_position = Position::from_fen("3b1rk1/1bqR2pp/5pn1/1p2rN2/2p1p3/2P1B2Q/1PB2PPP/R5K1 b - - 1 1", None);
    assert_eq!(position.get_zobrish_hash(), expected_position.get_zobrish_hash());
    assert_eq!(position.get_side(), PieceColor::Black);
  }

  #[test]
  fn test_zobrist_hash_short_castle_by_white() {
    let mut position = Position::from_fen("r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4", None);
    assert_eq!(position.get_side(), PieceColor::White);

    let mov = Move::from_uci_notation("e1g1", &position).unwrap();
    position.make_move(mov, None);
    let expected_position = Position::from_fen("r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4", None);
    assert_eq!(position.get_zobrish_hash(), expected_position.get_zobrish_hash());
    assert_eq!(position.get_side(), PieceColor::Black);
  }

  #[test]
  fn test_zobrist_hash_long_castle_by_white() {
    let mut position = Position::from_fen("rnb1kb1r/pp3ppp/4p3/q2p4/3QnB2/2N5/PPP1PPPP/R3KBNR w KQkq - 0 8", None);
    assert_eq!(position.get_side(), PieceColor::White);

    let mov = Move::from_uci_notation("e1c1", &position).unwrap();
    position.make_move(mov, None);
    let expected_position = Position::from_fen("rnb1kb1r/pp3ppp/4p3/q2p4/3QnB2/2N5/PPP1PPPP/2KR1BNR b kq - 1 8", None);
    assert_eq!(position.get_zobrish_hash(), expected_position.get_zobrish_hash());
    assert_eq!(position.get_side(), PieceColor::Black);
  }

  #[test]
  fn test_zobrist_hash_en_passant_move_by_white() {
    let mut position = Position::from_fen("r1bqkbnr/ppp3pp/2n5/4Pp2/2Bp4/5N2/PP3PPP/RNBQK2R w KQkq f6 0 7", None);
    assert_eq!(position.get_side(), PieceColor::White);

    let mov = Move::from_uci_notation("e5f6", &position).unwrap();
    position.make_move(mov, None);
    let expected_position = Position::from_fen("r1bqkbnr/ppp3pp/2n2P2/8/2Bp4/5N2/PP3PPP/RNBQK2R b KQkq - 0 7", None);
    assert_eq!(position.get_zobrish_hash(), expected_position.get_zobrish_hash());
    assert_eq!(position.get_side(), PieceColor::Black);
  }

  #[test]
  fn test_zobrist_hash_promotion_move_by_white() {
    let mut position_for_knight_promotion = Position::from_fen("r1b1kbnr/ppp3Pp/3q4/4n3/2Bp4/5N2/PP3PPP/RNBQK2R w KQkq - 1 9", None);
    assert_eq!(position_for_knight_promotion.get_side(), PieceColor::White);

    let mov = Move::from_uci_notation("g7h8n", &position_for_knight_promotion).unwrap();
    position_for_knight_promotion.make_move(mov, None);

    let mut position_for_bishop_promotion = Position::from_fen("r1b1kbnr/ppp3Pp/3q4/4n3/2Bp4/5N2/PP3PPP/RNBQK2R w KQkq - 1 9", None);
    assert_eq!(position_for_bishop_promotion.get_side(), PieceColor::White);

    let mov = Move::from_uci_notation("g7f8b", &position_for_bishop_promotion).unwrap();
    position_for_bishop_promotion.make_move(mov, None);

    let mut position_for_rook_promotion = Position::from_fen("r3kbnr/1pP3pp/p7/3qn3/3pP1b1/5N2/P4PPP/RNBQ1RK1 w - - 1 12", None);
    assert_eq!(position_for_rook_promotion.get_side(), PieceColor::White);

    let mov = Move::from_uci_notation("c7c8r", &position_for_rook_promotion).unwrap();
    position_for_rook_promotion.make_move(mov, None);

    let mut position_for_queen_promotion = Position::from_fen("r3kbnr/1pP3pp/p7/3qn3/3pP1b1/5N2/P4PPP/RNBQ1RK1 w - - 1 12", None);
    assert_eq!(position_for_queen_promotion.get_side(), PieceColor::White);

    let mov = Move::from_uci_notation("c7c8q", &position_for_queen_promotion).unwrap();
    position_for_queen_promotion.make_move(mov, None);

    let expected_position_for_knight_promotion = Position::from_fen("r1b1kbnN/ppp4p/3q4/4n3/2Bp4/5N2/PP3PPP/RNBQK2R b KQq - 0 9", None);
    let expected_position_for_bishop_promotion = Position::from_fen("r1b1kBnr/ppp4p/3q4/4n3/2Bp4/5N2/PP3PPP/RNBQK2R b KQkq - 0 9", None);
    let expected_position_for_rook_promotion = Position::from_fen("r1R1kbnr/1p4pp/p7/3qn3/3pP1b1/5N2/P4PPP/RNBQ1RK1 b - - 0 12", None);
    let expected_position_for_queen_promotion = Position::from_fen("r1Q1kbnr/1p4pp/p7/3qn3/3pP1b1/5N2/P4PPP/RNBQ1RK1 b - - 0 12", None);

    assert_eq!(position_for_knight_promotion.get_zobrish_hash(), expected_position_for_knight_promotion.get_zobrish_hash());
    assert_eq!(position_for_bishop_promotion.get_zobrish_hash(), expected_position_for_bishop_promotion.get_zobrish_hash());
    assert_eq!(position_for_rook_promotion.get_zobrish_hash(), expected_position_for_rook_promotion.get_zobrish_hash());
    assert_eq!(position_for_queen_promotion.get_zobrish_hash(), expected_position_for_queen_promotion.get_zobrish_hash());

    assert_eq!(position_for_knight_promotion.get_side(), PieceColor::Black);
    assert_eq!(position_for_bishop_promotion.get_side(), PieceColor::Black);
    assert_eq!(position_for_rook_promotion.get_side(), PieceColor::Black);
    assert_eq!(position_for_queen_promotion.get_side(), PieceColor::Black);
  }

  #[test]
  fn test_zobrist_hash_normal_move_by_black() {
    let mut position = Position::from_fen("r3r1k1/p1p2ppp/Q7/1p6/1P3p1b/5q1N/P4N1P/3R1K1R b - - 1 26", None);
    assert_eq!(position.get_side(), PieceColor::Black);

    let mov = Move::from_uci_notation("f3e2", &position).unwrap();
    position.make_move(mov, None);
    let expected_position = Position::from_fen("r3r1k1/p1p2ppp/Q7/1p6/1P3p1b/7N/P3qN1P/3R1K1R w - - 2 27", None);
    assert_eq!(position.get_zobrish_hash(), expected_position.get_zobrish_hash());
    assert_eq!(position.get_side(), PieceColor::White);
  }

  #[test]
  fn test_zobrist_hash_short_castle_by_black() {
    let mut position = Position::from_fen("rn2k2r/ppp1bppp/8/5q2/2NP1p2/2P2P1N/PP3K1P/R2Q1B1R b kq - 0 15", None);
    assert_eq!(position.get_side(), PieceColor::Black);

    let mov = Move::from_uci_notation("e8g8", &position).unwrap();
    position.make_move(mov, None);
    let expected_position = Position::from_fen("rn3rk1/ppp1bppp/8/5q2/2NP1p2/2P2P1N/PP3K1P/R2Q1B1R w - - 1 16", None);
    assert_eq!(position.get_zobrish_hash(), expected_position.get_zobrish_hash());
    assert_eq!(position.get_side(), PieceColor::White);
  }

  #[test]
  fn test_zobrist_hash_long_castle_by_black() {
    let mut position = Position::from_fen("r3k2r/pRp2ppn/2nqp2p/3p4/3P2P1/P1P1PN1P/2P2P2/2BQK2R b Kkq - 0 13", None);
    assert_eq!(position.get_side(), PieceColor::Black);

    let mov = Move::from_uci_notation("e8c8", &position).unwrap();
    position.make_move(mov, None);
    let expected_position = Position::from_fen("2kr3r/pRp2ppn/2nqp2p/3p4/3P2P1/P1P1PN1P/2P2P2/2BQK2R w K - 1 14", None);
    assert_eq!(position.get_zobrish_hash(), expected_position.get_zobrish_hash());
    assert_eq!(position.get_side(), PieceColor::White);
  }

  #[test]
  fn test_zobrist_hash_en_passant_move_by_black() {
    let mut position = Position::from_fen("2rq1r1k/1B2b3/p3p1P1/1p6/3P1pP1/PP1Q4/4n2P/R4KR1 b - g3 0 24", None);
    assert_eq!(position.get_side(), PieceColor::Black);

    let mov = Move::from_uci_notation("f4g3", &position).unwrap();
    position.make_move(mov, None);
    let expected_position = Position::from_fen("2rq1r1k/1B2b3/p3p1P1/1p6/3P4/PP1Q2p1/4n2P/R4KR1 w - - 0 25", None);
    assert_eq!(position.get_zobrish_hash(), expected_position.get_zobrish_hash());
    assert_eq!(position.get_side(), PieceColor::White);
  }

  #[test]
  fn test_zobrist_hash_promotion_move_by_black() {
    let mut position_for_knight_promotion = Position::from_fen("2rq3k/8/p3p1P1/1p6/3PB2b/PP5Q/5r1p/R2K2R1 b - - 3 28", None);
    assert_eq!(position_for_knight_promotion.get_side(), PieceColor::Black);

    let mov = Move::from_uci_notation("h2h1n", &position_for_knight_promotion).unwrap();
    position_for_knight_promotion.make_move(mov, None);

    let mut position_for_bishop_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/p4r1p/1R1K4 b - - 4 33", None);
    assert_eq!(position_for_bishop_promotion.get_side(), PieceColor::Black);

    let mov = Move::from_uci_notation("a2b1b", &position_for_bishop_promotion).unwrap();
    position_for_bishop_promotion.make_move(mov, None);

    let mut position_for_rook_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/p4r1p/1R1K4 b - - 4 33", None);
    assert_eq!(position_for_rook_promotion.get_side(), PieceColor::Black);

    let mov = Move::from_uci_notation("a2a1r", &position_for_rook_promotion).unwrap();
    position_for_rook_promotion.make_move(mov, None);

    let mut position_for_queen_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/p4r1p/1R1K4 b - - 4 33", None);
    assert_eq!(position_for_queen_promotion.get_side(), PieceColor::Black);

    let mov = Move::from_uci_notation("h2h1q", &position_for_queen_promotion).unwrap();
    position_for_queen_promotion.make_move(mov, None);

    let expected_position_for_knight_promotion = Position::from_fen("2rq3k/8/p3p1P1/1p6/3PB2b/PP5Q/5r2/R2K2Rn w - - 0 29", None);
    let expected_position_for_bishop_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/5r1p/1b1K4 w - - 0 34", None);
    let expected_position_for_rook_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/5r1p/rR1K4 w - - 0 34", None);
    let expected_position_for_queen_promotion = Position::from_fen("2rq4/1B5R/p3pkPQ/8/3P4/1P6/p4r2/1R1K3q w - - 0 34", None);

    assert_eq!(position_for_knight_promotion.get_zobrish_hash(), expected_position_for_knight_promotion.get_zobrish_hash());
    assert_eq!(position_for_bishop_promotion.get_zobrish_hash(), expected_position_for_bishop_promotion.get_zobrish_hash());
    assert_eq!(position_for_rook_promotion.get_zobrish_hash(), expected_position_for_rook_promotion.get_zobrish_hash());
    assert_eq!(position_for_queen_promotion.get_zobrish_hash(), expected_position_for_queen_promotion.get_zobrish_hash());

    assert_eq!(position_for_knight_promotion.get_side(), PieceColor::White);
    assert_eq!(position_for_bishop_promotion.get_side(), PieceColor::White);
    assert_eq!(position_for_rook_promotion.get_side(), PieceColor::White);
    assert_eq!(position_for_queen_promotion.get_side(), PieceColor::White);
  }
}
