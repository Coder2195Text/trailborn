use godot::classes::{AnimatedSprite2D, CapsuleShape2D, Shape2D};
use godot::global::PropertyHint;
use godot::meta::PropertyHintInfo;
use godot::{
  classes::{IStaticBody2D, StaticBody2D},
  prelude::*,
};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::impl_EnumVar;

#[derive(
  EnumIter, GodotConvert, Export, Debug, EnumString, IntoStaticStr, Default, Clone, Copy,
)]
#[strum(serialize_all = "snake_case")]
#[godot(via = GString)]
pub enum TreeSize {
  #[default]
  Large,
  Medium,
  Small,
}

#[derive(
  EnumIter, GodotConvert, Export, Debug, EnumString, IntoStaticStr, Default, Clone, Copy,
)]
#[strum(serialize_all = "snake_case")]
#[godot(via = GString)]
pub enum TreeType {
  Barren,
  Bushwood,
  Cherry,
  Coconut,
  FanPalm,
  FrostGreen,
  Fruit,
  #[default]
  Greenleaf,
  Maple,
  Moss,
  Snow,
}

impl_EnumVar!( for TreeSize, TreeType );

struct TreeData {
  offset: Vector2,
  shape: Shape2D,
}

#[derive(GodotClass)]
#[class(base=StaticBody2D,init)]
pub struct TreeObject {
  #[export]
  #[var(get,set=set_tree_size)]
  size: TreeSize,
  #[export]
  #[var(get,set=set_tree_type)]
  tree_type: TreeType,

  base: Base<StaticBody2D>,
}

#[godot_api]
impl IStaticBody2D for TreeObject {
  fn ready(&mut self) {
    self.update_tree();
  }
}

#[godot_api]
impl TreeObject {
  #[func]
  fn set_tree_size(&mut self, size: GString) {
    self.size.set_property(size);

    self.update_tree();
  }

  #[func]
  fn set_tree_type(&mut self, tree_type: GString) {
    self.tree_type.set_property(tree_type);

    self.update_tree();
  }

  #[func]
  fn update_tree(&mut self) {
    let tree_type = self.tree_type;
    let tree_size = self.size;
    let base = self.base_mut();

    let tree_type: &str = tree_type.into();
    let tree_size: &str = tree_size.into();

    let animation_name = format!("{}_{}", tree_type, tree_size);

    if let Some(mut sprite) = base.try_get_node_as::<AnimatedSprite2D>("Sprite") {
      sprite.set_animation(GString::from(animation_name).arg());
    }
  }

  // fn get_tree_details(&mut self) {

  //   let tree_type = self.tree_type;
  //   let tree_size = self.size;

  //   match (tree_type, tree_size) {
  //     (TreeType::Barren, TreeSize::Large) => TreeData {
  //       offset: Vector2::new(0.0, -64.0),
  //       shape: CapsuleShape2D::new_gd().set_height(),
  //     },
  //     _ => TreeData {
  //       offset: Vector2::new(0.0, -32.0),
  //       shape: Shape2D::default(),
  //     },
  //   };

  // }
}
