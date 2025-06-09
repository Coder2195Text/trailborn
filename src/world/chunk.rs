use std::time::Duration;

use godot::{
  classes::{Engine, FileAccess, ResourceLoader, Sprite2D, file_access::ModeFlags},
  prelude::*,
};

use crate::{
  core::{
    constants::{CHUNK_LENGTH, CHUNK_SIZE, TILE_SIZE},
    position::Position,
  },
  entities::player::Player,
  world::world::World,
};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Chunk {
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

    base.to_variant();

    let offset = Vector2::new(offset.x as f32, offset.y as f32) * TILE_SIZE;

    base.set_position(offset);
  }

  #[func]
  pub fn unload(&mut self) {
    let mut base = self.base_mut();
    // Unload the chunk
    // and save it to disk
    base.queue_free();
  }

  #[func]
  pub fn load(chunk_pos: Vector2i) -> Gd<Self> {
    // do filesystem operations to load the chunk
    godot_print!("Chunk loading at position: {:?}", chunk_pos);

    let world_id = World::singleton().bind().get_world_id();

    let file = FileAccess::open(
      &GString::format(
        &"user://saves/{0}/chunks/{1}_{2}.chunk".into(),
        &[
          world_id,
          chunk_pos.x.to_string().into(),
          chunk_pos.y.to_string().into(),
        ]
        .to_variant(),
      ),
      ModeFlags::READ,
    );

    let mut obj = Gd::from_init_fn(move |base| Self { base, chunk_pos });

    let mut resource_loader = ResourceLoader::singleton();

    let grass = resource_loader
      .load("res://nodes/objects/grass.tscn")
      .unwrap()
      .cast::<PackedScene>();

    if let Some(file) = file {
      // load file data
    } else {
      for _ in 0..10 {
        let mut grass_instance = grass.instantiate_as::<Sprite2D>();

        grass_instance.set_position(Vector2::new(
          rand::random::<f32>() * CHUNK_LENGTH,
          rand::random::<f32>() * CHUNK_LENGTH,
        ));

        obj.add_child(&grass_instance);
      }
      godot_print!("Chunk file not found at position: {:?}", chunk_pos);
    }

    godot_print!("Chunk loaded at position: {:?}", chunk_pos);

    // TODO: Get the player through alternative means
    let player = obj
      .get_parent()
      .map(|parent| parent.get_node_as::<Player>("Player"))
      .unwrap();

    player
      .signals()
      .offset_changed()
      .connect_other(&obj, Self::offset_position);

    obj
  }
}
