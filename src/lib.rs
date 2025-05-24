mod core;
mod entities;
mod scenes;
mod ui;
mod world;

#[cfg(test)]
mod tests;

use godot::prelude::*;

struct TrailbornExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TrailbornExtension {}
