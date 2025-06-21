use godot::classes::{AnimatedSprite2D, CollisionShape2D};
use godot::global::PropertyHint;
use godot::meta::PropertyHintInfo;
use godot::{
  classes::{IStaticBody2D, StaticBody2D},
  prelude::*,
};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::core::collision::Collider;
use crate::effects::player_reveal::PlayerReveal;
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

    let tree_type: &str = tree_type.into();
    let tree_size: &str = tree_size.into();

    let reveal_shapes = self.get_reveal_shapes();
    let collision_shapes = self.get_collision_shapes();
    let tree_offset = self.get_tree_offset();

    let mut base = self.base_mut();

    let animation_name = format!("{}_{}", tree_type, tree_size);

    if let Some(mut sprite) = base.try_get_node_as::<AnimatedSprite2D>("Sprite") {
      sprite.set_animation(GString::from(animation_name).arg());
      sprite.set_offset(tree_offset);
    }

    base.get_tree().map(|mut tree| {
      let reveals = tree.get_nodes_in_group("reveal");
      let solids = tree.get_nodes_in_group("solids");

      for mut node in reveals.iter_shared() {
        node.remove_from_group("reveal");
        node.queue_free();
      }

      for mut node in solids.iter_shared() {
        node.remove_from_group("solids");
        node.queue_free();
      }
    });

    if let Some(mut reveal) = base.try_get_node_as::<PlayerReveal>("Reveal") {
      for mut reveal_shape in reveal_shapes {
        reveal_shape.add_to_group("reveal");
        reveal.add_child(&reveal_shape);
      }
    }

    for mut collison_shape in collision_shapes {
      collison_shape.add_to_group("solids");
      base.add_child(&collison_shape);
    }
  }

  #[func]
  fn get_tree_offset(&self) -> Vector2 {
    match (self.size, self.tree_type) {
      _ => Vector2::ZERO,
    }
  }

  #[func]
  fn get_reveal_shapes(&self) -> Vec<Gd<CollisionShape2D>> {
    match (self.size, self.tree_type) {
      // insert multiple reveal shapes above if needed, otherwise default to one individual shape below
      _ => vec![match (self.size, self.tree_type) {
        _ => Collider::new().circle(100.0).done(),
      }],
    }
  }

  #[func]
  fn get_collision_shapes(&self) -> Vec<Gd<CollisionShape2D>> {
    match (self.size, self.tree_type) {
      // insert multiple collision shapes above if needed, otherwise default to one individual shape below
      _ => vec![match (self.size, self.tree_type) {
        _ => Collider::new().capsule(60.0, 14.0).deg(90.0).done(),
      }],
    }
  }
}
