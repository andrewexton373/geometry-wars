use avian2d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::color::palettes::css::RED;
use bevy::ecs::entity::EntityHash;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy_hanabi::{EffectInitializers, EffectProperties};
// use bevy_particle_systems::Playing;

use super::components::Laser;
use super::events::LaserEvent;

use crate::particles::components::ProjectileImpactParticles;
use crate::player::components::Player;
use crate::{asteroid::events::AblateEvent, health::events::DamageEvent};

pub fn setup_laser(mut commands: Commands, mut laser_query: Query<&mut Laser>) {
    // let line = shapes::Line(Vec2::ZERO, Vec2::X);

    // Create Laser if it Doesn't Exist
    let Ok(_laser) = laser_query.get_single_mut() else {
        commands
            .spawn(Laser)
            .insert((Transform::from_xyz(0.0, 0.0, 1.0), Name::new("Laser")));
        return;
    };
}

pub fn fire_laser_raycasting(
    commands: Commands,
    mut laser_event_reader: EventReader<LaserEvent>,
    mut ablate_event_writer: EventWriter<AblateEvent>,
    // rapier_context: Res<RapierContext>,
    // mut laser_query: Query<&mut Shape, With<Laser>>,
    // mut laser_impact_particles_query: Query<
    //     (Entity, &LaserImpactParticleSystem, &mut Transform),
    //     Without<Laser>,
    // >,
    player_q: Query<Entity, With<Player>>,
    spatial_query: SpatialQuery,
    mut damage_events: EventWriter<DamageEvent>,
    mut gizmos: Gizmos,
    mut effect: Query<
        (
            &mut EffectProperties,
            &mut EffectInitializers,
            &mut Transform,
        ),
        With<ProjectileImpactParticles>,
    >,
    // laser_particles: Res<LaserImpactParticleEffectHandle>
) {
    // TODO: Change Laser State To Off On Player Left Unclick
    // for (ent, _, mut _t) in laser_impact_particles_query.iter_mut() {
    //     // commands.entity(ent).remove::<Playing>();
    // }

    // let mut laser_path = laser_query.single_mut();
    let player_ent = player_q.single();

    // Exclude Player from Raycasting
    let excluded_entities: HashSet<Entity, EntityHash> = vec![player_ent].into_iter().collect();

    for fire_laser_event in laser_event_reader.read() {
        let laser_active = fire_laser_event.0;
        let ray_pos = fire_laser_event.1;
        let ray_dir = fire_laser_event.2;
        let ray_dir_2d = Dir2::new(ray_dir).unwrap();

        // If laser is active
        if laser_active {
            if let Some(first_hit) = spatial_query.cast_ray(
                ray_pos.as_dvec2(),
                Dir2::new(ray_dir).unwrap(),
                10000.0,
                false,
                &SpatialQueryFilter {
                    excluded_entities: excluded_entities.clone(),
                    ..default()
                },
            ) {
                let hit_point = ray_pos.as_dvec2() + ray_dir.as_dvec2() * first_hit.distance;
                let hit_normal = first_hit.normal;
                let hit_ent = first_hit.entity;

                //     commands.entity(ent).with_child(ParticleEffectBundle {
                //         effect: ParticleEffect::new(laser_particles.0.unwrap().clone_weak()),
                //         transform: Transform::from_translation(Vec3::Y),
                //         ..Default::default()
                //     });

                // for (ent, _, mut t) in laser_impact_particles_query.iter_mut() {

                //     // commands.entity(ent).insert(Playing);
                //     t.translation = hit_point.extend(0.0).as_vec3();

                // }

                gizmos.line_2d(ray_pos, hit_point.as_vec2(), Color::from(RED));

                ablate_event_writer.send(AblateEvent(
                    hit_ent,
                    hit_point.as_vec2(),
                    hit_normal.as_vec2(),
                ));
                damage_events.send(DamageEvent {
                    entity: hit_ent,
                    damage: 5.0,
                });

                // Note: On first frame where the effect spawns, EffectSpawner is spawned during
                // PostUpdate, so will not be available yet. Ignore for a frame if so.
                let Ok((mut properties, mut initializers, mut effect_transform)) =
                    effect.get_single_mut()
                else {
                    return;
                };

                effect_transform.translation = hit_point.extend(0.0).as_vec3();

                // Set the collision normal
                let normal = hit_normal.as_vec2().normalize();
                // info!("Collision: n={:?}", normal);
                properties.set("normal", normal.extend(0.).into());

                // Spawn the particles
                initializers.reset();
            } else {
                // Laser Hit Nothing
                gizmos.line_2d(ray_pos, ray_pos + ray_dir * 10000.0, Color::from(RED));
            }
        }
    }
}
