use zeno::perft::perft;
use zeno::position::Position;

#[test]
fn perft_startpos_depth_1() {
    let result = perft(
        1,
        &mut Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 20);
}
#[test]
fn perft_startpos_depth_2() {
    let result = perft(
        2,
        &mut Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 400);
}
#[test]
fn perft_startpos_depth_3() {
    let result = perft(
        3,
        &mut Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 8_902);
}

#[test]
fn perft_startpos_depth_4() {
    let result = perft(
        4,
        &mut Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 197_281);
}

#[test]
fn perft_startpos_depth_5() {
    let result = perft(
        5,
        &mut Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 4_865_609);
}
#[test]
fn perft_startpos_depth_6() {
    let result = perft(
        6,
        &mut Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 119_060_324);
}

#[test]
fn perft_kiwipete_depth_1() {
    let result = perft(
        1,
        &mut Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        ),
    );
    assert_eq!(result, 48);
}
#[test]
fn perft_kiwipete_depth_2() {
    let result = perft(
        2,
        &mut Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        ),
    );
    assert_eq!(result, 2039);
}
#[test]
fn perft_kiwipete_depth_3() {
    let result = perft(
        3,
        &mut Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        ),
    );
    assert_eq!(result, 97_862);
}

#[test]
fn perft_kiwipete_depth_4() {
    let result = perft(
        4,
        &mut Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        ),
    );
    assert_eq!(result, 4_085_603);
}

#[test]
fn perft_kiwipete_depth_5() {
    let result = perft(
        5,
        &mut Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        ),
    );
    assert_eq!(result, 193_690_690);
}
#[test]
fn perft_kiwipete_depth_6() {
    let result = perft(
        6,
        &mut Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        ),
    );
    assert_eq!(result, 8_031_647_685);
}
