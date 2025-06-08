use avian3d::prelude::*;
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                MainMenuSet.run_if(in_state(AppState::MainMenu)),
                GameplaySet
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(PausedState::Running)),
            ),
        )
        .configure_sets(
            FixedUpdate,
            (
                GameplaySet
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(PausedState::Running)),
                SingleplayerSet.run_if(in_state(GameModeState::Singleplayer)),
                MultiplayerSet
                    .in_set(GameplaySet)
                    .run_if(in_state(GameModeState::Multiplayer)),
            ),
        )
        .insert_state(AppState::Loading)
        .init_state::<GameModeState>()
        .init_state::<PausedState>()
        .init_state::<DebugState>()
        .add_systems(OnExit(AppState::InGame), resume)
        .add_systems(OnEnter(PausedState::Paused), pause_physics)
        .add_systems(OnEnter(PausedState::Running), resume_physics)
        .add_systems(OnEnter(DebugState::Enabled), start_physics_debug)
        .add_systems(OnEnter(DebugState::Disabled), stop_physics_debug);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainMenuSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameplaySet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingleplayerSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultiplayerSet;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
    Loading,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameModeState {
    #[default]
    NotInGame,
    Singleplayer,
    Multiplayer,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState {
    Paused,
    #[default]
    Running,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DebugState {
    #[default]
    Disabled,
    Enabled,
}

fn resume(mut ns: ResMut<NextState<PausedState>>) {
    ns.set(PausedState::Running);
}

fn pause_physics(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn resume_physics(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}

fn start_physics_debug(mut store: ResMut<GizmoConfigStore>) {
    let cfg = store.config_mut::<PhysicsGizmos>();
    cfg.0.enabled = true;
}

fn stop_physics_debug(mut store: ResMut<GizmoConfigStore>) {
    let cfg = store.config_mut::<PhysicsGizmos>();
    cfg.0.enabled = false;
}
