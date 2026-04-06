use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use crate::eval_params::EvalParams;
use crate::piece::{PieceColor, PieceType};
use crate::square::Square;
use crate::tuner::features::{FEATURES_ALL, Features, MAX_FEATURES};
use crate::utils::get_psqt_index;

#[derive(Clone, Debug)]
pub struct TunerParams {
  pub mg: [f32; MAX_FEATURES],
  pub eg: [f32; MAX_FEATURES],
}

impl TunerParams {
  #[inline(always)]
  pub fn from_eval_param(eval_params: &EvalParams) -> Self {
    let mut params_f32 = Self::fill_with(0.0);

    for feature in FEATURES_ALL {
      let index = feature.to_index();
      match feature {
        Features::Material(piece_type) => {
          params_f32.mg[index] = eval_params.get_mg_piece_value(piece_type) as f32;
          params_f32.eg[index] = eval_params.get_eg_piece_value(piece_type) as f32;
        }
        Features::Psqt(piece_type, square) => {
          params_f32.mg[index] = eval_params.get_mg_psqt_value(piece_type, PieceColor::White, square) as f32;
          params_f32.eg[index] = eval_params.get_eg_psqt_value(piece_type, PieceColor::White, square) as f32;
        }
      };
    }

    params_f32
  }

  #[inline(always)]
  pub const fn fill_with(value: f32) -> Self {
    TunerParams { mg: [value; MAX_FEATURES], eg: [value; MAX_FEATURES] }
  }

  pub fn save_to_file(&self, file_name: &str, header_comments: &str) -> Result<(), io::Error> {
    let mut string_buffer = String::with_capacity(8192);

    string_buffer.push_str(header_comments);
    string_buffer.push_str("\n");
    string_buffer.push_str("use crate::containers::ByPieceType;\n");

    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      // Middlegame
      println!();
      string_buffer.push_str("\n");

      println!("{piece_type:?} : MG {} - EG {}", self.mg[Features::Material(piece_type).to_index()] as i32, self.eg[Features::Material(piece_type).to_index()] as i32);

      string_buffer.push_str("static MG_");
      string_buffer.push_str(&format!("{:?}", piece_type).to_uppercase());
      string_buffer.push_str("_VALUE: i32 = ");
      string_buffer.push_str(&(self.mg[Features::Material(piece_type).to_index()] as i32).to_string());
      string_buffer.push_str(";\n");

      println!("MG PSQT");
      string_buffer.push_str("#[rustfmt::skip]\n");
      string_buffer.push_str("static MG_");
      string_buffer.push_str(&format!("{:?}", piece_type).to_uppercase());
      string_buffer.push_str("_TABLE: [i32; 64] = [");
      string_buffer.push_str("\n");
      for rank in 0..=7 {
        print!("{} ", 8 - rank);
        for file in 0..=7 {
          let index = rank * 8 + file;
          print!("{:4} ", self.mg[Features::Psqt(piece_type, get_psqt_index(PieceColor::White, index) as Square).to_index()] as i32);
          string_buffer.push_str(&format!("{:4}, ", self.mg[Features::Psqt(piece_type, get_psqt_index(PieceColor::White, index) as Square).to_index()] as i32).to_uppercase());
        }
        println!();
        string_buffer.push_str("\n");
      }
      for i in [' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
        print!("{:+04} ", i);
      }
      println!();
      string_buffer.push_str("];\n");

      // Endgame
      println!();
      string_buffer.push_str("\n");

      string_buffer.push_str("static EG_");
      string_buffer.push_str(&format!("{:?}", piece_type).to_uppercase());
      string_buffer.push_str("_VALUE: i32 = ");
      string_buffer.push_str(&(self.eg[Features::Material(piece_type).to_index()] as i32).to_string());
      string_buffer.push_str(";\n");

      println!("EG PSQT");
      string_buffer.push_str("#[rustfmt::skip]\n");
      string_buffer.push_str("static EG_");
      string_buffer.push_str(&format!("{:?}", piece_type).to_uppercase());
      string_buffer.push_str("_TABLE: [i32; 64] = [");
      string_buffer.push_str("\n");
      for rank in 0..=7 {
        print!("{} ", 8 - rank);
        for file in 0..=7 {
          let index = rank * 8 + file;
          print!("{:4} ", self.eg[Features::Psqt(piece_type, get_psqt_index(PieceColor::White, index) as Square).to_index()] as i32);
          string_buffer.push_str(&format!("{:4}, ", self.eg[Features::Psqt(piece_type, get_psqt_index(PieceColor::White, index) as Square).to_index()] as i32).to_uppercase());
        }
        println!();
        string_buffer.push_str("\n");
      }
      for i in [' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
        print!("{:+04} ", i);
      }
      println!();
      string_buffer.push_str("];\n");
    }

    string_buffer.push_str("\npub static MG_PIECES_VALUES: ByPieceType<i32> = ByPieceType::new(MG_PAWN_VALUE, MG_KNIGHT_VALUE, MG_BISHOP_VALUE, MG_ROOK_VALUE, MG_QUEEN_VALUE, MG_KING_VALUE);");
    string_buffer.push_str("\npub static EG_PIECES_VALUES: ByPieceType<i32> = ByPieceType::new(EG_PAWN_VALUE, EG_KNIGHT_VALUE, EG_BISHOP_VALUE, EG_ROOK_VALUE, EG_QUEEN_VALUE, EG_KING_VALUE);");
    string_buffer
      .push_str("\npub static MG_PIECES_SQUARES_TABLES: ByPieceType<[i32; 64]> = ByPieceType::new(MG_PAWN_TABLE, MG_KNIGHT_TABLE, MG_BISHOP_TABLE, MG_ROOK_TABLE, MG_QUEEN_TABLE, MG_KING_TABLE);");
    string_buffer
      .push_str("\npub static EG_PIECES_SQUARES_TABLES: ByPieceType<[i32; 64]> = ByPieceType::new(EG_PAWN_TABLE, EG_KNIGHT_TABLE, EG_BISHOP_TABLE, EG_ROOK_TABLE, EG_QUEEN_TABLE, EG_KING_TABLE);");

    let mut writer = BufWriter::new(File::create(file_name)?);
    write!(writer, "{}", string_buffer)?;

    Ok(())
  }
}
