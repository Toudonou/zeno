#[cfg(test)]
mod tuner_evaluation_tests {
  use zeno::eval_params::EVAL_PARAMS_DEFAULT;
  use zeno::evaluator::Evaluator;
  use zeno::position::Position;
  use zeno::tuner::position_ir::PositionIR;
  use zeno::tuner::tuner_evaluation::tuner_evaluation;
  use zeno::tuner::tuner_params::TunerParams;
  use zeno::utils::START_POSITION;

  #[test]
  fn test_position_ir_and_tuner_evaluation() {
    let fens = [
      START_POSITION,
      "r1b1r1k1/2q2p2/1pp3p1/p3b2p/2P2B2/P2B2P1/1PQ4P/R2R2K1 w - - 0 23",
      "4r3/1p6/4n1kp/p1PR2p1/P3N3/4K2P/2P2P2/8 w - - 2 42",
      "rnbqk2r/5ppp/p3pb2/1p1p4/PP1P4/2N1P3/5PPP/R2QKBNR b k - 0 10",
      "2b4k/1p5p/1q3n2/pP3pQ1/3P3R/P7/2P2PP1/2RK4 w - - 1 28",
      "5k1r/pp3pp1/5np1/nN6/1b3Nb1/8/PP3PPP/R1B1R1K1 w - - 6 22",
      "rn1q1rk1/1ppbbppp/p2pp3/3nP3/3P1B2/2N2N2/PPP1BPPP/R2QR1K1 w - - 1 11",
      "r1b1r1k1/1p3pp1/2pQ3p/8/p1B1PP2/6qP/PPP3P1/3RR2K w - - 2 25",
      "8/r7/6P1/5RKN/2k5/8/7P/8 w - - 5 62",
      "8/8/P7/1P2k3/3b4/3K1B2/8/8 w - - 30 111",
      "8/7p/p3pp2/6k1/3K4/r7/8/5R2 w - - 38 67",
      "1rb2b2/Rp3r2/PR1p2kp/3Pp3/N1P1P1p1/3B2P1/6K1/8 w - - 0 35",
      "8/2k5/5K2/3P4/p1N2n2/P7/8/8 w - - 0 54",
      "8/8/6p1/5kB1/p1p1p2P/P1P4b/2PK4/8 w - - 56 111",
      "8/8/2R5/P3kpp1/7p/6r1/8/2K5 w - - 0 52",
      "8/Q5p1/6kp/8/8/P3K1P1/2q5/8 b - - 8 39",
      "8/8/6k1/2p2p2/4r1n1/6P1/2KB4/8 w - - 1 57",
      "3rr1k1/pp1b1ppp/3b1n2/1P1pn3/Pq1B1N2/3B2QP/2P1NPP1/2KRR3 w - - 7 27",
      "3R4/8/2K5/4k3/8/8/8/7r b - - 18 64",
      "3rr1n1/1k6/p3p2p/4R1p1/6P1/2BP1R1P/1P4K1/8 w - - 0 30",
      "r1bqk2r/2p2pp1/p2p1n1p/1pbPp3/4PP1n/1BN3N1/PPP1Q1PP/R1B1K2R w k - 0 12",
      "r1b2rk1/pp1nppb1/2p2n1p/3pN3/3P2p1/qPNBP1B1/P1PQ1PPP/3R1RK1 w - - 6 13",
      "3rr1k1/2pq1p2/pp3n2/4p1pp/2PpP3/1P1PbPP1/1BB3K1/3RRQ2 w - - 0 36",
      "1k1rr3/pp1b2b1/2pp1pqn/3Pp2p/2P1P3/P4PPp/1PQNBB2/3RR1K1 w - - 1 22",
      "8/R4p2/5k2/2p4P/2NnK3/1P6/P7/7r w - - 5 47",
      "1B6/8/4p3/2k5/8/4K3/8/8 w - - 94 113",
      "r6r/1kp5/1p1bQ2p/3p4/3P4/P1q2P2/2N1RP2/4R1K1 b - - 0 36",
      "8/k7/2b1p3/2P1P3/p1p2p2/2B4P/5K2/R7 w - - 1 53",
      "R7/8/3pk3/4b1pN/2p3P1/1r6/5K2/8 w - - 0 44",
      "8/8/4K3/8/2R5/3k4/8/8 w - - 9 120",
      "rn1qk2r/pp3ppp/2pbpnb1/3p4/8/3PPP2/PPPNN1PP/R1BQKB1R w k - 6 8",
      "2r1r1k1/4n1b1/2p1p1pp/2QpP3/1P1P4/1NPq1N1P/6P1/2R1R1K1 w - - 1 37",
      "8/7B/r7/8/1k6/2p5/2K5/8 w - - 72 125",
      "r5k1/1Bp4p/5qp1/p4r2/1n1p4/1P1P4/bQPB2P1/R4RK1 b - - 0 27",
      "1k1rr3/ppp2pp1/2nb1nqp/3N4/2P1P3/4BP1P/PP2QP2/2KR1B1R w - - 0 15",
      "8/8/2p5/8/5R2/3k3P/6K1/4q3 w - - 4 70",
      "8/2k5/2pn4/2K5/8/8/8/1R6 w - - 22 72",
      "8/kp6/p1b1R1B1/8/6n1/P5P1/7r/3R2K1 w - - 0 39",
      "8/p4p2/n2qknp1/2p1p2r/1pP2P1p/P3Q3/1PN3PP/4RNK1 w - - 1 32",
      "1k2rr2/pbpp1p1R/np6/2bP1N2/3N2p1/P3P3/1PP1B1P1/2KR4 w - - 6 26",
      "r1bqr1k1/1pp2pbp/p1n2np1/3pp3/1P6/P1PPPN1P/1B2BPP1/RN1QK2R w K - 2 10",
      "3rr3/1pq2pk1/p1nbbnpp/4p3/P3P3/1PB1QNPP/3N1PBK/3RR3 w - - 0 25",
      "8/2p2k2/8/4K3/6P1/1p6/p4P2/7R w - - 0 61",
      "1k6/8/1PK3p1/8/8/1N1n4/8/8 w - - 4 54",
      "2r3k1/5pp1/p6p/4P3/2p5/P4q1P/2P5/4R2K w - - 0 39",
      "3r4/5pk1/p2qbnpp/3p4/3N2P1/P1N2Q1P/1KP2P2/3R4 b - - 22 42",
      "3r3r/1k1bnq2/pp3p2/3p3p/5P1N/PPB4P/2Q3P1/1K1RR3 w - - 0 31",
      "2kr1bnr/pp3pp1/2n1p1b1/q1ppP1Pp/7P/3P1N1B/PPPN1P2/R1BQ1RK1 w - - 6 11",
      "8/8/5k2/2p1n3/2PpPq2/8/2BR1P2/2K5 w - - 1 60",
      "2B5/p7/5p2/P1b1k3/1p5P/2PK4/8/8 w - - 0 44",
      "1kb4r/2q2p2/3p1bp1/2pPp1N1/1pP1Pn2/1P2NPQ1/8/3RR1K1 w - - 5 38",
      "6k1/8/5Q2/8/4P3/2P3P1/PPK5/8 w - - 1 48",
      "r4k1r/2p3pp/p4n2/4q3/P3b3/1BN1P2P/1P3PP1/Q5KR w - - 0 25",
      "8/pkp5/2p4p/1P5b/8/P3q1p1/1P1N4/3K1B2 w - - 1 47",
      "8/8/R7/1pr5/p4P2/4K2k/8/8 w - - 18 74",
      "8/8/8/8/4k3/6p1/8/5K2 w - - 1 79",
      "3b4/5k2/4R3/4K3/8/8/8/8 w - - 52 92",
      "8/5pkp/6p1/PB1r1b2/8/2P1K3/8/1R6 w - - 11 47",
      "2N5/1p6/p3pk2/P1b4r/5p1P/2P2r2/1P5P/R6K w - - 1 43",
      "1k1r3r/pbp2pb1/1n2qnp1/1BB4p/4P3/5N2/P1QN1PPP/1R2R1K1 w - - 6 22",
      "8/8/p1K2p2/1p6/1P3Pkp/2P5/3n4/6N1 w - - 2 47",
      "2r1r1k1/5pp1/5n2/3p1P1p/pPqPp3/P1PnR1PP/1BQ5/5RK1 w - - 10 41",
      "8/6b1/4k1P1/8/4K3/8/8/8 w - - 21 81",
      "b1r5/1q4pk/2nr3p/1QRNNp1P/3P4/4P3/5P2/2R3K1 w - - 1 43",
      "3r2k1/5pp1/p2p3p/1pqP4/4RP2/PQP4P/5PK1/8 w - - 0 30",
      "7r/7P/3p3R/p5k1/2P5/1K4P1/1P6/8 w - - 1 45",
      "r3r1k1/pp2q1pp/2p5/3pP3/5BQn/4P3/P1P2P2/1R3RK1 w - - 2 23",
      "5k2/1pp2p2/p7/5b2/1P1Pr1p1/1P6/1BP2K1P/2R5 w - - 2 26",
      "8/8/8/r7/1N5p/2k5/6PK/5r2 w - - 2 76",
    ];

    for fen in fens {
      let position = Position::from_fen(fen);
      if position.get_phase() > 200 {
        continue;
      }
      let position_ir = PositionIR::from_position(&position);
      let tuner_params = TunerParams::from_eval_param(&EVAL_PARAMS_DEFAULT);
      assert!((Evaluator::evaluate(&position, &EVAL_PARAMS_DEFAULT) * position.get_side().to_i32() - tuner_evaluation(&position_ir, &tuner_params) as i32).abs() <= 1);
    }
  }
}
