use std::ops::{Add, Neg};

use fixed::{FixedI32, types::extra::U3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
  pub x: FixedI32<U3>,
  pub y: FixedI32<U3>,
}

impl Add for Position {
  type Output = Position;

  fn add(self, other: Position) -> Position {
    Position {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Neg for Position {
  type Output = Position;

  fn neg(self) -> Position {
    Position {
      x: -self.x,
      y: -self.y,
    }
  }
}
