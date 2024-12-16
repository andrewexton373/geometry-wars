use avian2d::prelude::*;
use bevy::color::palettes::css::RED;
use bevy::prelude::*;

use crate::health::events::DamageEvent;

use super::components::Projectile;
use super::events::FireProjectileEvent;

pub const PROJECTILE_RADIUS: f64 = 0.160;

pub fn handle_fire_projectile_events(
    mut commands: Commands,
    global_transforms: Query<&GlobalTransform>,
    mut events: EventReader<FireProjectileEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
    
) {
    for evt in events.read() {
        const BULLET_SPEED: f32 = 6.0;

        let projectile_shape = Circle::new((PROJECTILE_RADIUS * crate::PIXELS_PER_METER) as f32);

        if let Ok(projectile_spawn_position) = global_transforms.get(evt.entity) {
            // Move projectile slightly ahead of where it was fired from to avoid self collision
            let projectile_spawn_position = projectile_spawn_position
                .translation()
                .truncate()
                .as_dvec2()
                + evt
                    .projectile_trajectory
                    .0
                    .trunc()
                    .normalize()
                    .as_vec2()
                    .as_dvec2()
                    * 20.0;

            commands.spawn((
                Projectile {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                },
                Mesh2d(meshes.add(projectile_shape)),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::from(RED)))),
                Transform::from_translation(
                    projectile_spawn_position.extend(2.0).as_vec3(),
                ),
                RigidBody::Dynamic,
                evt.projectile_trajectory,
                Collider::circle(projectile_shape.radius as f64),
            ));
        }
    }
}

pub fn handle_projectile_collision_event(
    mut commands: Commands,
    projectile_query: Query<(Entity, &CollidingEntities), With<Projectile>>,
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
