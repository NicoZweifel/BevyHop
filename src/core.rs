use std::{marker::PhantomData, num::NonZeroUsize};

use avian3d::{PhysicsPlugins, prelude::*};
use bevy::{
    asset::{AssetMetaCheck, LoadState},
    prelude::*,
};
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_hanabi::EffectAsset;
use bevy_skein::SkeinPlugin;

pub use crate::state::*;

pub const LEVEL_COUNT: usize = 3;
pub const SPAWN_POINT: Vec3 = Vec3::new(0.0, 8., 0.0);

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

impl AssetsLoading {
    pub fn get(&self, server: Res<AssetServer>) -> bool {
        self.0.iter().any(|x| match server.get_load_state(x.id()) {
            Some(x) => !matches!(x, LoadState::Loaded),
            None => true,
        })
    }
}

#[derive(Event)]
pub struct SpawnLevel(pub NonZeroUsize);

#[derive(Event)]
pub struct Respawn<S: Component> {
    pub translation: Vec3,
    _marker: PhantomData<S>,
}

impl<S: Component> Respawn<S> {
    pub fn new(translation: Vec3) -> Respawn<S> {
        Self {
            translation,
            ..default()
        }
    }
}

impl<S: Component> Default for Respawn<S> {
    fn default() -> Self {
        Self {
            translation: default(),
            _marker: default(),
        }
    }
}

#[derive(Debug, PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Prop,
    Boost,
    Checkpoint,
    End,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Character {
    name: String,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Ready;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Prop;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Ground;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct CheckPoint;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct End;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct SpeedBoost(pub f32);

#[derive(Resource, Debug, Default)]
pub struct History(pub Vec<Entity>);

impl History {
    pub fn last(&self, q_gtf: Query<&GlobalTransform, With<CheckPoint>>) -> Vec3 {
        if let Some(check_point) = self.0.last() {
            if let Ok(gtf) = q_gtf.get(*check_point) {
                let t = gtf.translation();
                return t.with_z(t.z + 4.);
            }
        };

        SPAWN_POINT
    }

    pub fn empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct ParticleEffects {
    pub boost_fx: Handle<EffectAsset>,
    pub boost_idle_fx: Handle<EffectAsset>,
    pub player_boost_fx: Handle<EffectAsset>,
    pub new_level_fx: [Handle<EffectAsset>; LEVEL_COUNT],
    pub checkpoint_fx: [Handle<EffectAsset>; LEVEL_COUNT],
}

impl ParticleEffects {
    pub fn get_new_level_fx(&self, lvl: NonZeroUsize) -> Handle<EffectAsset> {
        self.new_level_fx[lvl.get() - 1].clone()
    }

    pub fn get_checkpoint_fx(&self, lvl: NonZeroUsize) -> Handle<EffectAsset> {
        self.checkpoint_fx[lvl.get() - 1].clone()
    }
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLevel>()
            .insert_resource(AssetsLoading::default())
            .insert_resource(Time::<Fixed>::from_hz(128.0))
            .insert_resource(History::default())
            .register_type::<Prop>()
            .register_type::<Character>()
            .register_type::<TransformInterpolation>()
            .register_type::<RigidBody>()
            .register_type::<ColliderConstructor>()
            .register_type::<CheckPoint>()
            .register_type::<End>()
            .register_type::<SpeedBoost>()
            .register_type::<Ground>()
            .add_plugins((
                DefaultPlugins
                    .set(AssetPlugin {
                        // Wasm builds will check for meta files (that don't exist) if this isn't set.
                        // This causes errors and even panics in web builds on itch.
                        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                        meta_check: AssetMetaCheck::Never,
                        ..default()
                    })
                    .set(WindowPlugin {
                        primary_window: Window {
                            fit_canvas_to_parent: true,
                            ..default()
                        }
                        .into(),
                        ..default()
                    }),
                SkeinPlugin::default(),
                PhysicsPlugins::default(),
                PhysicsDebugPlugin::default(),
                UnitPlugin::<Prop>::default(),
                UnitPlugin::<LogicalPlayer>::default(),
            ));
    }
}

pub struct UnitPlugin<S> {
    _marker: PhantomData<S>,
}

impl<S: Component> Default for UnitPlugin<S> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<S: Component> Plugin for UnitPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_event::<Respawn<S>>()
            .add_systems(FixedUpdate, out_of_bounds::<S>.in_set(GameplaySet))
            .add_systems(PreUpdate, respawn::<S>.in_set(GameplaySet));
    }
}

pub fn cleanup_timed<S: Component>(
    mut cmd: Commands,
    mut q: Query<(Entity, &mut Lifetime), With<S>>,
    t: Res<Time>,
) {
    for (entity, mut lifetime) in &mut q {
        lifetime.timer.tick(t.delta());
        if !lifetime.timer.just_finished() {
            continue;
        }
        cmd.entity(entity).despawn();
    }
}

pub fn cleanup<S: Component>(mut cmd: Commands, q: Query<Entity, With<S>>) {
    for x in &q {
        cmd.entity(x).despawn();
    }
}

pub fn respawn<S: Component>(
    mut q: Query<(&mut Transform, &mut LinearVelocity), With<S>>,
    mut er: EventReader<Respawn<S>>,
) {
    for e in er.read() {
        for (mut transform, mut velocity) in &mut q {
            velocity.0 = Vec3::ZERO;
            transform.translation = e.translation;
        }
    }
}

pub fn out_of_bounds<S: Component>(
    q: Query<&Transform, With<S>>,
    history: Res<History>,
    q_gtf: Query<&GlobalTransform, With<CheckPoint>>,
    mut er: EventWriter<Respawn<S>>,
) {
    let spawn_point = history.last(q_gtf);

    for transform in &q {
        if !is_out_of_bounds(transform.translation, spawn_point) {
            continue;
        }

        er.write(Respawn::<S> {
            translation: spawn_point,
            ..default()
        });
    }
}

pub fn is_out_of_bounds(translation: Vec3, spawn_point: Vec3) -> bool {
    (spawn_point.y - translation.y).abs() >= 95.
}
