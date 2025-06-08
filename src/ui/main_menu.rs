use bevy::prelude::*;

use crate::core::*;

use super::*;

#[derive(Component)]
struct MainMenu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                OnExit(AppState::MainMenu),
                (cleanup::<MainMenu>, cleanup::<Camera3d>),
            );
    }
}

fn setup_main_menu(mut cmd: Commands, text_resource: Res<TextResource>) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));

    cmd.spawn((
        BackgroundColor(BACKGROUND),
        NodeBuilder::new().with_grow(true).get(),
        MainMenu,
    ))
    .with_children(|cmd| {
        cmd.spawn(NodeBuilder::new().get_card())
            .with_children(|cmd| {
                cmd.spawn(get_header(&text_resource));

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Play"), text_resource.get_button_text_props())],
                ))
                .observe(handle_play);

                #[cfg(not(target_arch = "wasm32"))]
                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Quit"), text_resource.get_button_text_props(),)],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
                        ew.write(AppExit::Success);
                    },
                );
            });
    });
}

fn handle_play(
    _: Trigger<Pointer<Click>>,
    mut ns: ResMut<NextState<AppState>>,

    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    ns.set(AppState::InGame);

    for mut window in &mut window_query {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
}
