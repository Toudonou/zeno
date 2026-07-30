use crate::piece::{PieceColor, PieceType};
use crate::position::Position;
use crate::square::Square;
use crate::tuner::features::Features;
use crate::{get_lsb, pop_lsb};

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
    Self {
      board: {
        let mut position_features = Vec::with_capacity(16);
        for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
          // Features::Material
          let index = Features::Material(piece_type).to_index();
          let feature = position.get_by_side_and_type(PieceColor::White, piece_type).count_ones() as i8 - position.get_by_side_and_type(PieceColor::Black, piece_type).count_ones() as i8;

          if feature != 0 {
            position_features.push((index, feature));
          }

          // Features::Psqt
          let mut board = position.get_by_type(piece_type);
          while board != 0 {
            let square = get_lsb!(board);

            let piece_normal_pov = position.get_piece_on_square(square);
            let piece_inverse_pov = position.get_piece_on_square(square ^ 56);

            let (index, feature) = match piece_normal_pov.color {
              PieceColor::Black => {
                let index = Features::Psqt(piece_type, square ^ 56).to_index();
                let feature = i8::from(piece_inverse_pov.piece_type == piece_type && piece_inverse_pov.color == PieceColor::White)
                  - i8::from(piece_normal_pov.piece_type == piece_type && piece_normal_pov.color == PieceColor::Black);
                (index, feature)
              }
              PieceColor::White => {
                let index = Features::Psqt(piece_type, square).to_index();
                let feature = i8::from(piece_normal_pov.piece_type == piece_type && piece_normal_pov.color == PieceColor::White)
                  - i8::from(piece_inverse_pov.piece_type == piece_type && piece_inverse_pov.color == PieceColor::Black);
                (index, feature)
              }
              PieceColor::None => panic!("Invalid color"),
            };

            if feature != 0 {
              position_features.push((index, feature));
            }

            pop_lsb!(board);
          }
        }
        position_features
      },
      mg_factor: (256f32 - position.get_phase() as f32) / 256f32,
      eg_factor: position.get_phase() as f32 / 256f32,
      side: position.get_side(),
    }
  }
}
