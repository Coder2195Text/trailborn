use godot::{
  classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D},
  prelude::*,
};

const PLAYER_ACCEL: f32 = 1000.0;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
struct Player {
  base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
  fn physics_process(&mut self, delta: f64) {
    let mut base = self.base_mut();
    let mut velocity = base.get_velocity();
    let delta = delta as f32;

    let mut sprite = base.get_node_as::<AnimatedSprite2D>("Sprite");

    let input = Input::singleton();

    let (up, down, left, right) = (
      input.is_action_pressed("up"),
      input.is_action_pressed("down"),
      input.is_action_pressed("left"),
      input.is_action_pressed("right"),
    );

    let accel = if input.is_action_pressed("run") {
      2.0
    } else {
      1.0
    } * PLAYER_ACCEL
      * delta;

    velocity *= 50.0 * delta;

    if up {
      velocity.y += -accel;
    }
    if down {
      velocity.y += accel;
    }
    if left {
      velocity.x += -accel;
    }
    if right {
      velocity.x += accel;
    }

    if up ^ down && !(left || right) {
      sprite.set_animation(if up { "up" } else { "down" });
    }

    if left ^ right {
      sprite.set_animation(if left { "left" } else { "right" });
    }

    if up || down || left || right {
      sprite.play();
    } else {
      sprite.stop();
      sprite.set_frame(0);
    }

    base.set_velocity(velocity);

    base.move_and_slide();
  }
}
