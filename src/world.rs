use avian3d::prelude::*;
use bevy::core_pipeline::Skybox;
use bevy::{gltf::Gltf, prelude::*, scene::SceneInstanceReady};
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_hanabi::ParticleEffect;
use bevy_water::*;
use std::{f32::consts::TAU, num::NonZeroUsize};

use crate::prelude::*;

pub struct WorldPlugin;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Sun;

const WATER_HEIGHT: f32 = 10.0;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Sun>()
            .insert_resource(WaterSettings {
                height: WATER_HEIGHT,
                ..default()
            })
            .add_plugins(WaterPlugin)
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
                (
                    prop_colliders,
                    ground_colliders,
                    boost_colliders,
                    end_colliders,
                    checkpoint_colliders,
                    setup_water,
                    translate_water,
                )
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

fn setup(mut commands: Commands, mut window: Query<&mut Window>, assets: Res<AssetServer>) {
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

    commands.insert_resource(MainScene {
        levels: (1..=LEVEL_COUNT)
            .map(|x| assets.load(format!("level{:?}.glb", x)) as Handle<Gltf>)
            .collect::<Vec<Handle<Gltf>>>()
            .try_into()
            .unwrap(),
        skyboxes: (1..=LEVEL_COUNT)
            .map(|x| assets.load(format!("skybox_{:?}_skybox.ktx2", x)) as Handle<Image>)
            .collect::<Vec<Handle<Image>>>()
            .try_into()
            .unwrap(),
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

#[derive(Resource)]
struct MainScene {
    levels: [Handle<Gltf>; LEVEL_COUNT],
    is_spawned: bool,
    skyboxes: [Handle<Image>; LEVEL_COUNT],
}

impl MainScene {
    fn level(&self, level: NonZeroUsize) -> &Handle<Gltf> {
        &self.levels[level.get() - 1]
    }

    fn skybox(&self, level: NonZeroUsize) -> &Handle<Image> {
        &self.skyboxes[level.get() - 1]
    }
}

#[derive(Resource)]
pub struct CurrentLevel(pub NonZeroUsize);

impl CurrentLevel {
    pub fn get(&self) -> NonZeroUsize {
        self.0
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
                |trigger: Trigger<OnCollisionStart>,
                 mut cmd: Commands,
                 mut history: ResMut<History>,
                 current_lvl: Res<CurrentLevel>,
                 fx: Res<ParticleEffects>| {
                    history.0.push(trigger.target());

                    let other_entity = trigger.collider;

                    cmd.entity(other_entity).with_child((
                        ParticleEffect::new(fx.get_checkpoint_fx(current_lvl.get())),
                        Visibility::Visible,
                        Lifetime {
                            timer: Timer::from_seconds(2., TimerMode::Once),
                        },
                    ));
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
                 current_lvl: Res<CurrentLevel>,
                 mut ns: ResMut<NextState<AppState>>,
                 mut ew: EventWriter<SpawnLevel>| {
                    let next_level = current_lvl.get().get() + 1;

                    if next_level > LEVEL_COUNT {
                        ns.set(AppState::GameOver);
                        return;
                    }

                    ew.write(SpawnLevel(NonZeroUsize::new(next_level).unwrap()));
                },
            );
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
}

fn rotate_speed_boost(mut cubes: Query<&mut Transform, With<SpeedBoost>>, timer: Res<Time>) {
    for mut transform in &mut cubes {
        let rotation = TAU * timer.delta_secs();
        transform.rotate_x(0.1 * rotation);
        transform.rotate_z(0.1 * rotation);
        transform.rotate_y(0.5 * rotation);
    }
}
