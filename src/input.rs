use bevy::{input::mouse::MouseWheel, prelude::*, window::CursorGrabMode};

use bevy_fps_controller::controller::*;

use avian_pickup::prelude::*;

use crate::core::*;

pub struct InputPlugin;

#[derive(Component)]
pub struct AutoJump;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AvianPickupPlugin::default(), FpsControllerPlugin))
            .add_systems(
                Update,
                (
                    manage_cursor,
                    scroll_events,
                    handle_auto_jump,
                    handle_reset.before(respawn::<LogicalPlayer>),
                )
                    .in_set(GameplaySet),
            )
            .add_systems(OnEnter(AppState::GameOver), enable_cursor)
            .add_systems(OnEnter(PausedState::Paused), enable_cursor)
            .add_systems(OnEnter(AppState::InGame), disable_cursor)
            .add_systems(
                OnEnter(PausedState::Running),
                disable_cursor.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                PreUpdate,
                auto_jump
                    .after(fps_controller_input)
                    .before(fps_controller_move),
            )
            .add_systems(
                RunFixedMainLoop,
                handle_pickup
                    .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop)
                    .in_set(GameplaySet),
            );
    }
}

fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    window_query: Query<&mut Window>,
    controller_query: Query<&mut FpsController>,
    mut ns: ResMut<NextState<PausedState>>,
) {
    if btn.just_pressed(MouseButton::Left) {
        disable_cursor(window_query, controller_query);
    }

    if key.just_pressed(KeyCode::Escape) {
        ns.set(PausedState::Paused);
    }
}

fn disable_cursor(
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    for mut window in &mut window_query {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
}

fn enable_cursor(
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    for mut window in &mut window_query {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}

fn scroll_events(mut er: EventReader<MouseWheel>) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in er.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
        }
    }
}

fn handle_pickup(
    mut ew: EventWriter<AvianPickupInput>,
    keys: Res<ButtonInput<MouseButton>>,
    actors: Query<Entity, With<AvianPickupActor>>,
) {
    for actor in &actors {
        if keys.just_pressed(MouseButton::Left) {
            ew.write(AvianPickupInput {
                action: AvianPickupAction::Throw,
                actor,
            });
        }

        if keys.just_pressed(MouseButton::Right) {
            ew.write(AvianPickupInput {
                action: AvianPickupAction::Drop,
                actor,
            });
        }

        if keys.pressed(MouseButton::Right) {
            ew.write(AvianPickupInput {
                action: AvianPickupAction::Pull,
                actor,
            });
        }
    }
}

fn auto_jump(mut q_input: Query<&mut FpsControllerInput, With<AutoJump>>) {
    for mut input in &mut q_input {
        input.jump = true;
    }
}

fn handle_auto_jump(
    mut cmd: Commands,
    q_player: Query<(Entity, Option<&AutoJump>), With<FpsControllerInput>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    {
        if !keys.just_pressed(KeyCode::Space) || !keys.pressed(KeyCode::ShiftLeft) {
            return;
        }

        for (player, auto_jump) in q_player {
            if auto_jump.is_some() {
                cmd.entity(player).remove::<AutoJump>();
                continue;
            };

            cmd.entity(player).insert(AutoJump);
        }
    }
}

fn handle_reset(
    keys: Res<ButtonInput<KeyCode>>,
    mut ew: EventWriter<Respawn<LogicalPlayer>>,
    mut history: ResMut<History>,
    q_gtf: Query<&GlobalTransform, With<CheckPoint>>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    if keys.pressed(KeyCode::ShiftLeft) {
        history.0.clear();
    };

    let spawn_point = history.last(q_gtf);

    ew.write(Respawn::<LogicalPlayer>::new(spawn_point));
}
