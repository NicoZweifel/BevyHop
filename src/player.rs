use crate::core::*;

use std::f32::consts::TAU;

use avian_pickup::actor::{AvianPickupActor, AvianPickupActorHoldConfig};
use avian3d::prelude::*;
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::camera::Exposure,
};
use bevy_fps_controller::controller::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, respawn);
    }
}

fn setup(mut cmd: Commands) {
    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let height = 3.0;
    let logical_entity = cmd
        .spawn((
            Collider::cylinder(1.0, height),
            // A capsule can be used but is NOT recommended
            // If you use it, you have to make sure each segment point is
            // equidistant from the translation of the player transform
            // Collider::capsule(0.5, height),
            (
                Friction {
                    dynamic_coefficient: 0.0,
                    static_coefficient: 0.0,
                    combine_rule: CoefficientCombine::Min,
                },
                Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombine::Min,
                },
                LinearVelocity::ZERO,
                TransformInterpolation,
                RigidBody::Dynamic,
                CollisionLayers::new(
                    CollisionLayer::Player,
                    [CollisionLayer::Default, CollisionLayer::Boost],
                ),
                Sleeping,
                LockedAxes::ROTATION_LOCKED,
                Mass(1.0),
                GravityScale(0.0),
                Dominance(5),
            ),
            Transform::from_translation(SPAWN_OFFSET),
            LogicalPlayer,
            (NotShadowCaster, NotShadowReceiver),
            (
                FpsControllerInput {
                    pitch: -TAU / 12.0,
                    yaw: TAU * 5.0 / 8.0,
                    ..default()
                },
                FpsController {
                    air_acceleration: 20.,
                    max_air_speed: 100.,
                    air_speed_cap: 10.,
                    friction: 10.,
                    ..default()
                },
            ),
            CollisionEventsEnabled,
        ))
        .insert(CameraConfig {
            height_offset: -0.5,
        })
        .id();

    cmd.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: TAU / 5.0,
            ..default()
        }),
        Exposure::SUNLIGHT,
        RenderPlayer { logical_entity },
        AvianPickupActor {
            interaction_distance: 7.,
            prop_filter: SpatialQueryFilter::from_mask([
                CollisionLayer::Prop,
                CollisionLayer::Boost,
            ]),
            actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Player),
            obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Default),
            hold: AvianPickupActorHoldConfig {
                // Make sure the prop is far enough away from
                // our collider when looking straight down
                pitch_range: -50.0_f32.to_radians()..=75.0_f32.to_radians(),
                ..default()
            },
            ..default()
        },
    ));
}

fn respawn(
    mut query: Query<(&mut Transform, &mut LinearVelocity), With<LogicalPlayer>>,
    history: Res<History>,
    q_gtf: Query<&GlobalTransform, With<CheckPoint>>,
) {
    let spawn_point = if let Some(check_point) = history.0.last() {
        if let Ok(gtf) = q_gtf.get(*check_point) {
            gtf.translation() + SPAWN_OFFSET
        } else {
            SPAWN_OFFSET
        }
    } else {
        SPAWN_OFFSET
    };

    for (mut transform, mut velocity) in &mut query {
        if (spawn_point.y - transform.translation.y).abs() < 100. {
            continue;
        }

        velocity.0 = Vec3::ZERO;
        transform.translation = spawn_point
    }
}
