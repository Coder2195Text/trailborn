use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct Settings {
  base: Base<Object>,
}

#[godot_api]
impl Settings {}
