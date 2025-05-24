use godot::{classes::ICamera2D, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Camera2D)]
struct GameCamera {}

#[godot_api]
impl ICamera2D for GameCamera {}
