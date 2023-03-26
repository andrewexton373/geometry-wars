use bevy::{
    prelude::*,
};

use bevy_particle_systems::*;
use crate::player::Player;

pub struct ParticlePlugin;

#[derive(Component)]
pub struct PlayerShipTrailParticles;

#[derive(Component)]
pub struct ProjectileImpactParticles;

#[derive(Component)]
pub struct ShipAstroidImpactParticles;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_startup_system(Self::setup_projectile_impact_particle_system)
            .add_system(Self::setup_player_ship_trail_particle_system)
            .add_startup_system(Self::setup_ship_astroid_impact_particle_system);
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
                    max_particles: 10_000,
                    texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                    spawn_rate_per_second: 2500.0.into(),
                    initial_speed: JitteredValue::jittered(200.0, -50.0..50.0),
                    lifetime: JitteredValue::jittered(0.30, -0.2..0.2),
                    color: ColorOverTime::Gradient(Gradient::new(vec![
                        ColorPoint::new(Color::RED, 0.0),
                        ColorPoint::new(Color::rgba(1.0, 0.0, 0.0, 0.0), 1.0),
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
        for (ent, player, children) in player_q.iter() {

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
                                max_particles: 10_000,
                                texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                                spawn_rate_per_second: 2500.0.into(),
                                initial_speed: JitteredValue::jittered(15.0, -1.0..1.0),
                                lifetime: JitteredValue::jittered(1.0, -0.5..0.5),
                                color: ColorOverTime::Gradient(Gradient::new(vec![
                                    ColorPoint::new(Color::WHITE, 0.0),
                                    ColorPoint::new(Color::rgba(1.0, 1.0, 1.0, 0.0), 1.0),
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

    fn setup_ship_astroid_impact_particle_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands
            .spawn(ParticleSystemBundle {
                particle_system: ParticleSystem {
                    max_particles: 10_000,
                    texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                    spawn_rate_per_second: 2500.0.into(),
                    initial_speed: JitteredValue::jittered(200.0, -50.0..50.0),
                    lifetime: JitteredValue::jittered(0.30, -0.2..0.2),
                    color: ColorOverTime::Gradient(Gradient::new(vec![
                        ColorPoint::new(Color::YELLOW, 0.0),
                        ColorPoint::new(Color::rgba(1.0, 1.0, 0.0, 0.0), 1.0),
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
