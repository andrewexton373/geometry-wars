use bevy::prelude::*;

use crate::player::components::Player;
use bevy_particle_systems::*;

pub struct ParticlePlugin;

#[derive(Component)]
pub struct PlayerShipTrailParticles;

#[derive(Component)]
pub struct ProjectileImpactParticles;

#[derive(Component)]
pub struct ShipAsteroidImpactParticles;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, (
                Self::setup_projectile_impact_particle_system,
                Self::setup_ship_asteroid_impact_particle_system
            ))
            .add_systems(Update, Self::setup_player_ship_trail_particle_system);
    }
}

#[derive(Component)]
pub struct LaserImpactParticleSystem;

#[derive(Component)]
pub struct ShipDamageParticleSystem;

impl ParticlePlugin {
    fn setup_projectile_impact_particle_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands
            .spawn(ParticleSystemBundle {
                particle_system: ParticleSystem {
                    max_particles: 1_000,
                    texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                    spawn_rate_per_second: 2500.0.into(),
                    initial_speed: JitteredValue::jittered(200.0, -50.0..50.0),
                    lifetime: JitteredValue::jittered(0.30, -0.2..0.2),
                    color: ColorOverTime::Gradient(Curve::new(vec![
                        CurvePoint::new(Color::RED, 0.0),
                        CurvePoint::new(Color::rgba(1.0, 0.0, 0.0, 0.0), 1.0),
                    ])),
                    looping: true,
                    system_duration_seconds: 10.0,
                    ..ParticleSystem::default()
                },
                ..ParticleSystemBundle::default()
            })
            .insert(LaserImpactParticleSystem)
            .insert(Transform::default());
    }

    fn setup_player_ship_trail_particle_system(
        mut commands: Commands,
        player_q: Query<(Entity, &Player, Option<&Children>)>,
        particle_q: Query<(Entity, &ParticleSystem)>,
        asset_server: Res<AssetServer>,
    ) {
        for (ent, _player, children) in player_q.iter() {
            let mut has_particle_system = false;

            if let Some(children) = children {
                for child in children.iter() {
                    if particle_q.contains(*child) {
                        has_particle_system = true;
                    }
                }
            }

            if !has_particle_system {
                commands.entity(ent).with_children(|parent| {
                    parent
                        .spawn(ParticleSystemBundle {
                            particle_system: ParticleSystem {
                                max_particles: 1_000,
                                texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                                spawn_rate_per_second: 2500.0.into(),
                                initial_speed: JitteredValue::jittered(15.0, -1.0..1.0),
                                lifetime: JitteredValue::jittered(1.0, -0.5..0.5),
                                color: ColorOverTime::Gradient(Curve::new(vec![
                                    CurvePoint::new(Color::WHITE, 0.0),
                                    CurvePoint::new(Color::rgba(1.0, 1.0, 1.0, 0.0), 1.0),
                                ])),
                                looping: true,
                                system_duration_seconds: 10.0,
                                ..ParticleSystem::default()
                            },
                            ..ParticleSystemBundle::default()
                        })
                        .insert(Playing); // TODO: add and remove this component on ship movement.
                });
            }
        }
    }

    fn setup_ship_asteroid_impact_particle_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands
            .spawn(ParticleSystemBundle {
                particle_system: ParticleSystem {
                    max_particles: 1_000,
                    texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                    spawn_rate_per_second: 2500.0.into(),
                    initial_speed: JitteredValue::jittered(200.0, -50.0..50.0),
                    lifetime: JitteredValue::jittered(0.30, -0.2..0.2),
                    color: ColorOverTime::Gradient(Curve::new(vec![
                        CurvePoint::new(Color::YELLOW, 0.0),
                        CurvePoint::new(Color::rgba(1.0, 1.0, 0.0, 0.0), 1.0),
                    ])),
                    looping: true,
                    system_duration_seconds: 10.0,
                    ..ParticleSystem::default()
                },
                ..ParticleSystemBundle::default()
            })
            .insert(ShipDamageParticleSystem)
            .insert(Transform::default());
    }
}
