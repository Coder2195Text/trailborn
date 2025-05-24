use std::ops::{Add, Neg};

use fixed::types::I32F32;
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

impl Neg for Position {
  type Output = Position;

  fn neg(self) -> Position {
    Position {
      x: -self.x,
      y: -self.y,
    }
  }
}

impl From<(i32, i32)> for Position {
  fn from((x, y): (i32, i32)) -> Self {
    Position {
      x: I32F32::from_num(x),
      y: I32F32::from_num(y),
    }
  }
}

impl From<(f32, f32)> for Position {
  fn from((x, y): (f32, f32)) -> Self {
    Position {
      x: I32F32::from_num(x),
      y: I32F32::from_num(y),
    }
  }
}

impl From<(I32F32, I32F32)> for Position {
  fn from((x, y): (I32F32, I32F32)) -> Self {
    Position { x, y }
  }
}

impl From<Position> for (I32F32, I32F32) {
  fn from(pos: Position) -> Self {
    (pos.x, pos.y)
  }
}

impl From<Vector2> for Position {
  fn from(vec: Vector2) -> Self {
    Position {
      x: I32F32::from_num(vec.x),
      y: I32F32::from_num(vec.y),
    }
  }
}

impl From<Vector2i> for Position {
  fn from(vec: Vector2i) -> Self {
    Position {
      x: I32F32::from_num(vec.x),
      y: I32F32::from_num(vec.y),
    }
  }
}
