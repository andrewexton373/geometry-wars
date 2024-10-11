use bevy::{color::palettes::css::{RED, YELLOW}, prelude::*};
use bevy_particle_systems::*;

use crate::player::components::Player;

use super::components::{
    LaserImpactParticleSystem, PlayerShipTrailParticles, ShipDamageParticleSystem,
};

pub fn setup_projectile_impact_particle_system(
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
                    CurvePoint::new(Color::from(RED), 0.0),
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

pub fn setup_player_ship_trail_particle_system(
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
                    .insert(PlayerShipTrailParticles)
                    .insert(Playing); // TODO: add and remove this component on ship movement.
            });
        }
    }
}

pub fn setup_ship_asteroid_impact_particle_system(
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
                    CurvePoint::new(Color::from(YELLOW), 0.0),
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
