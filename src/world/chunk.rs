use godot::prelude::*;

use crate::core::{
  constants::{CHUNK_SIZE, TILE_SIZE},
  position::Position,
};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct Chunk {
  #[export]
  chunk_pos: Vector2i,
  base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Chunk {
  fn ready(&mut self) {
    self.offset_position(Vector2i::ZERO);
  }
}

#[godot_api]
impl Chunk {
  #[func]
  pub fn offset_position(&mut self, offset: Vector2i) {
    let offset = self.chunk_pos * CHUNK_SIZE - offset;

    let mut base = self.base_mut();

    let offset = Vector2::new(offset.x as f32, offset.y as f32) * TILE_SIZE;

    base.set_position(offset);
  }

  fn unload(&mut self) {
    let mut base = self.base_mut();
    // Unload the chunk
    // and save it to disk
    base.queue_free();
  }

  fn load(chunk_pos: Vector2i) -> Gd<Self> {
    // do filesystem operations to load the chunk
    Gd::from_init_fn(|base| {
      // Accept a base of type Base<Node3D> and directly forward it.
      Self { base, chunk_pos }
    })
  }
}
