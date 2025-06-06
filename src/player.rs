use crate::{core::*, state::*};

use std::f32::consts::TAU;

use avian_pickup::actor::*;
use avian3d::prelude::*;
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{NotShadowCaster, NotShadowReceiver, VolumetricFog},
    prelude::*,
    render::{camera::Exposure, view::ColorGrading},
};
use bevy_fps_controller::controller::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                OnExit(AppState::InGame),
                (cleanup::<LogicalPlayer>, cleanup::<RenderPlayer>),
            );
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
                    [
                        CollisionLayer::Default,
                        CollisionLayer::Boost,
                        CollisionLayer::Checkpoint,
                        CollisionLayer::End,
                    ],
                ),
                Sleeping,
                LockedAxes::ROTATION_LOCKED,
                Mass(1.0),
                GravityScale(0.0),
                Dominance(5),
            ),
            (
                Visibility::Visible,
                Transform::from_translation(SPAWN_POINT),
            ),
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
        Camera {
            hdr: true,
            ..default()
        },
        Camera3d::default(),
        ColorGrading::default(),
        Bloom::NATURAL,
        Tonemapping::TonyMcMapface,
        VolumetricFog {
            ambient_intensity: 0.1,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: TAU / 5.0,
            ..default()
        }),
        Exposure::SUNLIGHT,
        RenderPlayer { logical_entity },
        Visibility::Visible,
        AvianPickupActor {
            interaction_distance: 5.,
            prop_filter: SpatialQueryFilter::from_mask([
                CollisionLayer::Prop,
                CollisionLayer::Boost,
            ]),
            actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Player),
            obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Default),
            throw: AvianPickupActorThrowConfig {
                linear_speed_range: 0.0..=10.0,
                ..default()
            },
            hold: AvianPickupActorHoldConfig {
                // Make sure the prop is far enough away from
                // our collider when looking straight down
                pitch_range: -50.0_f32.to_radians()..=75.0_f32.to_radians(),
                preferred_distance: 2.,
                ..default()
            },
            ..default()
        },
    ));
}
