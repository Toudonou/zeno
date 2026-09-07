use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use crate::eval_params::EvalParams;
use crate::piece::{PieceColor, PieceType};
use crate::square::Square;
use crate::tuner::features::{FEATURES_ALL, Features, MAX_FEATURES};
use crate::utils::get_psqt_index;

macro_rules! format_simple_features {
  ($buffer:expr, $variable_name:expr, $feature:expr, $source:expr) => {
    $buffer.push_str("\npub static ");
    $buffer.push_str($variable_name);
    $buffer.push_str(": i32 = ");
    $buffer.push_str(&($source[$feature.to_index()] as i32).to_string());
    $buffer.push_str(";")
  };
}

macro_rules! format_n_features {
  ($buffer:expr, $variable_name:expr, $feature:expr, $source:expr, $size:expr) => {
    $buffer.push_str("\npub static ");
    $buffer.push_str($variable_name);
    $buffer.push_str(": [i32; ");
    $buffer.push_str(&($size.to_string()));
    $buffer.push_str("] = [");
    for index in 0..($size - 1) {
      $buffer.push_str(&format!("{}, ", $source[$feature(index as u8).to_index()] as i32).to_uppercase());
    }
    $buffer.push_str(&format!("{}", $source[$feature($size - 1 as u8).to_index()] as i32).to_uppercase());
    $buffer.push_str("];");
  };
}

macro_rules! format_psqt_features {
  ($buffer:expr, $variable_name:expr, $piece_type:expr, $source:expr) => {
    $buffer.push_str("\n#[rustfmt::skip]\n");
    $buffer.push_str("pub static ");
    $buffer.push_str($variable_name);
    $buffer.push_str(": [i32; 64] = [");
    $buffer.push_str("\n");
    for rank in 0..=7 {
      for file in 0..=7 {
        let index = rank * 8 + file;
        $buffer.push_str(&format!("{:4}, ", $source[Features::Psqt($piece_type, get_psqt_index(PieceColor::White, index) as Square).to_index()] as i32).to_uppercase());
      }
      $buffer.push_str("\n");
    }
    $buffer.push_str("];");
  };
}

#[derive(Clone, Debug)]
pub struct TunerParams {
  pub mg: Vec<f32>,
  pub eg: Vec<f32>,
}

impl TunerParams {
  #[inline(always)]
  pub fn from_eval_param(eval_params: &EvalParams) -> Self {
    let mut params_f32 = Self::from_value(0.0);

    for &feature in FEATURES_ALL.iter() {
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
        Features::BishopPair => {
          params_f32.mg[index] = eval_params.get_mg_bishop_pair_value() as f32;
          params_f32.eg[index] = eval_params.get_eg_bishop_pair_value() as f32;
        }
        Features::DoubledPawns(file) => {
          params_f32.mg[index] = eval_params.get_mg_doubled_pawns_value(file) as f32;
          params_f32.eg[index] = eval_params.get_eg_doubled_pawns_value(file) as f32;
        }
        Features::PassedPawns(rank) => {
          params_f32.mg[index] = eval_params.get_mg_passed_pawns_value(rank, PieceColor::White) as f32;
          params_f32.eg[index] = eval_params.get_eg_passed_pawns_value(rank, PieceColor::White) as f32;
        }
        Features::IsolatedPawns(file) => {
          params_f32.mg[index] = eval_params.get_mg_isolated_pawns_value(file) as f32;
          params_f32.eg[index] = eval_params.get_eg_isolated_pawns_value(file) as f32;
        }
        Features::BackwardPawns(file) => {
          params_f32.mg[index] = eval_params.get_mg_backward_pawns_value(file) as f32;
          params_f32.eg[index] = eval_params.get_eg_backward_pawns_value(file) as f32;
        }
        Features::ConnectedPawns(file) => {
          params_f32.mg[index] = eval_params.get_mg_connected_pawns_value(file) as f32;
          params_f32.eg[index] = eval_params.get_eg_connected_pawns_value(file) as f32;
        }
      };
    }

    params_f32
  }

  #[inline(always)]
  pub fn from_value(value: f32) -> Self {
    TunerParams { mg: vec![value; MAX_FEATURES], eg: vec![value; MAX_FEATURES] }
  }

  #[inline(always)]
  pub fn fill_with(&mut self, value: f32) {
    self.mg.fill(value);
    self.eg.fill(value);
  }

  pub fn save_to_file(&mut self, file_name: &str, header_comments: &str) -> Result<(), io::Error> {
    let mut string_buffer = String::with_capacity(8192);

    string_buffer.push_str(&header_comments);
    string_buffer.push_str("\n");
    string_buffer.push_str("use crate::containers::ByPieceType;\n");

    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      format_simple_features!(string_buffer, &("MG_".to_owned() + &format!("{:?}", piece_type).to_uppercase() + "_VALUE"), Features::Material(piece_type), self.mg);
      format_simple_features!(string_buffer, &("EG_".to_owned() + &format!("{:?}", piece_type).to_uppercase() + "_VALUE"), Features::Material(piece_type), self.eg);
      string_buffer.push_str("\n");
    }

    format_simple_features!(string_buffer, "MG_BISHOP_PAIR", Features::BishopPair, self.mg);
    format_simple_features!(string_buffer, "EG_BISHOP_PAIR", Features::BishopPair, self.eg);
    string_buffer.push_str("\n");

    format_n_features!(string_buffer, "MG_DOUBLED_PAWNS", Features::DoubledPawns, self.mg, 8);
    format_n_features!(string_buffer, "EG_DOUBLED_PAWNS", Features::DoubledPawns, self.eg, 8);
    string_buffer.push_str("\n");

    format_n_features!(string_buffer, "MG_PASSED_PAWNS", Features::PassedPawns, self.mg, 8);
    format_n_features!(string_buffer, "EG_PASSED_PAWNS", Features::PassedPawns, self.eg, 8);
    string_buffer.push_str("\n");

    format_n_features!(string_buffer, "MG_ISOLATED_PAWNS", Features::IsolatedPawns, self.mg, 8);
    format_n_features!(string_buffer, "EG_ISOLATED_PAWNS", Features::IsolatedPawns, self.eg, 8);
    string_buffer.push_str("\n");

    format_n_features!(string_buffer, "MG_BACKWARD_PAWNS", Features::BackwardPawns, self.mg, 8);
    format_n_features!(string_buffer, "EG_BACKWARD_PAWNS", Features::BackwardPawns, self.eg, 8);
    string_buffer.push_str("\n");

    format_n_features!(string_buffer, "MG_CONNECTED_PAWNS", Features::ConnectedPawns, self.mg, 8);
    format_n_features!(string_buffer, "EG_CONNECTED_PAWNS", Features::ConnectedPawns, self.eg, 8);
    string_buffer.push_str("\n");

    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      format_psqt_features!(string_buffer, &("MG_".to_owned() + &format!("{:?}", piece_type).to_uppercase() + "_TABLE"), piece_type, self.mg);
      format_psqt_features!(string_buffer, &("EG_".to_owned() + &format!("{:?}", piece_type).to_uppercase() + "_TABLE"), piece_type, self.eg);
      string_buffer.push_str("\n");
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
