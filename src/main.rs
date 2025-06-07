mod audio;
mod color;
mod console;
mod core;
mod duration;
mod input;
mod particle;
mod player;
mod prelude;
mod state;
mod ui;
mod world;

use bevy::prelude::*;
use prelude::*;

fn main() {
    App::new()
        .add_plugins((
            CorePlugin,
            StatePlugin,
            ParticlePlugin,
            WorldPlugin,
            PlayerPlugin,
            DurationPlugin,
            InputPlugin,
            UiPlugin,
            ConsolePlugin,
            AudioPlugin,
        ))
        .run();
}
