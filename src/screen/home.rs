use godot::{classes::IControl, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Control)]
struct HomeScreen {}

#[godot_api]
impl IControl for HomeScreen {
  fn ready(&mut self) {}
}
