use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, display_text);
    }
}

fn setup(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        Text(String::from("")),
        TextFont {
            font: assets.load("fira_mono.ttf"),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
    ));
}

fn display_text(
    mut controller_query: Query<(&Transform, &LinearVelocity), With<LogicalPlayer>>,
    mut text_query: Query<&mut Text>,
) {
    for (transform, velocity) in &mut controller_query {
        for mut text in &mut text_query {
            text.0 = format!(
                "vel: {:.2}, {:.2}, {:.2}\npos: {:.2}, {:.2}, {:.2}\nspd: {:.2}",
                velocity.0.x,
                velocity.0.y,
                velocity.0.z,
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
                velocity.0.xz().length()
            );
        }
    }
}
