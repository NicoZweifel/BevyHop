use bevy::prelude::*;
use bevy_hanabi::*;

use crate::core::*;

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, cleanup_timed::<ParticleEffect>);
    }
}

pub(crate) fn setup(mut cmd: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    cmd.insert_resource(ParticleEffects {
        boost_effect: effects.add(setup_boost_effect()),
        boost_idle_effect: effects.add(setup_boost_idle_effect()),
        player_boost_effect: effects.add(setup_player_boost_effect()),
    });
}

pub(crate) fn setup_boost_effect() -> EffectAsset {
    let mut color_gradient1 = Gradient::new();

    color_gradient1.add_key(0.0, Vec4::new(0.283153, 0.708391, 0.141266, 0.8));
    color_gradient1.add_key(0.5, Vec4::new(0.14, 0.35, 0.07, 0.5));
    color_gradient1.add_key(1.0, Vec4::new(0.0, 0.1, 0., 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec3::splat(2.0));
    size_gradient1.add_key(0.1, Vec3::splat(0.4));
    size_gradient1.add_key(0.4, Vec3::splat(0.2));
    size_gradient1.add_key(1.0, Vec3::splat(0.));

    let writer = ExprWriter::new();

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.8)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.8).normal(writer.lit(2.)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add constant downward acceleration to simulate gravity
    let accel = writer.lit(Vec3::Y * -9.).expr();
    let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(6.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.).expr(),
        dimension: ShapeDimension::Volume,
    };

    // The velocity is random in any direction
    let center = writer.attr(Attribute::POSITION);

    // Give a bit of variation by randomizing the initial speed
    let speed = writer.lit(40.).uniform(writer.lit(80.));
    let dir = writer
        .rand(VectorType::VEC3F)
        .mul(writer.lit(2.0))
        .sub(writer.lit(1.0))
        .normalized();
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, (center + dir * speed).expr());

    let round = RoundModifier {
        roundness: writer.lit(1.0).expr(),
    };

    let orient = OrientModifier::new(OrientMode::ParallelCameraDepthPlane);

    let spawner = SpawnerSettings::once(512.0.into());

    EffectAsset::new(2048, spawner, writer.finish())
        .with_name("boost_effect")
        .with_simulation_space(SimulationSpace::Local)
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(update_drag)
        .update(update_accel)
        .render(round)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
            screen_space_size: false,
        })
        .render(orient)
}

pub(crate) fn setup_player_boost_effect() -> EffectAsset {
    let mut color_gradient1 = Gradient::new();

    color_gradient1.add_key(0.0, Vec4::new(0.283153, 0.708391, 0.141266, 0.8));
    color_gradient1.add_key(0.5, Vec4::new(0.14, 0.35, 0.07, 0.5));
    color_gradient1.add_key(1.0, Vec4::new(0.0, 0.1, 0., 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec3::splat(2.0));
    size_gradient1.add_key(0.1, Vec3::splat(0.4));
    size_gradient1.add_key(0.4, Vec3::splat(0.2));
    size_gradient1.add_key(1.0, Vec3::splat(0.));

    let writer = ExprWriter::new();

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.8)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.8).normal(writer.lit(1.)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add constant downward acceleration to simulate gravity
    let accel = writer.lit(Vec3::Y * -9.).expr();
    let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(6.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(2.).expr(),
        dimension: ShapeDimension::Volume,
    };

    // The velocity is random in any direction
    let center = writer.attr(Attribute::POSITION);

    // Give a bit of variation by randomizing the initial speed
    let speed = writer.lit(40.).uniform(writer.lit(80.));
    let dir = writer
        .rand(VectorType::VEC3F)
        .mul(writer.lit(2.0))
        .sub(writer.lit(1.0))
        .normalized();
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, (center + dir * speed).expr());

    let round = RoundModifier {
        roundness: writer.lit(1.0).expr(),
    };

    let orient = OrientModifier::new(OrientMode::ParallelCameraDepthPlane);

    let spawner = SpawnerSettings::once(256.0.into());

    let mut module = writer.finish();

    let tangent_accel =
        TangentAccelModifier::constant(&mut module, Vec3::ZERO, Vec3::new(0., 1., 1.), 60.);

    EffectAsset::new(2048, spawner, module)
        .with_name("player_boost_effect")
        .with_simulation_space(SimulationSpace::Local)
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(update_drag)
        .update(tangent_accel)
        .update(update_accel)
        .render(round)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
            screen_space_size: false,
        })
        .render(orient)
}

pub(crate) fn setup_boost_idle_effect() -> EffectAsset {
    let mut color_gradient1 = Gradient::new();

    color_gradient1.add_key(0.0, Vec4::new(0.283153, 0.708391, 0.141266, 0.5));
    color_gradient1.add_key(0.5, Vec4::new(0.14, 0.35, 0.07, 0.5));
    color_gradient1.add_key(1.0, Vec4::new(0.0, 0.1, 0., 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec3::splat(0.4));
    size_gradient1.add_key(0.1, Vec3::splat(0.2));
    size_gradient1.add_key(0.4, Vec3::splat(0.1));
    size_gradient1.add_key(1.0, Vec3::splat(0.));

    let writer = ExprWriter::new();

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.8)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.8).normal(writer.lit(1.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(6.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.5).expr(),
        dimension: ShapeDimension::Volume,
    };

    // The velocity is random in any direction
    let center = writer.attr(Attribute::POSITION);

    // Give a bit of variation by randomizing the initial speed
    let speed = writer.lit(1.).uniform(writer.lit(2.));
    let dir = writer
        .rand(VectorType::VEC3F)
        .mul(writer.lit(2.0))
        .sub(writer.lit(1.0))
        .normalized();
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, (center + dir * speed).expr());

    let round = RoundModifier {
        roundness: writer.lit(1.0).expr(),
    };

    let orient = OrientModifier::new(OrientMode::ParallelCameraDepthPlane);

    let spawner = SpawnerSettings::burst(6.0.into(), 0.1.into());

    let mut module = writer.finish();

    let tangent_accel =
        TangentAccelModifier::constant(&mut module, Vec3::ZERO, Vec3::new(0., -1., -1.), 20.);

    EffectAsset::new(512, spawner, module)
        .with_name("boost_idle_effect")
        .with_simulation_space(SimulationSpace::Local)
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(update_drag)
        .update(tangent_accel)
        .render(round)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
            screen_space_size: false,
        })
        .render(orient)
}
