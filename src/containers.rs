use std::ops::{Index, IndexMut};

use crate::piece::{PieceColor, PieceType};

#[derive(Clone, Copy, Debug)]
pub struct ByColor<T> {
  inner: [T; 2],
}

impl<T> ByColor<T> {
  pub const fn new(white: T, black: T) -> Self {
    // Black = 0, White = 1.
    Self { inner: [black, white] }
  }
}

impl<T: Copy + Default> Default for ByColor<T> {
  fn default() -> Self {
    Self { inner: [T::default(); 2] }
  }
}

impl<T> Index<PieceColor> for ByColor<T> {
  type Output = T;
  #[inline]
  fn index(&self, side: PieceColor) -> &Self::Output {
    &self.inner[side as usize]
  }
}

impl<T> IndexMut<PieceColor> for ByColor<T> {
  #[inline]
  fn index_mut(&mut self, side: PieceColor) -> &mut Self::Output {
    &mut self.inner[side as usize]
  }
}

#[derive(Clone, Copy, Debug)]
pub struct ByPieceType<T> {
  inner: [T; 6],
}

impl<T> ByPieceType<T> {
  pub const fn new(pawn: T, knight: T, bishop: T, rook: T, queen: T, king: T) -> Self {
    // [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING]
    Self { inner: [pawn, knight, bishop, rook, queen, king] }
  }
}

impl<T: Copy + Default> Default for ByPieceType<T> {
  fn default() -> Self {
    Self { inner: [T::default(); 6] }
  }
}

impl<T> Index<PieceType> for ByPieceType<T> {
  type Output = T;
  #[inline]
  fn index(&self, pt: PieceType) -> &Self::Output {
    &self.inner[pt as usize]
  }
}

impl<T> IndexMut<PieceType> for ByPieceType<T> {
  #[inline]
  fn index_mut(&mut self, pt: PieceType) -> &mut Self::Output {
    &mut self.inner[pt as usize]
  }
}
