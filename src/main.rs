// https://www.chessprogramming.org
// https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
// https://dev.to/larswaechter/zobrist-hashing-72n
// https://www.cs.cmu.edu/afs/cs/academic/class/15418-s12/www/competition/www.contrib.andrew.cmu.edu/~jvirdo/rasmussen-2004.pdf
// https://www.josherv.in/2021/03/19/chess-1/
// https://jdhwilkins.com/python-chess-efficient-move-generation-using-bitwise-operations/
// https://raytran.net/projects/protochess
// https://lichess.org/@/likeawizard/blog/review-of-different-board-representations-in-computer-chess/S9eQCAWa

use crate::position::Position;
use crate::uci::uci_make_move;
use crate::utils::{Coord, Move, MoveType};
use std::io;

mod attacks_table;
mod evaluation;
mod lookup_tables;
mod magic_bitboard_generator;
mod moves_generator;
mod position;
mod search;
mod uci;
mod utils;
mod zobrist_hash;

fn main() {
    let mut board = Position::from_fen("rkbnrbnq/pppppppp/8/8/8/8/PPPPPPPP/RKBNRBNQ w - - 0 1");

    board.print_board();
    board.make_move(&Move {
        source: Coord { rank: 1, file: 'd' },
        destination: Coord { rank: 3, file: 'c' },
        move_type: MoveType::Normal,
    });
    board.print_board();

    // board.print_board();
    // let mut board = board.get_board() >> ('d' as u8 - 'a' as u8);
    //
    // println!("{:b}", board);
    //
    // let mut sq: u64 = 0;
    // for i in 0..=7u64 {
    //     sq |= ((board >> (8 * i)) & 1) << i;
    // }
    // println!("{:b}", sq);
    // sq &= !(1 << ('d' as u8 - 'a' as u8));
    // println!("{:b}", sq);
    // println!("{:b}", lookup_tables::ROOK_LOOK_UP_TABLE[(8 - 4) as usize].get(&sq).unwrap());
    //
    // sq = 0b110111;
    // board = 0;
    // for i in 0..=7u64 {
    //     board |= ((sq >> i) & 1) << (8 * i);
    // }
    // println!("{:b}", board << ('d' as u8 - 'a' as u8));

    return;

    // while i != 0 {
    //     let des = i.trailing_zeros();
    //     println!("{}", des);
    //     i = i & !(1 << des);
    //     println!("{:b}", i);
    // }
    //
    // let mut position = Position::from_fen("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1");
    // position.print_board();
    //
    // loop {
    //     let mut command = String::new();
    //     io::stdin().read_line(&mut command).unwrap();
    //     let command = command.trim();
    //     uci_make_move(command, &mut position);
    //     position.print_board();
    // }
    //
    // return;
    // uci::uci_loop();
}
