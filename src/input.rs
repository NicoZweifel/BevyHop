use bevy::{input::mouse::MouseWheel, prelude::*, window::CursorGrabMode};

use bevy_egui::{EguiContexts, EguiPreUpdateSet};
use bevy_fps_controller::controller::*;

use avian_pickup::prelude::*;

use crate::{core::*, state::*};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AvianPickupPlugin::default(), FpsControllerPlugin))
            .add_systems(
                PreUpdate,
                (block_mouse_input, block_keyboard_input)
                    .after(EguiPreUpdateSet::ProcessInput)
                    .before(EguiPreUpdateSet::BeginPass),
            )
            .add_systems(
                Update,
                (
                    manage_cursor,
                    scroll_events,
                    handle_reset.before(respawn::<LogicalPlayer>),
                )
                    .in_set(GameplaySet),
            )
            .add_systems(
                RunFixedMainLoop,
                handle_input
                    .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop)
                    .in_set(GameplaySet),
            );
    }
}

fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
    mut ns: ResMut<NextState<PausedState>>,
) {
    for mut window in &mut window_query {
        if btn.just_pressed(MouseButton::Left) {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
            for mut controller in &mut controller_query {
                controller.enable_input = true;
            }
        }

        if key.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
            for mut controller in &mut controller_query {
                controller.enable_input = false;
            }

            ns.set(PausedState::Paused);
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

fn handle_input(
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

        if keys.pressed(MouseButton::Right) {}
    }
}

fn handle_reset(
    keys: Res<ButtonInput<KeyCode>>,
    mut ew: EventWriter<Respawn<LogicalPlayer>>,
    mut history: ResMut<History>,
    q_gtf: Query<&GlobalTransform, With<CheckPoint>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        if keys.pressed(KeyCode::ShiftLeft) {
            history.0.clear();
        };

        let spawn_point = history.last(q_gtf);

        ew.write(Respawn::<LogicalPlayer>::new(spawn_point));
    }
}

pub fn block_mouse_input(mut mouse: ResMut<ButtonInput<MouseButton>>, mut contexts: EguiContexts) {
    let Some(context) = contexts.try_ctx_mut() else {
        return;
    };

    if context.is_pointer_over_area() || context.wants_pointer_input() {
        mouse.reset_all();
    }
}

pub fn block_keyboard_input(
    mut keyboard_keycode: ResMut<ButtonInput<KeyCode>>,
    mut contexts: EguiContexts,
) {
    let Some(context) = contexts.try_ctx_mut() else {
        return;
    };

    if context.wants_keyboard_input() {
        keyboard_keycode.reset_all();
    }
}
