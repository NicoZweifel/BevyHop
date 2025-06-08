use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

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
    main_menu_layout(&mut cmd).with_children(|cmd| {
        cmd.spawn(NodeBuilder::new().get_card())
            .with_children(|cmd| {
                cmd.spawn(get_header(&text_resource));
                main_menu_content(cmd, &text_resource);
            });
    });
}

fn main_menu_layout<'a>(cmd: &'a mut Commands) -> EntityCommands<'a> {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));

    cmd.spawn((
        BackgroundColor(BACKGROUND),
        NodeBuilder::new().with_grow(true).get(),
        MainMenu,
    ))
}

fn main_menu_content(
    cmd: &mut RelatedSpawnerCommands<'_, ChildOf>,
    text_resource: &Res<TextResource>,
) {
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
    .observe(|_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
        ew.write(AppExit::Success);
    });
}

fn handle_play(_: Trigger<Pointer<Click>>, mut ns: ResMut<NextState<AppState>>) {
    ns.set(AppState::InGame);
}
