use godot::{
  classes::{Camera2D, ICamera2D},
  prelude::*,
};

use crate::core::constants::SCREEN_SIZE;

#[derive(GodotClass)]
#[class(base=Camera2D)]
pub struct GameCamera {
  target_position: Vector2,
  follow_speed: f32,
  base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for GameCamera {
  fn init(base: Base<Camera2D>) -> Self {
    GameCamera {
      target_position: Vector2::ZERO,
      follow_speed: 2.0,
      base,
    }
  }

  fn physics_process(&mut self, delta: f64) {
    let delta = delta as f32;
    let lerp_factor = self.follow_speed * delta;
    let target = self.target_position;

    let mut base = self.base_mut();
    let position = base.get_position();

    base.set_position(position.lerp(target, lerp_factor));
  }

  fn process(&mut self, delta: f64) {
    let mut base = self.base_mut();

    if let Some((size, ratio)) = base
      .get_viewport()
      .map(|v| v.get_visible_rect().size)
      .map(|s| (s, s.x / s.y))
    {
      let zoom = if ratio > 16.0 / 9.0 {
        size.x / SCREEN_SIZE.x as f32
      } else {
        size.y / SCREEN_SIZE.y as f32
      };
      base.set_zoom(Vector2::new(zoom, zoom));
    }
  }
}

#[godot_api]
impl GameCamera {
  #[func]
  pub fn set_target_position(&mut self, target: Vector2) {
    self.target_position = target;
  }

  #[func]
  pub fn set_follow_speed(&mut self, speed: f32) {
    self.follow_speed = speed;
  }

  #[func]
  pub fn get_target_position(&self) -> Vector2 {
    self.target_position
  }

  #[func]
  pub fn get_follow_speed(&self) -> f32 {
    self.follow_speed
  }
}
