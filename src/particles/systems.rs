use bevy::prelude::*;
use bevy_hanabi::prelude::*;
// use bevy_particle_systems::*;

use crate::{laser::components::Laser, player::components::Player};

use super::components::{PlayerShipTrailParticles, ProjectileImpactParticles};

pub fn setup_projectile_impact_particle_system(
    _trigger: Trigger<OnAdd, Laser>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {

     // Set `spawn_immediately` to false to spawn on command with Spawner::reset()
     let spawner = Spawner::once(100.0.into(), false);
     let writer = ExprWriter::new();

    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
    gradient.add_key(1.0, Vec4::ZERO);

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.5).expr(),
        dimension: ShapeDimension::Surface,
    };

    let normal = writer.add_property("normal", Vec3::ZERO.into());
    let normal = writer.prop(normal);
 
    let tangent = writer.lit(Vec3::Z).cross(normal.clone());
    let spread = writer.rand(ScalarType::Float) * writer.lit(2.) - writer.lit(1.);
    let speed = writer.rand(ScalarType::Float) * writer.lit(1000.0);
    let velocity = (normal + tangent * spread * writer.lit(5.0)).normalized() * speed; 
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, velocity.expr());

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles will stay
    // alive forever, and new particles can't be spawned instead.
    let lifetime = writer.lit(0.1); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime.expr());

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        spawner,
        // Move the expression module into the asset
        writer.finish(),
    )
    .with_name("MyEffect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_asset = effects.add(effect);

    commands
        .spawn((
            ProjectileImpactParticles,
            ParticleEffectBundle::new(effect_asset),
            Name::new("projectile_impact_effect")
        ));
}

pub fn setup_player_ship_trail_particle_system(
    _trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>
) {

    // Set `spawn_immediately` to false to spawn on command with Spawner::reset()
    let spawner = Spawner::once(100.0.into(), false);
    let writer = ExprWriter::new();

   // Define a color gradient from red to transparent black
   let mut gradient = Gradient::new();
   gradient.add_key(0.0, Vec4::new(1., 1., 1., 1.));
   gradient.add_key(1.0, Vec4::ZERO);

   // On spawn, randomly initialize the position of the particle
   // to be over the surface of a sphere of radius 2 units.
   let init_pos = SetPositionSphereModifier {
       center: writer.lit(Vec3::ZERO).expr(),
       radius: writer.lit(0.5).expr(),
       dimension: ShapeDimension::Surface,
   };

   let thrust_vector = writer.add_property("thrust_vector", Vec3::ZERO.into());
   let thrust_vector = writer.prop(thrust_vector);

   let tangent = writer.lit(Vec3::Z).cross(thrust_vector.clone());
   let spread = writer.rand(ScalarType::Float) * writer.lit(0.2) - writer.lit(0.1);
   let speed = writer.rand(ScalarType::Float) * writer.lit(100.0);
   let velocity = (thrust_vector + tangent * spread).normalized() * speed; 
   let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, velocity.expr());

   // Initialize the total lifetime of the particle, that is
   // the time for which it's simulated and rendered. This modifier
   // is almost always required, otherwise the particles will stay
   // alive forever, and new particles can't be spawned instead.
   let lifetime = writer.lit(1.0); // literal value "10.0"
   let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime.expr());

   // Create the effect asset
   let effect = EffectAsset::new(
       // Maximum number of particles alive at a time
       32768,
       // Spawn at a rate of 5 particles per second
       spawner,
       // Move the expression module into the asset
       writer.finish(),
   )
   .with_name("MyEffect2")
   .init(init_pos)
   .init(init_vel)
   .init(init_lifetime)
   // Render the particles with a color gradient over their
   // lifetime. This maps the gradient key 0 to the particle spawn
   // time, and the gradient key 1 to the particle death (10s).
   .render(ColorOverLifetimeModifier { gradient });

   // Insert into the asset system
   let effect_asset = effects.add(effect);

   commands.spawn((
        PlayerShipTrailParticles,
        ParticleEffectBundle::new(effect_asset),
        Name::new("ship_trail_particle_system"),
    ));

}

pub fn setup_ship_asteroid_impact_particle_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // commands
    //     .spawn(ParticleSystemBundle {
    //         particle_system: ParticleSystem {
    //             max_particles: 1_000,
    //             texture: ParticleTexture::Sprite(asset_server.load("px.png")),
    //             spawn_rate_per_second: 2500.0.into(),
    //             initial_speed: JitteredValue::jittered(200.0, -50.0..50.0),
    //             lifetime: JitteredValue::jittered(0.30, -0.2..0.2),
    //             color: ColorOverTime::Gradient(Curve::new(vec![
    //                 CurvePoint::new(Color::from(YELLOW), 0.0),
    //                 CurvePoint::new(Color::rgba(1.0, 1.0, 0.0, 0.0), 1.0),
    //             ])),
    //             looping: true,
    //             system_duration_seconds: 10.0,
    //             ..ParticleSystem::default()
    //         },
    //         ..ParticleSystemBundle::default()
    //     })
    //     .insert(ShipDamageParticleSystem)
    //     .insert(Transform::default());
}
