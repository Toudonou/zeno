use crate::tuner::features::{Features, FEATURES_ALL, MAX_FEATURES};
use crate::piece::PieceColor;
use crate::position::Position;

/// Position Intermediary Representation
#[derive(Clone, Debug)]
pub struct PositionIR {
  pub board: [i8; MAX_FEATURES],
  pub side: PieceColor,
  pub mg_factor: f32,
  pub eg_factor: f32,
}

impl PositionIR {
  #[inline(always)]
  pub fn from_position(position: &Position) -> PositionIR {
    Self {
      board: {
        let mut position_features = [0; MAX_FEATURES];
        for feature in FEATURES_ALL {
          let index = feature.to_index();
          position_features[index] = match feature {
            Features::Material(piece_type) => {
              position.get_by_side_and_type(PieceColor::White, piece_type).count_ones() as i8 - position.get_by_side_and_type(PieceColor::Black, piece_type).count_ones() as i8
            }
            Features::Psqt(piece_type, square) => {
              let piece_white_pov = position.get_piece_on_square(square);
              let piece_black_pov = position.get_piece_on_square(square ^ 56);

              i8::from(piece_white_pov.piece_type == piece_type && piece_white_pov.color == PieceColor::White)
                - i8::from(piece_black_pov.piece_type == piece_type && piece_black_pov.color == PieceColor::Black)
            }
          };
        }
        position_features
      },
      mg_factor: (256f32 - position.get_phase() as f32) / 256f32,
      eg_factor: position.get_phase() as f32 / 256f32,
      side: position.get_side(),
    }
  }
}
