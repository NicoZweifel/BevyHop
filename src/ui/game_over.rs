use bevy::prelude::*;

use crate::core::*;

use super::*;

#[derive(Component)]
struct GameOver;

pub struct GameOverPlugin;

#[derive(Component)]
struct GameOverMenu;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup_game_over_menu)
            .add_systems(
                OnExit(AppState::GameOver),
                (cleanup::<GameOverMenu>, cleanup::<Camera3d>),
            );
    }
}

fn setup_game_over_menu(mut cmd: Commands, text_resource: Res<TextResource>) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));

    cmd.spawn((
        NodeBuilder::new().with_grow(true).get(),
        GameOverMenu,
        BackgroundColor(BACKGROUND),
    ))
    .with_children(|cmd| {
        cmd.spawn(NodeBuilder::new().get_card())
            .with_children(|cmd| {
                cmd.spawn(get_header(&text_resource));

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Restart"), text_resource.get_button_text_props())],
                ))
                .observe(handle_restart);

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(
                        Text::new("Main Menu"),
                        text_resource.get_button_text_props()
                    )],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut ns: ResMut<NextState<AppState>>| {
                        ns.set(AppState::MainMenu);
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

fn handle_restart(
    _: Trigger<Pointer<Click>>,
    mut ns_paused_state: ResMut<NextState<PausedState>>,
    mut ns_app_state: ResMut<NextState<AppState>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    ns_paused_state.set(PausedState::Running);
    ns_app_state.set(AppState::InGame);

    for mut window in &mut window_query {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
}
