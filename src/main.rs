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

mod evaluation;
mod lookup_tables;
mod moves_generator;
mod position;
mod search;
mod uci;
mod utils;
mod zobrist_hash;

fn main() {
    let mut board =
        Position::from_fen("rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    board.print_board();

    board.make_move(&Move {
        source: Coord { rank: 1, file: 'e' },
        destination: Coord { rank: 1, file: 'g' },
        move_type: MoveType::ShortCastle,
    });
    board.print_board();

    // PLAY

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

    // uci::uci_loop();
}
