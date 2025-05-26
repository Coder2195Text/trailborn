mod core;
mod entity;
mod screen;
mod ui;
mod world;

#[cfg(test)]
mod tests;

use godot::{classes::Engine, prelude::*};
use world::world::World;

struct TrailbornExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TrailbornExtension {
  fn on_level_init(level: InitLevel) {
    if level == InitLevel::Scene {
      Engine::singleton().register_singleton("World", &World::new_alloc());
    }
  }

  fn on_level_deinit(level: InitLevel) {
    if level == InitLevel::Scene {
      // Let's keep a variable of our Engine singleton instance,
      // and MyEngineSingleton name.
      let mut engine = Engine::singleton();

      // Here, we manually retrieve our singleton(s) that we've registered,
      // so we can unregister them and free them from memory - unregistering
      // singletons isn't handled automatically by the library.
      if let Some(world) = engine.get_singleton("World") {
        // Unregistering from Godot, and freeing from memory is required
        // to avoid memory leaks, warnings, and hot reloading problems.
        engine.unregister_singleton("World");
        world.free();
      } else {
        // You can either recover, or panic from here.
        godot_error!("Failed to get singleton");
      }
    }
  }
}
