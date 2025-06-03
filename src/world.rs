use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};

use bevy::gltf::Gltf;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_hanabi::ParticleEffect;

use crate::core::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_world, scene_colliders, cleanup_timed::<SpeedBoost>),
        )
        .add_systems(Startup, setup)
        .add_observer(
            // log the component from the gltf spawn
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
    window.title = String::from("Minimal FPS Controller Example");

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(MainScene {
        handle: assets.load("playground.glb"),
        is_loaded: false,
        is_spawned: false,
    });
}

#[derive(Resource)]
struct MainScene {
    handle: Handle<Gltf>,
    is_loaded: bool,
    is_spawned: bool,
}

fn spawn_world(
    mut commands: Commands,
    mut main_scene: ResMut<MainScene>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    if main_scene.is_spawned {
        return;
    }

    let gltf = gltf_assets.get(&main_scene.handle);

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
                RigidBody::Dynamic,
                CollisionEventsEnabled,
                Sleeping,
                LinearVelocity::ZERO,
                Dominance(1),
                AngularVelocity::ZERO,
                GravityScale(0.),
            ))
            .observe(
                |trigger: Trigger<OnCollisionEnd>,
                 mut cmd: Commands,
                 q_gtf: Query<&GlobalTransform>,
                 effects: Res<ParticleEffects>,
                 mut q_player: Query<&mut LinearVelocity, With<LogicalPlayer>>| {
                    let boost = trigger.target();

                    let other_entity = trigger.collider;

                    let Ok(mut player) = q_player.get_mut(other_entity) else {
                        return;
                    };

                    player.0 *= Vec3::splat(1.2);

                    cmd.entity(boost).despawn();

                    let Ok(gtf) = q_gtf.get(boost) else {
                        return;
                    };

                    cmd.spawn((
                        ParticleEffect::new(effects.boost_effect.clone()),
                        Transform::from_translation(gtf.translation()),
                        Lifetime {
                            timer: Timer::from_seconds(2., TimerMode::Once),
                        },
                    ));
                },
            )
            .with_child(ParticleEffect::new(effects.boost_idle_effect.clone()))
            .with_child(PointLight {
                color: Color::srgba(0.283153, 0.708391, 0.141266, 1.),
                radius: 0.4,
                intensity: 10_000_000.0,
                ..default()
            });
    }

    for ground in &q_ground {
        main_scene.is_loaded = true;

        cmd.entity(ground).insert((
            CollisionLayers::new(CollisionLayer::Default, LayerMask::ALL),
            ColliderConstructor::TrimeshFromMesh,
            RigidBody::Static,
        ));
    }
}
