// https://www.chessprogramming.org
// https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
// https://dev.to/larswaechter/zobrist-hashing-72n
// https://www.cs.cmu.edu/afs/cs/academic/class/15418-s12/www/competition/www.contrib.andrew.cmu.edu/~jvirdo/rasmussen-2004.pdf
// https://www.josherv.in/2021/03/19/chess-1/
// https://jdhwilkins.com/python-chess-efficient-move-generation-using-bitwise-operations/
// https://raytran.net/projects/protochess
// https://lichess.org/@/likeawizard/blog/review-of-different-board-representations-in-computer-chess/S9eQCAWa
// https://github.com/jhonnold/berserk
// Use x & (x - 1) to clear the least significant bit: this is faster than x &= ~(1 << from)


// TODO: Verify in the is_check() that the move doesn't live the king in check
// TODO: Undo function
// TODO: CHECKMATE TO AVOID KING BEING CAPTURE

use Zeno::moves_generator::{generate_legal_moves};
use Zeno::position::Position;
use Zeno::utils::{Move, MoveType};
use std::time::Instant;
use thousands::Separable;

fn perft(depth: i32, position:  &Position) -> u64 {
    let mut number_of_move: u64 = 0;
    if depth == 0 {
        number_of_move = 1;
    } else {
        let turn = position.get_turn();
        let moves: Vec<Move> = generate_legal_moves(position, &turn);
        for m in moves {
            let mut temp_position = position.clone();
            temp_position.make_move(&m, true);
            number_of_move += perft(depth - 1, &temp_position);
            // println!("{:?}", m);
            // temp_position.print_board();
        }
    }
    number_of_move
}

fn main() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    Position::from_fen(fen).print_board();

    let mut start = Instant::now();
    print!("Pertf(1) = {} in ", perft(1, &mut Position::from_fen(fen)).separate_with_commas());
    let mut duration = start.elapsed();
    println!("{:?}s", duration.as_secs());
    start = Instant::now();
    print!("Pertf(2) = {} in ", perft(2, &mut Position::from_fen(fen)).separate_with_commas());
    duration = start.elapsed();
    println!("{:?}s", duration.as_secs());
    start = Instant::now();
    print!("Pertf(3) = {} in ", perft(3, &mut Position::from_fen(fen)).separate_with_commas());
    duration = start.elapsed();
    println!("{:?}s", duration.as_secs());
    start = Instant::now();
    print!("Pertf(4) = {} in ", perft(4, &mut Position::from_fen(fen)).separate_with_commas());
    duration = start.elapsed();
    println!("{:?}s", duration.as_secs());
    start = Instant::now();
    print!("Pertf(5) = {} in ", perft(5, &mut Position::from_fen(fen)).separate_with_commas());
    duration = start.elapsed();
    println!("{:?}s", duration.as_secs());
    // print!("Pertf(6) = {} in ", perft(6, &mut Position::from_fen(fen)).separate_with_commas());
    // duration = start.elapsed();
    // println!("{:?}s", duration.as_secs());
    // print!("Pertf(7) = {} in ", perft(7, &Position::from_fen(fen)).separate_with_commas());
    // duration = start.elapsed();
    // println!("{:?}s", duration.as_secs());

    // Zeno::uci::uci_loop();

    // let mut board =
    //     Position::from_fen("rnbqkbnr/ppppp1pp/8/8/5Pp1/8/PPPPP1PP/RNBQKBNR b KQkq f3 0 1");
    //
    // board.print_board();
    //
    // board.make_move(
    //     &Move {
    //         source: 30,
    //         destination: 21,
    //         move_type: MoveType::EnPassant,
    //     },
    //     false,
    // );
    // board.print_board();
    // board.make_move(
    //     &Move {
    //         source: 12,
    //         destination: 28,
    //         move_type: MoveType::EnPassant,
    //     },
    //     false,
    // );
    // board.print_board();
    // println!("{:?}", board.get_en_passant());
    // board.make_move(
    //     &Move {
    //         source: 21,
    //         destination: 13,
    //         move_type: MoveType::EnPassant,
    //     },
    //     false,
    // );
    // board.print_board();
    // println!("{:?}", board.get_en_passant());
    // board.make_move(
    //     &Move {
    //         source: 6,
    //         destination: 23,
    //         move_type: MoveType::EnPassant,
    //     },
    //     false,
    // );
    // board.print_board();
    //
    // for _ in 0..15 {
    //     let mov = search::best_move(&board);
    //     let move_type_character: char;
    //
    //     match mov.move_type {
    //         MoveType::Normal => move_type_character = ' ',
    //         MoveType::ShortCastle => move_type_character = 'n',
    //         MoveType::LongCastle => move_type_character = 'n',
    //         MoveType::PawnToKnight => move_type_character = 'n',
    //         MoveType::PawnToBishop => move_type_character = 'b',
    //         MoveType::PawnToRook => move_type_character = 'r',
    //         MoveType::PawnToQueen => move_type_character = 'q',
    //     }
    //
    //     println!(
    //         "bestmove {}{}{}{}{}",
    //         mov.source.file,
    //         mov.source.rank,
    //         mov.destination.file,
    //         mov.destination.rank,
    //         move_type_character
    //     );
    //
    //     board.make_move(&mov, false);
    //     board.print_board();
    // }
    //
    // // PLAY
    //
    // // board.print_board();
    // // let mut board = board.get_board() >> ('d' as u8 - 'a' as u8);
    // //
    // // println!("{:b}", board);
    // //
    // // let mut sq: u64 = 0;
    // // for i in 0..=7u64 {
    // //     sq |= ((board >> (8 * i)) & 1) << i;
    // // }
    // // println!("{:b}", sq);
    // // sq &= !(1 << ('d' as u8 - 'a' as u8));
    // // println!("{:b}", sq);
    // // println!("{:b}", lookup_tables::ROOK_LOOK_UP_TABLE[(8 - 4) as usize].get(&sq).unwrap());
    // //
    // // sq = 0b110111;
    // // board = 0;
    // // for i in 0..=7u64 {
    // //     board |= ((sq >> i) & 1) << (8 * i);
    // // }
    // // println!("{:b}", board << ('d' as u8 - 'a' as u8));
    //
    // return;
}
