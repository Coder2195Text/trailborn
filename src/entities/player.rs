use godot::{
  classes::{
    AnimatedSprite2D, AtlasTexture, CharacterBody2D, ICharacterBody2D, ResourceLoader, Texture2D,
  },
  prelude::*,
};

use crate::core::position::Position;

const PLAYER_ACCEL: f32 = 1000.0;
const SKINS: [&str; 24] = [
  "res://assets/textures/player/F_01.png",
  "res://assets/textures/player/F_02.png",
  "res://assets/textures/player/F_03.png",
  "res://assets/textures/player/F_04.png",
  "res://assets/textures/player/F_05.png",
  "res://assets/textures/player/F_06.png",
  "res://assets/textures/player/F_07.png",
  "res://assets/textures/player/F_08.png",
  "res://assets/textures/player/F_09.png",
  "res://assets/textures/player/F_10.png",
  "res://assets/textures/player/F_11.png",
  "res://assets/textures/player/F_12.png",
  "res://assets/textures/player/M_01.png",
  "res://assets/textures/player/M_02.png",
  "res://assets/textures/player/M_03.png",
  "res://assets/textures/player/M_04.png",
  "res://assets/textures/player/M_05.png",
  "res://assets/textures/player/M_06.png",
  "res://assets/textures/player/M_07.png",
  "res://assets/textures/player/M_08.png",
  "res://assets/textures/player/M_09.png",
  "res://assets/textures/player/M_10.png",
  "res://assets/textures/player/M_11.png",
  "res://assets/textures/player/M_12.png",
];

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
  base: Base<CharacterBody2D>,
  world_position: Position,
  skin_index: i32,
}

#[godot_api]
impl ICharacterBody2D for Player {
  fn init(base: Base<CharacterBody2D>) -> Self {
    Player {
      base,
      world_position: Position::ZERO,
      skin_index: 0,
    }
  }

  fn ready(&mut self) {
    let texture = ResourceLoader::singleton()
      .load("res://assets/textures/player/F_08.png")
      .unwrap()
      .cast::<Texture2D>();

    self.change_skin(texture);
  }

  fn physics_process(&mut self, delta: f64) {
    let input = Input::singleton();

    let (next_skin, prev_skin) = (
      input.is_action_just_pressed("next_skin"),
      input.is_action_just_pressed("prev_skin"),
    );

    if next_skin {
      if self.skin_index >= SKINS.len() as i32 - 1 {
        self.skin_index = 0;
      } else {
        self.skin_index += 1;
      }
    }

    if prev_skin {
      if self.skin_index <= 0 {
        self.skin_index = SKINS.len() as i32 - 1;
      } else {
        self.skin_index -= 1;
      }
    }

    if next_skin || prev_skin {
      let skin = ResourceLoader::singleton()
        .load(SKINS[self.skin_index as usize])
        .unwrap()
        .cast::<Texture2D>();
      self.change_skin(skin);
    }

    let mut base = self.base_mut();
    let mut velocity = base.get_velocity();
    let delta = delta as f32;

    let mut sprite = base.get_node_as::<AnimatedSprite2D>("Sprite");

    let (up, down, left, right, run) = (
      input.is_action_pressed("up"),
      input.is_action_pressed("down"),
      input.is_action_pressed("left"),
      input.is_action_pressed("right"),
      input.is_action_pressed("run"),
    );

    let accel = if run { 2.0 } else { 1.0 } * PLAYER_ACCEL * delta;

    velocity *= 50.0 * delta;
    if up ^ down {
      velocity.y += if up { -accel } else { accel };
    }
    if left ^ right {
      velocity.x += if left { -accel } else { accel };
    }

    if up ^ down && !(left || right) {
      sprite.set_animation(if up { "up" } else { "down" });
    }

    if left ^ right {
      sprite.set_animation(if left { "left" } else { "right" });
    }

    if up != down || left != right {
      sprite.set_speed_scale(if run { 2.0 } else { 1.0 });
      sprite.play();
    } else {
      sprite.stop();
      sprite.set_frame(0);
    }

    base.set_velocity(velocity);
    base.move_and_slide();
  }
}

#[godot_api]
impl Player {
  #[func]
  fn change_skin(&mut self, skin: Gd<Texture2D>) {
    let base = self.base_mut();

    let mut sprite = base.get_node_as::<AnimatedSprite2D>("Sprite");
    let (anim, frame, playing) = (
      sprite.get_animation(),
      sprite.get_frame(),
      sprite.is_playing(),
    );

    let frames = sprite.get_sprite_frames().unwrap();
    let anims = frames.get_animation_names();

    for anim_idx in 0..anims.len() {
      let anim = anims.get(anim_idx).unwrap();
      for i in 0..frames.get_frame_count(anim.arg()) {
        let mut frame = frames
          .get_frame_texture(anim.arg(), i)
          .unwrap()
          .cast::<AtlasTexture>();
        frame.set_atlas(&skin);
      }
    }

    sprite.set_animation(&anim);
    sprite.set_frame(frame);
    // this block of code is to prevent the godot bug with white textures
    sprite.play();
    if !playing {
      sprite.stop();
    }
    // end hotfix
  }
}
