use bevy::prelude::*;
use bevy_console::*;
use clap::Parser;

use crate::state::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_console::ConsolePlugin)
            .insert_resource(ConsoleConfiguration::default())
            .add_console_command::<ExampleCommand, _>(example_command)
            .add_console_command::<DebugCommand, _>(debug)
            .add_console_command::<PauseCommand, _>(pause);
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
        reply!(log, "hello {msg}");
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "debug")]
struct DebugCommand {}

fn debug(
    mut log: ConsoleCommand<DebugCommand>,
    s: Res<State<DebugState>>,
    mut ew: EventWriter<DebugToggle>,
) {
    let Some(Ok(DebugCommand {})) = log.take() else {
        return;
    };

    match s.get() {
        DebugState::Disabled => {
            reply!(log, "Enable Debug");
        }
        DebugState::Enabled => {
            reply!(log, "Disable Debug");
        }
    }

    ew.write(DebugToggle);
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "pause")]
struct PauseCommand {}

fn pause(
    mut log: ConsoleCommand<PauseCommand>,
    s: Res<State<PausedState>>,
    mut ew: EventWriter<PauseToggle>,
) {
    let Some(Ok(PauseCommand {})) = log.take() else {
        return;
    };

    match s.get() {
        PausedState::Paused => {
            reply!(log, "Resuming");
        }
        PausedState::Running => {
            reply!(log, "Pausing");
        }
    }

    ew.write(PauseToggle);
}
