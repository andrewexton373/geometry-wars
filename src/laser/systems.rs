use avian2d::prelude::{LayerMask, PhysicsLayer, SpatialQuery, SpatialQueryFilter};
use bevy::color::palettes::css::RED;
use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::*;
use bevy_hanabi::{EffectProperties, EffectSpawner, Value, VectorValue};

use super::components::Laser;
use super::events::LaserEvent;

use crate::particles::components::ProjectileImpactParticles;
use crate::player::components::Player;
use crate::GameLayer;
use crate::{asteroid::events::AblateEvent, health::events::DamageEvent};

pub fn setup_laser(mut commands: Commands, mut laser_query: Query<&mut Laser>) {
    // let line = shapes::Line(Vec2::ZERO, Vec2::X);

    // Create Laser if it Doesn't Exist
    let Ok(_laser) = laser_query.single_mut() else {
        commands
            .spawn(Laser)
            .insert((Transform::from_xyz(0.0, 0.0, 1.0), Name::new("Laser")));
        return;
    };
}

pub fn fire_laser_raycasting(
    mut commands: Commands,
    mut laser_event_reader: MessageReader<LaserEvent>,
    player_q: Query<Entity, With<Player>>,
    spatial_query: SpatialQuery,
    mut damage_events: MessageWriter<DamageEvent>,
    mut gizmos: Gizmos,
    mut effect: Query<
        (&mut EffectProperties, &mut EffectSpawner, &mut Transform),
        With<ProjectileImpactParticles>,
    >,
) {
    let player_ent = player_q.single().expect("No Player");

    // Exclude Player from Raycasting
    let excluded_entities: EntityHashSet = vec![player_ent].into_iter().collect();

    // Note: On first frame where the effect spawns, EffectSpawner is spawned during
    // PostUpdate, so will not be available yet. Ignore for a frame if so.
    let Ok((mut properties, mut effect_spawner, mut effect_transform)) = effect.single_mut() else {
        return;
    };

    effect_spawner.active = false;

    for fire_laser_event in laser_event_reader.read() {
        let laser_active = fire_laser_event.0;
        let ray_pos = fire_laser_event.1;
        let ray_dir = fire_laser_event.2;

        // If laser is active
        if laser_active {
            if let Some(first_hit) = spatial_query.cast_ray(
                ray_pos.as_dvec2(),
                Dir2::new(ray_dir).unwrap(),
                10000.0,
                false,
                &SpatialQueryFilter {
                    excluded_entities: excluded_entities.clone(),
                    mask: LayerMask(GameLayer::Default.to_bits()),
                },
            ) {
                let hit_point = ray_pos.as_dvec2() + ray_dir.as_dvec2() * first_hit.distance;
                let hit_normal = first_hit.normal;
                let hit_ent = first_hit.entity;

                gizmos.line_2d(ray_pos, hit_point.as_vec2(), Color::from(RED));

                commands.write_message(AblateEvent {
                    entity: hit_ent,
                    position: hit_point.as_vec2(),
                    normal: hit_normal.as_vec2(),
                });

                damage_events.write(DamageEvent {
                    entity: hit_ent,
                    damage: 5.0,
                });

                println!("Hit Point: {:?}", hit_point);
                effect_transform.translation = hit_point.extend(0.0).as_vec3();

                // Set the collision normal
                let normal = hit_normal.as_vec2().normalize();
                info!("Collision: n={:?}", normal);
                properties.set(
                    "normal",
                    Value::Vector(VectorValue::new_vec3(normal.extend(0.))),
                );

                effect_transform.translation = hit_point.extend(0.0).as_vec3();
                effect_spawner.active = true;
            } else {
                // Laser Hit Nothing
                gizmos.line_2d(ray_pos, ray_pos + ray_dir * 10000.0, Color::from(RED));
            }
        }
    }
}
