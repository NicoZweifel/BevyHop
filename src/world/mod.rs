mod collider;
mod core;

pub use core::*;

use bevy::core_pipeline::Skybox;
use bevy::{gltf::Gltf, prelude::*, scene::SceneInstanceReady};
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_hanabi::ParticleEffect;
use bevy_water::*;
use collider::ColliderPlugin;
use std::{f32::consts::TAU, num::NonZeroUsize};

use crate::prelude::*;

pub struct WorldPlugin;

const WATER_HEIGHT: f32 = 10.0;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Sun>()
            .insert_resource(WaterSettings {
                height: WATER_HEIGHT,
                ..default()
            })
            .add_plugins((WaterPlugin, ColliderPlugin))
            .add_event::<SpawnLevel>()
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (spawn_level, spawn_world)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                (setup_water, translate_water)
                    .after(spawn_world)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                (cleanup_timed::<SpeedBoost>).in_set(GameplaySet),
            )
            .add_systems(
                OnExit(AppState::InGame),
                (cleanup::<SceneRoot>, reset_world),
            )
            .add_systems(Update, rotate_speed_boost.in_set(GameplaySet))
            .add_observer(
                |trigger: Trigger<SceneInstanceReady>,
                 children: Query<&Children>,
                 characters: Query<&Character>| {
                    for entity in children.iter_descendants(trigger.target()) {
                        let Ok(character) = characters.get(entity) else {
                            continue;
                        };
                        info!(?character);
                    }
                },
            )
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 10000.0,
                affects_lightmapped_meshes: true,
            });
    }
}

fn setup(
    mut commands: Commands,
    mut window: Query<&mut Window>,
    assets: Res<AssetServer>,

    mut loading: ResMut<AssetsLoading>,
) {
    let mut window = window.single_mut().unwrap();
    window.title = String::from("Bevy Hop");

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::DIRECT_SUNLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let levels: [Handle<Gltf>; LEVEL_COUNT] = (1..=LEVEL_COUNT)
        .map(|x| assets.load(format!("level{:?}.glb", x)) as Handle<Gltf>)
        .collect::<Vec<Handle<Gltf>>>()
        .try_into()
        .unwrap();

    let skyboxes: [Handle<Image>; LEVEL_COUNT] = (1..=LEVEL_COUNT)
        .map(|x| assets.load(format!("skybox/skybox_{:?}_skybox.ktx2", x)) as Handle<Image>)
        .collect::<Vec<Handle<Image>>>()
        .try_into()
        .unwrap();

    levels
        .iter()
        .map(|x| x.clone().into())
        .for_each(|x| loading.0.push(x));

    skyboxes
        .iter()
        .map(|x| x.clone().into())
        .for_each(|x| loading.0.push(x));

    commands.insert_resource(MainScene {
        levels,
        skyboxes,
        is_spawned: false,
    });

    commands.insert_resource(CurrentLevel(NonZeroUsize::MIN));
}

fn reset_world(mut world: ResMut<MainScene>, mut current_level: ResMut<CurrentLevel>) {
    world.is_spawned = false;
    current_level.0 = NonZeroUsize::MIN;
}

fn setup_water(mut q_water: Query<&mut Transform, (With<WaterTiles>, Without<Ready>)>) {
    for mut water in &mut q_water {
        water.scale = Vec3::splat(8.);
    }
}

fn translate_water(
    mut q_water: Query<&mut Transform, With<WaterTiles>>,
    history: Res<History>,
    q_gtf: Query<&GlobalTransform, With<CheckPoint>>,
) {
    let spawn_point = history.last(q_gtf);
    for mut water in &mut q_water {
        water.translation.y = spawn_point.y - 170.;
    }
}

fn spawn_world(
    mut cmd: Commands,
    mut main_scene: ResMut<MainScene>,
    current_level: ResMut<CurrentLevel>,
    gltf_assets: Res<Assets<Gltf>>,
    q_camera: Query<Entity, With<Camera3d>>,
    mut water_settings: ResMut<WaterSettings>,
    q_player: Query<Entity, With<LogicalPlayer>>,
    fx: Res<ParticleEffects>,
) {
    if main_scene.is_spawned {
        return;
    }

    let gltf = gltf_assets.get(main_scene.level(current_level.get()));

    if let Some(gltf) = gltf {
        let scene = gltf.scenes.first().unwrap().clone();
        cmd.spawn(SceneRoot(scene));

        main_scene.is_spawned = true;
    }

    let skybox_handle = main_scene.skybox(current_level.get());
    for entity in &q_camera {
        cmd.entity(entity).remove::<Skybox>().insert(Skybox {
            image: skybox_handle.clone(),
            brightness: match current_level.get().get() {
                1 => 30000.,
                2 => 50000.,
                3 => 50000.,
                _ => 10000.,
            },
            ..default()
        });
    }

    water_settings.deep_color = match current_level.get().get() {
        1 => Resurrect64::DEEP_PURPLE,
        2 => Resurrect64::DARK_CYAN,
        3 => Resurrect64::DARK_RED_1,
        _ => Resurrect64::DARK_CYAN,
    };

    for player in &q_player {
        cmd.entity(player).with_child((
            ParticleEffect::new(fx.get_new_level_fx(current_level.get())),
            Visibility::Visible,
            Lifetime {
                timer: Timer::from_seconds(2., TimerMode::Once),
            },
        ));
    }
}

fn spawn_level(
    mut cmd: Commands,
    mut history: ResMut<History>,
    scene: Single<Entity, With<SceneRoot>>,
    level_duration: Res<LevelDuration>,
    mut run_duration: ResMut<RunDuration>,
    mut current_level: ResMut<CurrentLevel>,
    mut main_scene: ResMut<MainScene>,
    mut er: EventReader<SpawnLevel>,
    mut q_player: Query<&mut Transform, With<LogicalPlayer>>,
) {
    let spawn_point = SPAWN_POINT;

    let scene = scene.into_inner();

    for e in er.read() {
        history.0.clear();

        run_duration.results[current_level.get().get() - 1] = level_duration.0.elapsed();

        current_level.0 = e.0;
        main_scene.is_spawned = false;

        cmd.entity(scene).despawn();

        for mut transform in &mut q_player {
            transform.translation = spawn_point;
        }
    }
}

fn rotate_speed_boost(mut cubes: Query<&mut Transform, With<SpeedBoost>>, timer: Res<Time>) {
    for mut transform in &mut cubes {
        let rotation = TAU * timer.delta_secs();
        transform.rotate_x(0.1 * rotation);
        transform.rotate_z(0.1 * rotation);
        transform.rotate_y(0.5 * rotation);
    }
}
