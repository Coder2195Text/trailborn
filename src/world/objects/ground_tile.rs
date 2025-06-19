use std::str::FromStr;
use strum::IntoEnumIterator;

use godot::{
  classes::{AnimatedSprite2D, Area2D, IArea2D},
  global::PropertyHint,
  meta::PropertyHintInfo,
  prelude::*,
};

use strum::{EnumIter, IntoStaticStr};
use strum_macros::EnumString;

use crate::impl_EnumVar;

#[derive(
  EnumIter, GodotConvert, Export, Debug, EnumString, IntoStaticStr, Default, Clone, Copy,
)]
#[strum(serialize_all = "snake_case")]
#[godot(via = GString)]
pub enum GroundTileType {
  #[default]
  Grass,
  Dirt,
  Stone,
  StuffBitch,
}

impl_EnumVar! { for GroundTileType }

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct GroundTile {
  #[export]
  #[var(get,set=set_tile)]
  tile: GroundTileType,
  base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for GroundTile {
  fn ready(&mut self) {
    self.update_tile();
  }
}

#[godot_api]
impl GroundTile {
  #[func]
  fn set_tile(&mut self, tile: GString) {
    self.tile.set_property(tile);

    self.update_tile();
  }

  #[func]
  fn update_tile(&mut self) {
    let tile_name: &str = self.tile.into();
    let base = self.base_mut();

    if let Some(mut sprite) = base.try_get_node_as::<AnimatedSprite2D>("Sprite") {
      sprite.set_animation(tile_name);
    }
  }
}
