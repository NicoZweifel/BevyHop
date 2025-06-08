use bevy::prelude::*;

use crate::core::*;

use super::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PausedState::Paused), setup_pause_menu)
            .add_systems(OnExit(PausedState::Paused), cleanup::<PauseMenu>);
    }
}

#[derive(Component)]
struct PauseMenu;

fn setup_pause_menu(
    mut cmd: Commands,
    debug_state: Res<State<DebugState>>,
    text_resource: Res<TextResource>,
) {
    cmd.spawn((
        NodeBuilder::new().with_grow(true).get(),
        PauseMenu,
        BackgroundColor(BACKGROUND.with_alpha(match debug_state.get() {
            DebugState::Disabled => 0.5,
            DebugState::Enabled => 0.,
        })),
    ))
    .with_children(|cmd| {
        cmd.spawn(NodeBuilder::new().get_card())
            .with_children(|cmd| {
                cmd.spawn(get_header(&text_resource));

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Resume"), text_resource.get_button_text_props(),)],
                ))
                .observe(handle_resume);

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(
                        Text::new("Main Menu"),
                        text_resource.get_button_text_props()
                    )],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut ns_app_state: ResMut<NextState<AppState>>| {
                        ns_app_state.set(AppState::MainMenu);
                    },
                );

                #[cfg(not(target_arch = "wasm32"))]
                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Quit"), text_resource.get_button_text_props())],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
                        ew.write(AppExit::Success);
                    },
                );
            });
    });
}

fn handle_resume(
    _: Trigger<Pointer<Click>>,
    mut ns: ResMut<NextState<PausedState>>,

    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    ns.set(PausedState::Running);

    for mut window in &mut window_query {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
}
