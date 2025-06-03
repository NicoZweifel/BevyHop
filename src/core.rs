use avian3d::{PhysicsPlugins, prelude::*};
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_hanabi::EffectAsset;
use bevy_skein::SkeinPlugin;

pub const SPAWN_POINT: Vec3 = Vec3::new(0.0, 8., 0.0);

#[derive(Debug, PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Prop,
    Boost,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Character {
    name: String,
}

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
pub struct SpeedBoost(pub f32);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct History(pub Vec<Entity>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct ParticleEffects {
    pub boost_effect: Handle<EffectAsset>,
    pub boost_idle_effect: Handle<EffectAsset>,
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(128.0))
        .register_type::<Character>()
        .register_type::<TransformInterpolation>()
        .register_type::<RigidBody>()
        .register_type::<ColliderConstructor>()
        .register_type::<CheckPoint>()
        .register_type::<SpeedBoost>()
        .register_type::<Ground>()
        .register_type::<Prop>();
    }
}

pub fn cleanup_timed<S: Component>(
    mut commands: Commands,
    mut q_values: Query<(Entity, &mut Lifetime), With<S>>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut q_values {
        lifetime.timer.tick(time.delta());
        if !lifetime.timer.just_finished() || commands.get_entity(entity).is_err() {
            continue;
        }
        commands.entity(entity).despawn();
    }
}
