use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use avian3d::prelude::*;
use bevy_hanabi::ParticleEffect;
use std::num::NonZeroUsize;

use super::*;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
        );
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
            .observe(checkpoint_collision);
    }
}

fn checkpoint_collision(
    trigger: Trigger<OnCollisionStart>,
    mut cmd: Commands,
    mut history: ResMut<History>,
    current_lvl: Res<CurrentLevel>,
    fx: Res<ParticleEffects>,
) {
    history.0.push(trigger.target());

    let other_entity = trigger.collider;

    cmd.entity(other_entity).with_child((
        ParticleEffect::new(fx.get_checkpoint_fx(current_lvl.get())),
        Visibility::Visible,
        Lifetime {
            timer: Timer::from_seconds(2., TimerMode::Once),
        },
    ));
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
            .observe(end_collision);
    }
}

fn end_collision(
    _: Trigger<OnCollisionStart>,
    current_lvl: Res<CurrentLevel>,
    mut ns: ResMut<NextState<AppState>>,
    mut ew: EventWriter<SpawnLevel>,
) {
    let next_level = current_lvl.get().get() + 1;

    if next_level > LEVEL_COUNT {
        ns.set(AppState::GameOver);
        return;
    }

    ew.write(SpawnLevel(NonZeroUsize::new(next_level).unwrap()));
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
                children![
                    ParticleEffect::new(effects.boost_idle_fx.clone()),
                    PointLight {
                        color: Resurrect64::GREEN,
                        radius: 3.0,
                        intensity: 3_000_000.0,
                        shadows_enabled: false,
                        ..default()
                    }
                ],
            ))
            .observe(boost_collision);
    }
}

fn boost_collision(
    trigger: Trigger<OnCollisionEnd>,
    mut cmd: Commands,
    q_gtf: Query<&GlobalTransform>,
    fx: Res<ParticleEffects>,
    mut q_boosted: Query<&mut LinearVelocity>,
    sounds: Res<Sounds>,
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
        ParticleEffect::new(fx.player_boost_fx.clone()),
        Visibility::Visible,
        Lifetime {
            timer: Timer::from_seconds(2., TimerMode::Once),
        },
    ));

    cmd.spawn((
        Visibility::Visible,
        ParticleEffect::new(fx.boost_fx.clone()),
        Transform::from_translation(gtf.translation()),
        Lifetime {
            timer: Timer::from_seconds(2., TimerMode::Once),
        },
    ));

    cmd.spawn((
        AudioPlayer::new(sounds.boost_sound.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::Linear(0.2),
            ..default()
        },
    ));
}
