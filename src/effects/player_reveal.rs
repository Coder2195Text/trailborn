use godot::{
  classes::{Area2D, IArea2D},
  prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct PlayerReveal {
  base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PlayerReveal {
  fn process(&mut self, _delta: f64) {
    let base = self.base();

    let parent = base
      .get_parent()
      .map(|p| p.try_cast::<Node2D>().ok())
      .flatten();

    if let Some(mut parent) = parent {
      let modulate = parent.get_modulate();

      parent.set_modulate(modulate.lerp(
        Color::from_rgba(
          1.0,
          1.0,
          1.0,
          if base.has_overlapping_bodies() {
            0.5
          } else {
            1.0
          },
        ),
        0.05,
      ));
    }
  }
}
