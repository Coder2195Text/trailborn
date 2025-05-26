pub const TILE_SIZE: f32 = 32.0;
pub const CHUNK_SIZE: i32 = 20;

#[derive(Debug, Default)]
pub enum LoadState {
  Loading(f32),
  Loaded,
  Error,
  #[default]
  Unloaded,
}
