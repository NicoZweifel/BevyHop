use std::num::NonZeroUsize;

use bevy::prelude::*;
use bevy_console::*;
use bevy_fps_controller::controller::*;
use clap::Parser;

use crate::prelude::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleConfiguration::default())
            .add_console_command::<ExampleCommand, _>(example_command)
            .add_console_command::<LevelCommand, _>(level)
            .add_console_command::<DebugCommand, _>(debug)
            .add_console_command::<PauseCommand, _>(pause)
            .add_console_command::<NoClipCommand, _>(noclip);
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "hello")]
struct ExampleCommand {
    #[arg(index = 1, default_value = "darkness, my old friend")]
    msg: String,
}

fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        reply!(log, "Hello {msg}");
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "level")]
struct LevelCommand {
    #[arg(index = 1, default_value_t = 1)]
    level: usize,
}

fn level(mut log: ConsoleCommand<LevelCommand>, mut ew: EventWriter<SpawnLevel>) {
    if let Some(Ok(LevelCommand { level })) = log.take() {
        reply!(log, "Loading Level {level}");

        let Some(level) = NonZeroUsize::new(level) else {
            reply!(log, "Level must be greather than 0!");
            return;
        };

        if level.get() > LEVEL_COUNT {
            reply!(
                log,
                "Level {level} does not exist! The MAX Level is {LEVEL_COUNT}."
            );
            return;
        };

        ew.write(SpawnLevel(level));
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "noclip")]
struct NoClipCommand {}

fn noclip(mut log: ConsoleCommand<NoClipCommand>, mut q_controller: Query<&mut FpsController>) {
    let Some(Ok(NoClipCommand {})) = log.take() else {
        return;
    };

    for mut controller in &mut q_controller {
        controller.move_mode = match controller.move_mode {
            MoveMode::Noclip => MoveMode::Ground,
            MoveMode::Ground => MoveMode::Noclip,
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "debug")]
struct DebugCommand {}

fn debug(
    mut log: ConsoleCommand<DebugCommand>,
    s: Res<State<DebugState>>,
    mut ns: ResMut<NextState<DebugState>>,
) {
    let Some(Ok(DebugCommand {})) = log.take() else {
        return;
    };

    ns.set(match s.get() {
        DebugState::Disabled => {
            reply!(log, "Debug Enabled!");
            DebugState::Enabled
        }
        DebugState::Enabled => {
            reply!(log, "Debug Disabled!");
            DebugState::Disabled
        }
    })
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "pause")]
struct PauseCommand {}

fn pause(
    mut log: ConsoleCommand<PauseCommand>,
    s: Res<State<PausedState>>,
    mut ns: ResMut<NextState<PausedState>>,
) {
    let Some(Ok(PauseCommand {})) = log.take() else {
        return;
    };

    ns.set(match s.get() {
        PausedState::Paused => {
            reply!(log, "Resuming!");
            PausedState::Running
        }
        PausedState::Running => {
            reply!(log, "Pausing!");
            PausedState::Paused
        }
    })
}
