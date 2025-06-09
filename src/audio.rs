use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_fps_controller::controller::LogicalPlayer;

use crate::core::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, dive_sound)
            .add_systems(OnEnter(AppState::InGame), ocean_sound)
            .add_systems(OnExit(AppState::InGame), cleanup::<OceanSound>);
    }
}

fn setup(asset_server: Res<AssetServer>, mut cmd: Commands, mut loading: ResMut<AssetsLoading>) {
    let ocean_sound = asset_server.load("ocean_sound/ocean.mp3");
    let dive_sound = asset_server.load("dive_sound/dive.mp3");

    loading.0.push(ocean_sound.clone().into());
    loading.0.push(dive_sound.clone().into());

    cmd.insert_resource(Sounds {
        ocean_sound,
        dive_sound,
    });
}

#[derive(Component)]
pub struct OceanSound;

fn ocean_sound(mut cmd: Commands, sounds: Res<Sounds>) {
    cmd.spawn((
        OceanSound,
        AudioPlayer::new(sounds.ocean_sound.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(0.1),
            ..default()
        },
    ));
}

fn dive_sound(
    mut cmd: Commands,
    q: Query<&Transform, With<LogicalPlayer>>,
    mut er: EventReader<Respawn<LogicalPlayer>>,
    sounds: Res<Sounds>,
) {
    for e in er.read() {
        for tf in &q {
            if !is_out_of_bounds(tf.translation, e.translation) {
                continue;
            };

            cmd.spawn((
                AudioPlayer::new(sounds.dive_sound.clone()),
                PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::Linear(0.15),
                    ..default()
                },
            ));
        }
    }
}
