use std::ops::{Add, Neg};

use fixed::{traits::Fixed, types::I29F3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
  pub x: I29F3,
  pub y: I29F3,
}

impl Position {
  pub const ZERO: Self = Position {
    x: I29F3::ZERO,
    y: I29F3::ZERO,
  };
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
