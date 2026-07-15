use crate::evaluator::Evaluator;
use crate::piece::PieceColor;
use crate::position::Position;
use crate::tuner::features::{FEATURES_ALL, Features};

/// Position Intermediate Representation
#[derive(Clone, Debug)]
pub struct PositionIR {
  pub board: Vec<(usize, i8)>,
  pub side: PieceColor,
  pub mg_factor: f32,
  pub eg_factor: f32,
}

impl PositionIR {
  #[inline(always)]
  pub fn from_position(position: &Position) -> PositionIR {
    let phase = position.evaluate_phase();
    Self {
      board: {
        let mut position_features = Vec::with_capacity(64);
        for &feature in FEATURES_ALL.iter() {
          let index = feature.to_index();
          let feature = match feature {
            Features::Material(piece_type) => {
              position.get_by_side_and_type(PieceColor::White, piece_type).count_ones() as i8 - position.get_by_side_and_type(PieceColor::Black, piece_type).count_ones() as i8
            }
            Features::Psqt(piece_type, square) => {
              let piece_white_pov = position.get_piece_on_square(square);
              let piece_black_pov = position.get_piece_on_square(square ^ 56);

              i8::from(piece_white_pov.piece_type == piece_type && piece_white_pov.color == PieceColor::White)
                - i8::from(piece_black_pov.piece_type == piece_type && piece_black_pov.color == PieceColor::Black)
            }
            Features::BishopPair => i8::from(Evaluator::has_bishop_pair(position, PieceColor::White)) - i8::from(Evaluator::has_bishop_pair(position, PieceColor::Black)),
          };

          if feature != 0 {
            position_features.push((index, feature));
          }
        }
        position_features
      },
      mg_factor: (256f32 - phase as f32) / 256f32,
      eg_factor: phase as f32 / 256f32,
      side: position.get_side(),
    }
  }
}
