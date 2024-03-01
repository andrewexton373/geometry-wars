use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_xpbd_2d::components::{Collider, CollidingEntities, LinearVelocity, RigidBody};

use crate::health::events::DamageEvent;

use super::components::Projectile;
use super::events::FireProjectileEvent;

pub const PROJECTILE_RADIUS: f32 = 0.160;

pub fn handle_fire_projectile_events(
    mut commands: Commands,
    global_transforms: Query<&GlobalTransform>,
    mut events: EventReader<FireProjectileEvent>,
) {
    for evt in events.read() {
        const BULLET_SPEED: f32 = 6.0;

        let projectile_shape = shapes::Circle {
            radius: PROJECTILE_RADIUS * crate::PIXELS_PER_METER,
            ..default()
        };

        if let Ok(projectile_spawn_position) = global_transforms.get(evt.entity) {
            // Move projectile slightly ahead of where it was fired from to avoid self collision
            let projectile_spawn_position = projectile_spawn_position.translation().truncate()
                + evt.projectile_trajectory.0.trunc().normalize() * 20.0;

            commands.spawn((
                Projectile {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                },
                ShapeBundle {
                    path: GeometryBuilder::build_as(&projectile_shape),
                    spatial: SpatialBundle {
                        transform: Transform::from_translation(
                            projectile_spawn_position.extend(2.0),
                        ),
                        ..default()
                    },

                    ..default()
                },
                Fill::color(Color::RED),
                RigidBody::Dynamic,
                evt.projectile_trajectory,
                Collider::ball(projectile_shape.radius),
            ));
        }
    }
}

pub fn handle_projectile_collision_event(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &CollidingEntities), With<Projectile>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let mut projectiles_to_remove: Vec<Entity> = Vec::new();

    for (projectile_ent, _projectile) in projectile_query.iter() {
        if let Ok((_, colliding_entities)) = projectile_query.get(projectile_ent) {
            for colliding_entity in colliding_entities.iter() {
                damage_events.send(DamageEvent {
                    entity: *colliding_entity,
                    damage: 5.0,
                });
            }
            if !colliding_entities.is_empty() {
                projectiles_to_remove.push(projectile_ent);
            }
        }
    }

    // Remove Projectiles that experienced a collision.
    for projectile_to_remove in projectiles_to_remove.iter() {
        commands.entity(*projectile_to_remove).despawn_recursive();
    }
}
