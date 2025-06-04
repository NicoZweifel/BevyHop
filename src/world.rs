use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};

use bevy::gltf::Gltf;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_hanabi::ParticleEffect;
use std::f32::consts::TAU;

use crate::core::*;
use crate::state::GameplaySet;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_world,
                scene_colliders,
                cleanup_timed::<SpeedBoost>,
                rotate,
            )
                .in_set(GameplaySet),
        )
        .add_systems(Startup, setup)
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
        levels: (1..=2)
            .map(|x| assets.load(format!("level{:?}.glb", x)) as Handle<Gltf>)
            .collect(),
        is_loaded: false,
        is_spawned: false,
        current_level: 0,
    });
}

#[derive(Resource)]
struct MainScene {
    levels: Vec<Handle<Gltf>>,
    is_loaded: bool,
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

    let gltf = gltf_assets.get(&main_scene.levels[main_scene.current_level]);

    if let Some(gltf) = gltf {
        let scene = gltf.scenes.first().unwrap().clone();
        commands.spawn(SceneRoot(scene));

        main_scene.is_spawned = true;
    }
}

fn scene_colliders(
    mut cmd: Commands,
    mut main_scene: ResMut<MainScene>,
    q_props: Query<Entity, With<Prop>>,
    q_ground: Query<Entity, With<Ground>>,
    q_checkpoint: Query<Entity, With<CheckPoint>>,
    q_end: Query<Entity, With<End>>,
    q_boost: Query<(Entity, &MeshMaterial3d<StandardMaterial>), With<SpeedBoost>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    effects: Res<ParticleEffects>,
) {
    if !main_scene.is_spawned {
        return;
    }

    if main_scene.is_loaded {
        return;
    }

    for prop in &q_props {
        cmd.entity(prop).insert((
            CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL),
            ColliderConstructor::ConvexHullFromMesh,
            TransformInterpolation,
            RigidBody::Dynamic,
        ));
    }

    for (boost, mat) in &q_boost {
        let material = materials.get_mut(mat).unwrap();
        material.unlit = true;

        cmd.entity(boost)
            .insert((
                CollisionLayers::new(CollisionLayer::Boost, [CollisionLayer::Player]),
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

    for ground in &q_ground {
        // TODO separate and mark as ready individually (0 unmarked left = is_loaded = true)
        main_scene.is_loaded = true;

        cmd.entity(ground).insert((
            CollisionLayers::new(CollisionLayer::Default, LayerMask::ALL),
            ColliderConstructor::TrimeshFromMesh,
            RigidBody::Static,
        ));
    }

    for checkpoint in &q_checkpoint {
        cmd.entity(checkpoint)
            .insert(CollisionEventsEnabled)
            .observe(
                |trigger: Trigger<OnCollisionStart>,
                 mut history: ResMut<History>,
                 q_target: Query<&GlobalTransform, With<CheckPoint>>,
                 q_player: Query<&GlobalTransform, With<LogicalPlayer>>| {
                    let other_entity = trigger.collider;

                    let Ok(target_gtf) = q_target.get(trigger.target()) else {
                        return;
                    };

                    let Ok(player_gtf) = q_player.get(other_entity) else {
                        return;
                    };

                    if player_gtf.translation().y < target_gtf.translation().y {
                        return;
                    };

                    history.0.push(trigger.target());
                },
            );
    }

    for end in &q_end {
        cmd.entity(end).insert(CollisionEventsEnabled).observe(
            |trigger: Trigger<OnCollisionStart>,
             mut cmd: Commands,
             mut history: ResMut<History>,
             q_target: Query<&GlobalTransform, With<End>>,
             q_player: Query<&GlobalTransform, With<LogicalPlayer>>,
             scene: Single<Entity, With<SceneRoot>>,
             mut main_scene: ResMut<MainScene>| {
                let other_entity = trigger.collider;

                let Ok(target_gtf) = q_target.get(trigger.target()) else {
                    return;
                };

                let Ok(player_gtf) = q_player.get(other_entity) else {
                    return;
                };

                if player_gtf.translation().y < target_gtf.translation().y {
                    return;
                };

                history.0.clear();

                main_scene.current_level += 1;
                main_scene.is_spawned = false;
                main_scene.is_loaded = false;

                cmd.entity(scene.into_inner()).despawn();
            },
        );
    }
}

fn boost_collision(
    trigger: Trigger<OnCollisionEnd>,
    mut cmd: Commands,
    q_gtf: Query<&GlobalTransform>,
    effects: Res<ParticleEffects>,
    mut q_player: Query<&mut LinearVelocity, With<LogicalPlayer>>,
) {
    let boost = trigger.target();

    let other_entity = trigger.collider;

    let Ok(mut player) = q_player.get_mut(other_entity) else {
        return;
    };

    player.0 *= Vec3::splat(1.2);

    let Ok(gtf) = q_gtf.get(boost) else {
        return;
    };

    cmd.entity(other_entity).with_child((
        ParticleEffect::new(effects.player_boost_effect.clone()),
        Visibility::Visible,
        Lifetime {
            timer: Timer::from_seconds(2., TimerMode::Once),
        },
    ));
    cmd.spawn((
        Visibility::Visible,
        ParticleEffect::new(effects.boost_effect.clone()),
        Transform::from_translation(gtf.translation()),
        Lifetime {
            timer: Timer::from_seconds(2., TimerMode::Once),
        },
    ));
}

fn rotate(mut cubes: Query<&mut Transform, With<SpeedBoost>>, timer: Res<Time>) {
    for mut transform in &mut cubes {
        transform.rotate_x(0.1 * TAU * timer.delta_secs());
        transform.rotate_z(0.1 * TAU * timer.delta_secs());
        transform.rotate_y(0.5 * TAU * timer.delta_secs());
    }
}
