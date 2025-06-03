mod core;
mod input;
mod particle;
mod player;
mod prelude;
mod ui;
mod world;

use bevy::prelude::*;
use prelude::*;

fn main() {
    App::new()
        .add_plugins((
            CorePlugin,
            ParticlePlugin,
            WorldPlugin,
            PlayerPlugin,
            InputPlugin,
            UiPlugin,
        ))
        .run();
}
