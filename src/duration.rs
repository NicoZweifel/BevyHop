use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_fps_controller::controller::LogicalPlayer;

use crate::{core::*, world::LEVEL_COUNT};

#[derive(Resource, Reflect, Debug, Default)]
pub struct LevelDuration(pub Stopwatch);

#[derive(Resource, Reflect, Debug, Default)]
pub struct RunDuration {
    pub results: [Duration; LEVEL_COUNT],
}

pub struct DurationPlugin;

impl Plugin for DurationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelDuration::default())
            .insert_resource(RunDuration::default())
            .add_systems(Update, reset_timer);
    }
}

fn reset_timer(
    mut er: EventReader<Respawn<LogicalPlayer>>,
    mut timer: ResMut<LevelDuration>,
    history: Res<History>,
) {
    if !history.empty() {
        return;
    }

    for _ in er.read() {
        timer.0.reset();
    }
}
