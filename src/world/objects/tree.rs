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
#[class(base=StaticBody2D,init,tool)]
pub struct TreeObject {
  #[export]
  #[var(get,set=set_tree_size)]
  size: TreeSize,
  #[export]
  #[var(get,set=set_tree_type)]
  tree_type: TreeType,

  ready: bool,
  shapes: Vec<Gd<CollisionShape2D>>,

  base: Base<StaticBody2D>,
}

#[godot_api]
impl IStaticBody2D for TreeObject {
  fn ready(&mut self) {
    self.ready = true;
    self.update_tree();
  }
}

#[godot_api]
impl TreeObject {
  #[func]
  pub fn set_tree_size(&mut self, size: GString) {
    self.size.set_property(size);
    if self.ready {
      self.update_tree();
    }
  }

  #[func]
  pub fn set_tree_type(&mut self, tree_type: GString) {
    self.tree_type.set_property(tree_type);

    if self.ready {
      self.update_tree();
    }
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

    if let Some(mut reveal) = base.try_get_node_as::<PlayerReveal>("Reveal") {
      let mut shapes = vec![];
      for reveal_shape in &reveal_shapes {
        reveal.add_child(reveal_shape);
        shapes.push(reveal_shape.clone());
      }

      for collision_shape in &collision_shapes {
        base.add_child(collision_shape);
        shapes.push(collision_shape.clone());
      }

      drop(base);

      for shape in self.shapes.iter_mut() {
        shape.queue_free();
      }

      self.shapes = shapes;
    }
  }

  #[func]
  fn get_tree_offset(&self) -> Vector2 {
    match (self.tree_type, self.size) {
      (TreeType::Barren, TreeSize::Large) => Vector2::new(3.0, -25.0),
      (TreeType::Barren, TreeSize::Medium) => Vector2::new(1.0, -21.0),
      (TreeType::Barren, TreeSize::Small) => Vector2::new(-1.0, -10.0),

      (TreeType::Bushwood, TreeSize::Large) => Vector2::new(1.0, -28.0),
      (TreeType::Bushwood, TreeSize::Medium) => Vector2::new(0.0, -18.0),
      (TreeType::Bushwood, TreeSize::Small) => Vector2::new(0.0, -15.0),

      (TreeType::Snow, TreeSize::Large) => Vector2::new(1.5, -26.0),
      (TreeType::Snow, TreeSize::Medium) => Vector2::new(0.0, -22.0),
      (TreeType::Snow, TreeSize::Small) => Vector2::new(0.0, -15.0),
      _ => Vector2::ZERO,
    }
  }

  #[func]
  fn get_reveal_shapes(&self) -> Vec<Gd<CollisionShape2D>> {
    // reveal shape should encompass a all the leaves and branches
    match (self.tree_type, self.size) {
      // insert multiple reveal shapes above if needed, otherwise default to one individual shape below
      _ => vec![match (self.tree_type, self.size) {
        (TreeType::Barren, TreeSize::Large) => Collider::new()
          .circle(64.0)
          .at(Vector2::new(4.0, -60.0))
          .done(),
        (TreeType::Barren, TreeSize::Medium) => Collider::new()
          .circle(55.0)
          .at(Vector2::new(0.0, -54.0))
          .done(),
        (TreeType::Barren, TreeSize::Small) => Collider::new()
          .circle(37.0)
          .at(Vector2::new(-2.0, -31.0))
          .done(),

        (TreeType::Bushwood, TreeSize::Large) => Collider::new()
          .circle(71.0)
          .at(Vector2::new(2.0, -68.0))
          .done(),
        (TreeType::Bushwood, TreeSize::Medium) => Collider::new()
          .circle(55.0)
          .at(Vector2::new(0.0, -46.0))
          .done(),
        (TreeType::Bushwood, TreeSize::Small) => Collider::new()
          .circle(41.0)
          .at(Vector2::new(-0.0, -39.0))
          .done(),

        (TreeType::Snow, TreeSize::Large) => Collider::new()
          .circle(71.0)
          .at(Vector2::new(1.0, -68.0))
          .done(),
        (TreeType::Snow, TreeSize::Medium) => Collider::new()
          .circle(58.0)
          .at(Vector2::new(0.0, -56.0))
          .done(),
        (TreeType::Snow, TreeSize::Small) => Collider::new()
          .circle(42.0)
          .at(Vector2::new(1.0, -35.0))
          .done(),

        _ => Collider::none(),
      }],
    }
  }

  #[func]
  fn get_collision_shapes(&self) -> Vec<Gd<CollisionShape2D>> {
    // REALLY IMPORTANT: This should mostly always be the trunk, which is supposed to be centered at (0,0) aka lack of an at() call
    // Only time we use coords is for extremely strange trees that might have double trunks or something
    match (self.tree_type, self.size) {
      // insert multiple collision shapes above if needed, otherwise default to one individual shape below
      _ => vec![match (self.tree_type, self.size) {
        (TreeType::Barren, TreeSize::Large) => Collider::new().capsule(54.0, 13.0).deg(90.0).done(),
        (TreeType::Barren, TreeSize::Medium) => {
          Collider::new().capsule(46.0, 13.0).deg(90.0).done()
        }
        (TreeType::Barren, TreeSize::Small) => Collider::new().capsule(36.0, 11.0).deg(90.0).done(),

        (TreeType::Bushwood, TreeSize::Large) => {
          Collider::new().capsule(54.0, 17.0).deg(90.0).done()
        }
        (TreeType::Bushwood, TreeSize::Medium) => {
          Collider::new().capsule(36.0, 12.0).deg(90.0).done()
        }
        (TreeType::Bushwood, TreeSize::Small) => {
          Collider::new().capsule(28.0, 8.0).deg(90.0).done()
        }

        (TreeType::Snow, TreeSize::Large) => Collider::new().capsule(50.0, 16.0).deg(90.0).done(),
        (TreeType::Snow, TreeSize::Medium) => Collider::new().capsule(46.0, 14.0).deg(90.0).done(),
        (TreeType::Snow, TreeSize::Small) => Collider::new().capsule(36.0, 9.0).deg(90.0).done(),

        _ => Collider::none(),
      }],
    }
  }
}
