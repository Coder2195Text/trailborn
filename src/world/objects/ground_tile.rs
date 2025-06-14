use std::default;

use godot::{classes::IArea2D, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, GodotConvert, Var, Export, Default)]
#[godot(via = GString)]
pub enum GroundTileType {
  #[default]
  Grass,
  Dirt,
  Stone,
}

impl GroundTileType {
  fn to_str(&self) -> &str {
    match self {
      Self::Dirt => "dirt",
      Self::Grass => "grass",
      Self::Stone => "stone",
    }
  }
}

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct GroundTile {
  #[export]
  tile: GroundTileType,
}

#[godot_api]
impl IArea2D for GroundTile {}
