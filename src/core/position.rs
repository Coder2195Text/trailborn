use std::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub};

use fixed::{
  traits::{FromFixed, ToFixed},
  types::{I32F32, extra::LeEqU64},
};
use godot::builtin::{Vector2, Vector2i};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
  pub x: I32F32,
  pub y: I32F32,
}

impl Position {
  pub const ZERO: Self = Position {
    x: I32F32::ZERO,
    y: I32F32::ZERO,
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

impl Sub for Position {
  type Output = Position;

  fn sub(self, other: Position) -> Position {
    Position {
      x: self.x - other.x,
      y: self.y - other.y,
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

impl<X, Y> From<(X, Y)> for Position
where
  X: ToFixed,
  Y: ToFixed,
{
  fn from((x, y): (X, Y)) -> Self {
    Position {
      x: I32F32::from_num(x),
      y: I32F32::from_num(y),
    }
  }
}

// refuses to work with normal Into trait
impl Position {
  fn into<X: FromFixed, Y: FromFixed>(self) -> (X, Y) {
    (self.x.to_num::<X>(), self.y.to_num::<Y>())
  }
}

impl From<Position> for (I32F32, I32F32) {
  fn from(pos: Position) -> Self {
    (pos.x, pos.y)
  }
}

impl From<Vector2> for Position {
  fn from(vec: Vector2) -> Self {
    return Self::from((vec.x, vec.y));
  }
}

impl From<Vector2i> for Position {
  fn from(vec: Vector2i) -> Self {
    Self::from((vec.x, vec.y))
  }
}

impl<T> Div<T> for Position
where
  T: ToFixed + Copy,
{
  type Output = Position;

  fn div(self, scalar: T) -> Position {
    Position {
      x: self.x / I32F32::from_num(scalar),
      y: self.y / I32F32::from_num(scalar),
    }
  }
}

impl<T> Mul<T> for Position
where
  T: ToFixed + Copy,
{
  type Output = Position;

  fn mul(self, scalar: T) -> Position {
    Position {
      x: self.x * I32F32::from_num(scalar),
      y: self.y * I32F32::from_num(scalar),
    }
  }
}

impl<T> Rem<T> for Position
where
  T: ToFixed + Copy,
{
  type Output = Position;

  fn rem(self, scalar: T) -> Position {
    Position {
      x: self.x % I32F32::from_num(scalar),
      y: self.y % I32F32::from_num(scalar),
    }
  }
}

impl Add<Vector2> for Position {
  type Output = Position;

  fn add(self, other: Vector2) -> Position {
    Position {
      x: self.x + I32F32::from_num(other.x),
      y: self.y + I32F32::from_num(other.y),
    }
  }
}

impl Into<Vector2> for Position {
  fn into(self) -> Vector2 {
    Vector2::new(self.x.to_num(), self.y.to_num())
  }
}
