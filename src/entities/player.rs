use fixed::traits::LossyInto;
use godot::{
  classes::{
    AnimatedSprite2D, AtlasTexture, CharacterBody2D, ICharacterBody2D, Input, ResourceLoader,
    Texture2D,
  },
  prelude::*,
};

use crate::core::{constants::TILE_SIZE, position::Position};

use super::traits::Entity;

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

const SKIN_COUNT: i32 = SKINS.len() as i32;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
  base: Base<CharacterBody2D>,
  offset: Vector2i,
  #[export(range=(0.0,SKIN_COUNT as f64 - 1.0))]
  skin_index: i32,
}

#[godot_api]
impl ICharacterBody2D for Player {
  fn init(base: Base<CharacterBody2D>) -> Self {
    Player {
      base,
      offset: Vector2i::ZERO,
      skin_index: 0,
    }
  }

  fn ready(&mut self) {
    let texture = ResourceLoader::singleton()
      .load(SKINS[self.skin_index as usize])
      .unwrap()
      .cast::<Texture2D>();

    self.change_skin(texture);
  }

  fn physics_process(&mut self, delta: f64) {
    let input = Input::singleton();

    let mut offset = self.get_offset();

    let (next_skin, prev_skin) = (
      input.is_action_just_pressed("next_skin"),
      input.is_action_just_pressed("prev_skin"),
    );

    if next_skin {
      if self.skin_index >= SKIN_COUNT - 1 {
        self.skin_index = 0;
      } else {
        self.skin_index += 1;
      }
    }

    if prev_skin {
      if self.skin_index <= 0 {
        self.skin_index = SKIN_COUNT - 1;
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

    let initial_position = base.get_position();

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

    base.set_velocity(velocity);
    base.move_and_slide();

    let final_position = base.get_position();

    let delta = (final_position - initial_position).length();

    if up ^ down && !(left || right) {
      sprite.set_animation(if up { "up" } else { "down" });
    }

    if left ^ right {
      sprite.set_animation(if left { "left" } else { "right" });
    }

    if (up != down || left != right) && delta > 0.01 {
      sprite.set_speed_scale(if run { 2.0 } else { 1.0 });
      sprite.play();
    } else {
      sprite.stop();
      sprite.set_frame(0);
    }

    let mut position = base.get_position();

    if position.x >= TILE_SIZE {
      offset.x += 1;
      position.x -= TILE_SIZE;
    } else if position.x <= -TILE_SIZE {
      offset.x -= 1;
      position.x += TILE_SIZE;
    }

    if position.y >= TILE_SIZE {
      offset.y += 1;
      position.y -= TILE_SIZE;
    } else if position.y <= -TILE_SIZE {
      offset.y -= 1;
      position.y += TILE_SIZE;
    }

    base.set_position(position);

    drop(base);

    self.signals().offset_changed().emit(offset);

    self.set_offset(offset);
  }
}

#[godot_api]
impl Player {
  #[signal]
  pub fn offset_changed(offset: Vector2i);

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

  #[func]
  fn get_offset(&mut self) -> Vector2i {
    self.offset
  }

  #[func]
  fn set_offset(&mut self, offset: Vector2i) {
    self.offset = offset;
  }
}

impl Entity for Player {
  fn get_position(&self) -> Position {
    let pos = Position::from(self.offset);

    pos
  }

  fn set_position(&mut self, pos: Position) {
    self.offset = Vector2i::new(pos.x.int().lossy_into(), pos.y.int().lossy_into());
    let mut base = self.base_mut();

    base.set_position(Vector2::new(
      pos.x.frac().lossy_into(),
      pos.y.frac().lossy_into(),
    ));
  }
}
