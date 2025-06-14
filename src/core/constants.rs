use godot::builtin::Vector2i;

pub const TILE_SIZE: f32 = 32.0;
pub const CHUNK_SIZE: i32 = 20;
pub const CHUNK_LENGTH: f32 = CHUNK_SIZE as f32 * TILE_SIZE;

pub const SCREEN_SIZE: Vector2i = Vector2i::new(1280, 720);
