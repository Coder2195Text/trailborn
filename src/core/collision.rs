use godot::prelude::*;

use godot::classes::{CapsuleShape2D, CircleShape2D, CollisionShape2D, RectangleShape2D};

pub struct Collider {
  inner: Gd<CollisionShape2D>,
}

impl Collider {
  pub fn new() -> Self {
    Collider {
      inner: CollisionShape2D::new_alloc(),
    }
  }

  pub fn at(mut self, position: Vector2) -> Self {
    self.inner.set_position(position);
    self
  }

  pub fn circle(mut self, radius: f32) -> Self {
    let mut shape = CircleShape2D::new_gd();
    shape.set_radius(radius);

    self.inner.set_shape(&shape);
    self
  }

  pub fn capsule(mut self, height: f32, radius: f32) -> Self {
    let mut shape = CapsuleShape2D::new_gd();
    shape.set_height(height);
    shape.set_radius(radius);

    self.inner.set_shape(&shape);
    self
  }

  pub fn rect(mut self, width: f32, height: f32) -> Self {
    let mut shape = RectangleShape2D::new_gd();
    shape.set_size(Vector2 {
      x: width,
      y: height,
    });
    self.inner.set_shape(&shape);
    self
  }

  pub fn rad(mut self, angle: f32) -> Self {
    self.inner.set_rotation(angle);
    self
  }

  pub fn deg(mut self, angle: f32) -> Self {
    self.inner.set_rotation_degrees(angle);
    self
  }

  pub fn done(self) -> Gd<CollisionShape2D> {
    self.inner
  }
}
