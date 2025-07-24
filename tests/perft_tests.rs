use Zeno::moves_generator::generate_moves;
use Zeno::position::Position;

fn perft(depth: i32, position: &Position) -> u32 {
    let mut number_of_move = 0;
    if depth == 0 {
        number_of_move = 1;
    } else {
    let moves = generate_moves(position, &position.get_turn());
        for m in moves {
            let mut temp_position = position.clone();
            temp_position.make_move(&m, true);
            if !temp_position.is_check(&temp_position.get_turn()){
                number_of_move += perft(depth - 1, &temp_position);
            }
        }
    }

    number_of_move
}
#[test]
fn perft_1() {
    let result = perft(
        1,
        &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 20);
}
#[test]
fn perft_2() {
    let result = perft(
        2,
        &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 400);
}
#[test]
fn perft_3() {
    let result = perft(
        3,
        &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 8_902);
}

#[test]
fn perft_4() {
    let result = perft(
        4,
        &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    );
    assert_eq!(result, 197_281);
}

// #[test]
// fn perft_5() {
//     let result = perft(
//         5,
//         &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
//     );
//     assert_eq!(result, 4_865_609);
// }
// #[test]
// fn perft_6() {
//     let result = perft(
//         6,
//         &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
//     );
//     assert_eq!(result, 119_060_324);
// }
