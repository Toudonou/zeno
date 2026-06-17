use crate::piece::{PieceColor, PieceType};
use crate::pop_lsb;
use crate::position::Position;
use crate::tuner::features::{FEATURES_ALL, Features, MAX_FEATURES};
use crate::utils::{TOTAL_PHASE, get_phase};

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
    let mut phase = TOTAL_PHASE;
    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
      for color in [PieceColor::White, PieceColor::Black] {
        let mut board = position.get_by_side_and_type(color, piece_type);
        while board != 0 {
          phase -= get_phase(piece_type);
          pop_lsb!(board);
        }
      }
    }

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
      mg_factor: (256f32 - phase as f32) / 256f32,
      eg_factor: phase as f32 / 256f32,
      side: position.get_side(),
    }
  }
}
