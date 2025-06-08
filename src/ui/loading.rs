use bevy::prelude::*;

use crate::core::*;

use super::*;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_loading_screen.after(setup_font))
            .add_systems(
                OnExit(AppState::Loading),
                (cleanup::<LoadingScreen>, cleanup::<Camera3d>),
            );
    }
}

#[derive(Component)]
struct LoadingScreen;

fn setup_loading_screen(mut cmd: Commands, text_resource: Res<TextResource>) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));

    cmd.spawn((
        NodeBuilder::new().with_grow(true).get(),
        LoadingScreen,
        BackgroundColor(BACKGROUND),
        children![
            get_header(&text_resource),
            (
                NodeBuilder::new().get_card(),
                children![(
                    Text::new("Loading..."),
                    text_resource.get_button_text_props()
                )],
            )
        ],
    ));
}
