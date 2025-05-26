use godot::{classes::class_macros::registry::signal, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct World {
  seed: i32,
  base: Base<Object>,
}

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
}
