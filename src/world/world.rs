use std::collections::HashMap;

use godot::{classes::Engine, prelude::*};

use super::chunk::Chunk;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct World {
  world_id: GString,
  seed: i32,
  base: Base<Node>,
}

#[godot_api]
impl INode for World {}

#[godot_api]
impl World {
  #[func]
  pub fn set_seed(&mut self, seed: i32) {
    self.seed = seed;
  }

  #[func]
  pub fn get_seed(&self) -> i32 {
    self.seed
  }

  #[signal]
  fn chunk_loaded(chunk_pos: Vector2i);

  #[signal]
  fn chunk_unloaded(chunk_pos: Vector2i);

  #[signal]
  fn place_changed(name: GString);

  #[signal]
  fn place_loaded();

  #[func]
  pub fn get_world_id(&self) -> GString {
    self.world_id.clone()
  }

  #[func]
  pub fn set_world_id(&mut self, world_id: GString) {
    self.world_id = world_id;
  }
}

impl World {
  pub fn singleton() -> Gd<Self> {
    Engine::singleton()
      .get_singleton("World")
      .unwrap()
      .cast::<World>()
  }
}
