use regex::Regex;
use std::fmt::{Display, Formatter};

use crate::piece::{Piece, PieceColor};
use crate::position::Position;
use crate::square::SquareOps;
use crate::{piece::PieceType, square::Square};

pub static MOVE_LIST_MAX_SIZE: usize = 252;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MoveType {
  Normal = 0,
  ShortCastle = 1,
  LongCastle = 2,
  PawnToKnight = 3,
  PawnToBishop = 4,
  PawnToRook = 5,
  PawnToQueen = 6,
  EnPassant = 7,
}

impl From<u16> for MoveType {
  #[inline(always)]
  fn from(value: u16) -> Self {
    match value {
      0 => MoveType::Normal,
      1 => MoveType::ShortCastle,
      2 => MoveType::LongCastle,
      3 => MoveType::PawnToKnight,
      4 => MoveType::PawnToBishop,
      5 => MoveType::PawnToRook,
      6 => MoveType::PawnToQueen,
      7 => MoveType::EnPassant,
      _ => panic!("Invalid MoveType value: {}", value),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move(u16);

impl Move {
  /// Pack source, destination, and move_type into a 16-bit value:
  /// [ move_type:3 | destination:6 | source:6 ]
  #[inline(always)]
  pub fn new(source: Square, destination: Square, move_type: MoveType) -> Self {
    Self(((move_type as u16) << 12) | ((destination as u16) << 6) | (source as u16))
  }

  #[inline(always)]
  pub fn to_u16(self) -> u16 {
    self.0
  }

  #[inline(always)]
  pub fn from_u16(value: u16) -> Self {
    Move(value)
  }

  #[inline(always)]
  pub fn from_uci_notation(move_string: &str, position: &Position) -> Option<Move> {
    let reg = Regex::new(r"^[a-h][1-8][a-h][1-8][nbrq]?$").unwrap();
    if !reg.is_match(move_string) {
      println!("Incorrect uci move notation: {}", move_string);
      return None;
    }

    let part: Vec<char> = move_string.chars().collect();
    let source_rank = part[1].to_digit(10).unwrap() - 1;
    let source_file = part[0];
    let destination_rank = part[3].to_digit(10).unwrap() - 1;
    let destination_file = part[2];
    let mut move_type = MoveType::Normal;

    if move_string == "e1g1" && (position.get_piece_on_square(Square::from_algebric_notation("e1")) == Piece { color: PieceColor::White, piece_type: PieceType::King }) {
      move_type = MoveType::ShortCastle;
    } else if move_string == "e1c1" && (position.get_piece_on_square(Square::from_algebric_notation("e1")) == Piece { color: PieceColor::White, piece_type: PieceType::King }) {
      move_type = MoveType::LongCastle;
    } else if move_string == "e8g8" && (position.get_piece_on_square(Square::from_algebric_notation("e8")) == Piece { color: PieceColor::Black, piece_type: PieceType::King }) {
      move_type = MoveType::ShortCastle;
    } else if move_string == "e8c8" && (position.get_piece_on_square(Square::from_algebric_notation("e8")) == Piece { color: PieceColor::Black, piece_type: PieceType::King }) {
      move_type = MoveType::LongCastle;
    } else if part.len() == 5 {
      match part[4] {
        'n' => move_type = MoveType::PawnToKnight,
        'b' => move_type = MoveType::PawnToBishop,
        'r' => move_type = MoveType::PawnToRook,
        'q' => move_type = MoveType::PawnToQueen,
        _ => {}
      }
    } else if (8 * destination_rank as u8 + destination_file as u8 - 'a' as u8) == position.get_en_passant()
      && position.get_piece_on_square(8 * source_rank as u8 + source_file as u8 - 'a' as u8).piece_type == PieceType::Pawn
    {
      move_type = MoveType::EnPassant;
    }

    let source = (source_rank * 8) as u8 + source_file as u8 - 'a' as u8;
    let destination = (destination_rank * 8) as u8 + destination_file as u8 - 'a' as u8;

    Some(Move::new(source, destination, move_type))
  }

  #[inline(always)]
  pub fn source(self) -> Square {
    (self.0 & 0b111111) as Square
  }

  #[inline(always)]
  pub fn destination(self) -> Square {
    ((self.0 >> 6) & 0b111111) as Square
  }

  #[inline(always)]
  pub fn move_type(self) -> MoveType {
    u16::into((self.0 >> 12) & 0b111)
  }

  #[inline(always)]
  pub fn is_promotion(self) -> bool {
    match self.move_type() {
      MoveType::PawnToKnight | MoveType::PawnToBishop | MoveType::PawnToRook | MoveType::PawnToQueen => true,
      _ => false,
    }
  }

  #[inline(always)]
  pub fn to_uci_string(self) -> String {
    let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let ranks = [1, 2, 3, 4, 5, 6, 7, 8];

    files[self.source().get_file() as usize].to_string()
      + &ranks[self.source().get_rank() as usize].to_string()
      + &files[self.destination().get_file() as usize].to_string()
      + &ranks[self.destination().get_rank() as usize].to_string()
      + match self.move_type() {
        MoveType::PawnToKnight => "n",
        MoveType::PawnToBishop => "b",
        MoveType::PawnToRook => "r",
        MoveType::PawnToQueen => "q",
        _ => "",
      }
  }
}

impl Default for Move {
  #[inline(always)]
  fn default() -> Self {
    Self::new(0, 0, MoveType::Normal)
  }
}

impl Display for Move {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_uci_string())
  }
}

pub struct MoveList {
  pub moves: [Move; MOVE_LIST_MAX_SIZE], // For padding; Total size 512
  pub count: usize,
}

impl MoveList {
  #[inline(always)]
  pub fn new() -> Self {
    MoveList { moves: [Move::default(); MOVE_LIST_MAX_SIZE], count: 0 }
  }

  #[inline(always)]
  pub fn push(&mut self, m: Move) {
    self.moves[self.count] = m;
    self.count += 1;
  }
}
