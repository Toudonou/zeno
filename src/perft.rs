use crate::moves_generator::generate_pseudo_legal_moves;
use crate::position::Position;
use std::time::Instant;

pub fn perft(depth: i32, position: &Position) -> u64 {
    let mut number_of_move: u64 = 0;
    if depth == 0 {
        number_of_move = 1;
    } else {
        let turn = position.get_turn();
        let moves = generate_pseudo_legal_moves(position, &turn);
        for mov in &moves {
            match mov {
                None => break,
                Some(m) => {
                    let mut temp_position = position.clone();
                    temp_position.make_move(&m, true);
                    if !temp_position.is_check(&turn) {
                        number_of_move += perft(depth - 1, &temp_position);
                    }
                }
            }
        }
    }
    number_of_move
}

pub fn pertf_divide(depth: i32, position: &Position) {
    let turn = position.get_turn();
    let moves = generate_pseudo_legal_moves(position, &turn);
    for mov in moves {
        match mov {
            None => break,
            Some(m) => {
                let mut temp_position = position.clone();
                temp_position.make_move(&m, true);
                let start = Instant::now();
                let number_of_move = perft(depth - 1, &temp_position);
                let duration = start.elapsed();
                println!(
                    "Move : {}{}{}{} => {} in {:?}",
                    ('a' as u8 + (m.source % 8) as u8) as char,
                    1 + m.source / 8,
                    ('a' as u8 + (m.destination % 8) as u8) as char,
                    1 + m.destination / 8,
                    number_of_move,
                    duration
                );
            }
        }
    }
}
