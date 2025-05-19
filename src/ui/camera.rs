use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Camera2D)]
struct GameCamera {}

#[godot_api]
impl ICamera2D for GameCamera {}
