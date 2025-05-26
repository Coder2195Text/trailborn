use crate::core::position::Position;

pub trait Entity {
  fn get_position(&self) -> Position;
  fn set_position(&mut self, pos: Position);
}
