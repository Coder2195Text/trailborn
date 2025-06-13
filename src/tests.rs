use godot::builtin::{Vector2, Vector2i};

use crate::{core::position::Position, world::chunk::Chunk};

#[test]
fn get_chunk_pos() {
  let chunk_position_tests: Vec<(Position, Vector2i)> = vec![
    (Position::from((0, 0)), Vector2i::new(0, 0)),
    (Position::from((32, 32)), Vector2i::new(1, 1)),
    (Position::from((39, 39)), Vector2i::new(1, 1)),
    (Position::from((-0.01, -0.01)), Vector2i::new(-1, -1)),
    (Position::from((-20000.10, 10)), Vector2i::new(-1001, 0)),
    (Position::from((20000.10, -10)), Vector2i::new(1000, -1)),
    (Position::from((0, -0.01)), Vector2i::new(0, -1)),
  ];

  for (pos, expected) in chunk_position_tests {
    assert_eq!(Chunk::get_chunk_at_position(pos), expected)
  }
}

#[test]
fn get_chunk_offset() {
  let chunk_offset_tests: Vec<(Position, Vector2)> = vec![
    (Position::from((0, 0)), Vector2::new(0.0, 0.0)),
    (Position::from((32, 32)), Vector2::new(12.0, 12.0)),
    (Position::from((39, 39)), Vector2::new(19.0, 19.0)),
    (Position::from((-0.01, -0.01)), Vector2::new(19.99, 19.99)),
    (Position::from((-20000.1, 10)), Vector2::new(19.90, 10.0)),
    (Position::from((20000.1, -10)), Vector2::new(0.10, 10.0)),
    (Position::from((0, -0.01)), Vector2::new(0.0, 19.99)),
  ];

  for (pos, expected) in chunk_offset_tests {
    assert_eq!(
      Chunk::get_chunk_offset(pos),
      expected,
      "Failed for position: {:?}, expected offset: {:?}",
      pos,
      expected
    )
  }
}
