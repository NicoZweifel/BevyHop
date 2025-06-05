use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};

use bevy::gltf::Gltf;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_hanabi::ParticleEffect;
use std::f32::consts::TAU;

use crate::core::*;
use crate::state::{AppState, GameplaySet};

pub struct WorldPlugin;

const LEVEL_COUNT: usize = 3;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLevel>()
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (spawn_level, spawn_world)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                (
                    prop_colliders,
                    ground_colliders,
                    boost_colliders,
                    end_colliders,
                    checkpoint_colliders,
                )
                    .after(spawn_world)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                (cleanup_timed::<SpeedBoost>).in_set(GameplaySet),
            )
            .add_systems(Update, rotate.in_set(GameplaySet))
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
            })
            .insert_resource(ClearColor(Color::linear_rgb(0.83, 0.96, 0.96)));
    }
}

fn setup(mut commands: Commands, mut window: Query<&mut Window>, assets: Res<AssetServer>) {
    let mut window = window.single_mut().unwrap();
    window.title = String::from("Bevy Hop");

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(MainScene {
        levels: (1..=LEVEL_COUNT)
            .map(|x| assets.load(format!("level{:?}.glb", x)) as Handle<Gltf>)
            .collect(),
        is_spawned: false,
        current_level: 1,
    });
}

#[derive(Resource)]
struct MainScene {
    levels: Vec<Handle<Gltf>>,
    is_spawned: bool,
    current_level: usize,
}

fn spawn_world(
    mut commands: Commands,
    mut main_scene: ResMut<MainScene>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    if main_scene.is_spawned {
        return;
    }

    let gltf = gltf_assets.get(&main_scene.levels[main_scene.current_level - 1]);

    if let Some(gltf) = gltf {
        let scene = gltf.scenes.first().unwrap().clone();
        commands.spawn(SceneRoot(scene));

        main_scene.is_spawned = true;
    }
}

fn prop_colliders(
    mut cmd: Commands,
    main_scene: Res<MainScene>,
    q_props: Query<Entity, (With<Prop>, Without<Ready>)>,
) {
    if !main_scene.is_spawned {
        return;
    }

    for prop in &q_props {
        cmd.entity(prop).insert((
            Ready,
            CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL),
            ColliderConstructor::ConvexHullFromMesh,
            TransformInterpolation,
            RigidBody::Dynamic,
        ));
    }
}

fn boost_colliders(
    mut cmd: Commands,
    main_scene: Res<MainScene>,
    q_boost: Query<(Entity, &MeshMaterial3d<StandardMaterial>), (With<SpeedBoost>, Without<Ready>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    effects: Res<ParticleEffects>,
) {
    if !main_scene.is_spawned {
        return;
    }

    for (boost, mat) in &q_boost {
        let material = materials.get_mut(mat).unwrap();
        material.unlit = true;

        cmd.entity(boost)
            .insert((
                Ready,
                CollisionLayers::new(
                    CollisionLayer::Boost,
                    [CollisionLayer::Player, CollisionLayer::Prop],
                ),
                ColliderConstructor::ConvexHullFromMesh,
                CollisionEventsEnabled,
            ))
            .observe(boost_collision)
            .with_child(ParticleEffect::new(effects.boost_idle_effect.clone()))
            .with_child(PointLight {
                color: Color::srgba(0.283153, 0.708391, 0.141266, 0.5),
                radius: 2.0,
                intensity: 5_000_000.0,
                ..default()
            });
    }
}

fn ground_colliders(
    mut cmd: Commands,
    main_scene: Res<MainScene>,
    q_ground: Query<Entity, (With<Ground>, Without<Ready>)>,
) {
    if !main_scene.is_spawned {
        return;
    }

    for ground in &q_ground {
        cmd.entity(ground).insert((
            Ready,
            CollisionLayers::new(CollisionLayer::Default, LayerMask::ALL),
            ColliderConstructor::TrimeshFromMesh,
            RigidBody::Static,
        ));
    }
}

fn checkpoint_colliders(
    mut cmd: Commands,
    main_scene: Res<MainScene>,
    q_checkpoint: Query<Entity, (With<CheckPoint>, Without<Ready>)>,
) {
    if !main_scene.is_spawned {
        return;
    }

    for checkpoint in &q_checkpoint {
        cmd.entity(checkpoint)
            .insert((
                Ready,
                CollisionLayers::new(CollisionLayer::Checkpoint, [CollisionLayer::Player]),
                ColliderConstructor::TrimeshFromMesh,
                CollisionEventsEnabled,
            ))
            .observe(
                |trigger: Trigger<OnCollisionStart>, mut history: ResMut<History>| {
                    history.0.push(trigger.target());
                },
            );
    }
}

fn end_colliders(
    mut cmd: Commands,
    main_scene: Res<MainScene>,
    q_end: Query<Entity, (With<End>, Without<Ready>)>,
) {
    if !main_scene.is_spawned {
        return;
    }

    for end in &q_end {
        cmd.entity(end)
            .insert((
                Ready,
                CollisionLayers::new(CollisionLayer::End, [CollisionLayer::Player]),
                ColliderConstructor::TrimeshFromMesh,
                CollisionEventsEnabled,
            ))
            .observe(
                |_: Trigger<OnCollisionStart>,
                 main_scene: Res<MainScene>,
                 mut ew: EventWriter<SpawnLevel>| {
                    ew.write(SpawnLevel(main_scene.current_level + 1));
                },
            );
    }
}

fn spawn_level(
    mut cmd: Commands,
    mut history: ResMut<History>,
    scene: Single<Entity, With<SceneRoot>>,
    mut main_scene: ResMut<MainScene>,
    mut er: EventReader<SpawnLevel>,
    mut q_player: Query<&mut Transform, With<LogicalPlayer>>,
) {
    let spawn_point = SPAWN_POINT;

    let scene = scene.into_inner();

    for e in er.read() {
        history.0.clear();

        main_scene.current_level = e.0;
        main_scene.is_spawned = false;

        cmd.entity(scene).despawn();

        for mut transform in &mut q_player {
            transform.translation = spawn_point
        }
    }
}

fn boost_collision(
    trigger: Trigger<OnCollisionEnd>,
    mut cmd: Commands,
    q_gtf: Query<&GlobalTransform>,
    fx: Res<ParticleEffects>,
    mut q_boosted: Query<&mut LinearVelocity>,
) {
    let boost = trigger.target();

    let other_entity = trigger.collider;

    let Ok(mut boosted) = q_boosted.get_mut(other_entity) else {
        return;
    };

    let boost_value = 1.2;

    boosted.0 *= Vec3::splat(boost_value).with_y(1.);

    let Ok(gtf) = q_gtf.get(boost) else {
        return;
    };

    cmd.entity(other_entity).with_child((
        ParticleEffect::new(fx.player_boost_effect.clone()),
        Visibility::Visible,
        Lifetime {
            timer: Timer::from_seconds(2., TimerMode::Once),
        },
    ));

    cmd.spawn((
        Visibility::Visible,
        ParticleEffect::new(fx.boost_effect.clone()),
        Transform::from_translation(gtf.translation()),
        Lifetime {
            timer: Timer::from_seconds(2., TimerMode::Once),
        },
    ));
}

fn rotate(mut cubes: Query<&mut Transform, With<SpeedBoost>>, timer: Res<Time>) {
    for mut transform in &mut cubes {
        let rotation = TAU * timer.delta_secs();
        transform.rotate_x(0.1 * rotation);
        transform.rotate_z(0.1 * rotation);
        transform.rotate_y(0.5 * rotation);
    }
}
