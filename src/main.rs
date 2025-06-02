use avian_pickup::{
    AvianPickupPlugin,
    actor::{AvianPickupActor, AvianPickupActorHoldConfig},
    input::{AvianPickupAction, AvianPickupInput},
};
use bevy::{input::mouse::MouseWheel, prelude::*, scene::SceneInstanceReady};
use bevy_skein::SkeinPlugin;
use std::f32::consts::TAU;

use avian3d::prelude::*;
use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    math::Vec3Swizzles,
    prelude::*,
    render::camera::Exposure,
    window::CursorGrabMode,
};

use bevy_fps_controller::controller::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Character {
    name: String,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Prop;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Ground;

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 8., 0.0);

fn scroll_events(mut evr_scroll: EventReader<MouseWheel>) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(128.0))
        .register_type::<Character>()
        .register_type::<TransformInterpolation>()
        .register_type::<RigidBody>()
        .register_type::<ColliderConstructor>()
        .register_type::<Ground>()
        .register_type::<Prop>()
        .add_plugins((
            DefaultPlugins,
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
            //PhysicsDebugPlugin::default(),
            AvianPickupPlugin::default(),
            FpsControllerPlugin,
        ))
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
        .insert_resource(ClearColor(Color::linear_rgb(0.83, 0.96, 0.96)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                manage_cursor,
                spawn_world,
                scene_colliders,
                display_text,
                respawn,
                scroll_events,
            ),
        )
        .add_systems(
            RunFixedMainLoop,
            handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
        )
        .run();
}

#[derive(Debug, PhysicsLayer, Default)]
enum CollisionLayer {
    #[default]
    Default,
    Player,
    Prop,
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

    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let height = 3.0;
    let logical_entity = commands
        .spawn((
            Collider::cylinder(1.0, height),
            // A capsule can be used but is NOT recommended
            // If you use it, you have to make sure each segment point is
            // equidistant from the translation of the player transform
            // Collider::capsule(0.5, height),
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
            RigidBody::Dynamic,
            CollisionLayers::new(CollisionLayer::Player, CollisionLayer::Default),
            Sleeping,
            LockedAxes::ROTATION_LOCKED,
            Mass(1.0),
            GravityScale(0.0),
            Transform::from_translation(SPAWN_POINT),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                air_acceleration: 60.,
                max_air_speed: 60.,
                air_speed_cap: 2.5,
                friction: 10.,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: -0.5,
        })
        .id();

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: TAU / 5.0,
            ..default()
        }),
        Exposure::SUNLIGHT,
        RenderPlayer { logical_entity },
        AvianPickupActor {
            prop_filter: SpatialQueryFilter::from_mask(CollisionLayer::Prop),
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

    commands.insert_resource(MainScene {
        handle: assets.load("playground.glb"),
        is_loaded: false,
        is_spawned: false,
    });

    commands.spawn((
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

/// Pass player input along to `avian_pickup`
fn handle_input(
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    key_input: Res<ButtonInput<MouseButton>>,
    actors: Query<Entity, With<AvianPickupActor>>,
) {
    for actor in &actors {
        if key_input.just_pressed(MouseButton::Left) {
            avian_pickup_input_writer.write(AvianPickupInput {
                action: AvianPickupAction::Throw,
                actor,
            });
        }
        if key_input.just_pressed(MouseButton::Right) {
            avian_pickup_input_writer.write(AvianPickupInput {
                action: AvianPickupAction::Drop,
                actor,
            });
        }
        if key_input.pressed(MouseButton::Right) {
            avian_pickup_input_writer.write(AvianPickupInput {
                action: AvianPickupAction::Pull,
                actor,
            });
        }
    }
}

fn respawn(mut query: Query<(&mut Transform, &mut LinearVelocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.0 = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
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
            TransformInterpolation,
            ColliderConstructor::ConvexHullFromMesh,
            RigidBody::Dynamic,
        ));
    }

    for ground in &q_ground {
        cmd.entity(ground).insert((
            ColliderConstructor::TrimeshFromMesh,
            CollisionLayers::new(CollisionLayer::Default, LayerMask::ALL),
            RigidBody::Static,
        ));
    }

    main_scene.is_loaded = true;
}

fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    for mut window in &mut window_query {
        if btn.just_pressed(MouseButton::Left) {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
            for mut controller in &mut controller_query {
                controller.enable_input = true;
            }
        }
        if key.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
            for mut controller in &mut controller_query {
                controller.enable_input = false;
            }
        }
    }
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
