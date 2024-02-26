use crate::{inventory::components::{Inventory, InventoryItem}, items::Amount, player::components::Player};
use bevy::prelude::*;
use bevy_particle_systems::Playing;
use bevy_prototype_lyon::{
    draw::Fill, entity::ShapeBundle, geometry::GeometryBuilder, prelude::FillOptions,
};
use bevy_tweening::{
    lens::TextColorLens, Animator, EaseFunction, RepeatCount, Tween, TweenCompleted,
};
use bevy_xpbd_2d::{
    components::{Collider, LinearVelocity, Mass, RigidBody},
    math::PI,
    plugins::{collision::Collisions, spatial_query::SpatialQuery},
};
use ordered_float::OrderedFloat;
use rand::Rng;

use crate::{
    collectible::components::Collectible,
    particles::components::ShipDamageParticleSystem,
    space_station::components::SpaceStation,
    ui::context_clue::resources::{ContextClue, ContextClues},
    PIXELS_PER_METER,
};

use super::{
    components::{Asteroid, AsteroidComposition, AsteroidMaterial, AsteroidSize, Splittable},
    events::{AblateEvent, SpawnAsteroidEvent, SplitAsteroidEvent},
    plugin::LASER_DAMAGE,
    resources::{AsteroidSpawner, InventoryFullNotificationTimer},
};

// System to spawn asteroids at some distance away from the ship in random directions,
// each asteroid with an initial velocity aimed towards the players ship
pub fn spawn_asteroids_aimed_at_ship(
    mut commands: Commands,
    mut spawn_asteroid_events: EventWriter<SpawnAsteroidEvent>,
    player_query: Query<(&Player, &GlobalTransform)>,
    base_station_query: Query<(&SpaceStation, &GlobalTransform)>,
    mut asteroid_spawner: ResMut<AsteroidSpawner>,
    time: Res<Time>,
    _spatial: SpatialQuery,
    _query: Query<(&Collider, &Transform)>,
) {
    const SPAWN_DISTANCE: f32 = 350.0;

    asteroid_spawner.timer.tick(time.delta());

    if asteroid_spawner.timer.finished() {
        asteroid_spawner.timer.reset();

        let mut rng = rand::thread_rng();
        let (_player, player_g_transform) = player_query.single();
        let (_base_station, base_station_g_transform) = base_station_query.single();

        let distance_to_base_station =
            (player_g_transform.translation() - base_station_g_transform.translation()).length();
        let player_position = player_g_transform.translation().truncate();

        let rand_x: f32 = rng.gen_range(-PI..PI);
        let rand_y: f32 = rng.gen_range(-PI..PI);
        let rand_direction = Vec2::new(rand_x.cos(), rand_y.sin()).normalize();

        let random_spawn_position =
            player_position + (rand_direction * SPAWN_DISTANCE * crate::PIXELS_PER_METER);
        let direction_to_player = (player_position - random_spawn_position).normalize() * 200.0; // maybe?

        let asteroid = Asteroid::new_with(AsteroidSize::Large, AsteroidComposition::new_with_distance(distance_to_base_station));
        let asteroid_transform = Transform::from_translation(random_spawn_position.extend(0.0));
        let asteroid_linear_velocity = LinearVelocity(direction_to_player);

        spawn_asteroid_events.send(SpawnAsteroidEvent(asteroid, asteroid_transform, asteroid_linear_velocity));
    }
}

pub fn despawn_far_asteroids(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &mut Asteroid, &mut Transform), With<Asteroid>>,
    player_query: Query<(&Player, &Transform), (With<Player>, Without<Asteroid>)>,
) {
    const DESPAWN_DISTANCE: f32 = 1000.0 * PIXELS_PER_METER;
    let (_player, transform) = player_query.single();
    let player_position = transform.translation.truncate();

    for (entity, _asteroid, transform) in asteroid_query.iter() {
        let asteroid_position = transform.translation.truncate();
        if player_position.distance(asteroid_position) > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// TODO: Verify this is working...
pub fn update_collectible_material_color(
    mut asteroid_query: Query<(&Asteroid, &mut Fill), With<Asteroid>>,
) {
    for (asteroid, mut fill) in asteroid_query.iter_mut() {
        if asteroid.size == AsteroidSize::OreChunk {
            match asteroid.primary_composition() {
                AsteroidMaterial::Iron => {
                    *fill = Fill {
                        color: Color::GRAY,
                        options: FillOptions::default(),
                    };
                }
                AsteroidMaterial::Silver => {
                    *fill = Fill {
                        color: Color::SILVER,
                        options: FillOptions::default(),
                    };
                }
                AsteroidMaterial::Gold => {
                    *fill = Fill {
                        color: Color::GOLD,
                        options: FillOptions::default(),
                    };
                }
                _ => {}
            }
        }
    }
}

pub fn handle_asteroid_collision_event(
    mut commands: Commands,
    collisions: Res<Collisions>,
    mut asteroid_query: Query<(Entity, &Asteroid, &Mass), With<Asteroid>>,
    mut player_query: Query<(Entity, &mut Player, &mut Inventory), With<Player>>,
    mut inventory_full_notification: ResMut<InventoryFullNotificationTimer>,
    mut player_damage_particle_query: Query<(Entity, &ShipDamageParticleSystem, &mut Transform)>,
) {
    let (player_ent, mut player, mut inventory) = player_query.single_mut();

    let (damage_particles_ent, _, mut damage_particles_t) =
        player_damage_particle_query.single_mut();
    commands.entity(damage_particles_ent).remove::<Playing>();

    for (asteroid_entity, asteroid, mass) in asteroid_query.iter_mut() {
        if let Some(collision) = collisions.get(player_ent, asteroid_entity) {
            let mut asteroid_collision = false;

            match asteroid.size {
                AsteroidSize::OreChunk => {
                    let ore_chunk_mass = mass.0;

                    for comp in asteroid.composition.percent_composition().iter() {
                        if !inventory.add_to_inventory(&InventoryItem::Material(
                            *comp.0,
                            Amount::Weight(OrderedFloat(comp.1 * ore_chunk_mass)),
                        )) {
                            inventory_full_notification.0 =
                                Some(Timer::from_seconds(3.0, TimerMode::Once));
                        }
                    }

                    // FIXME: will despawn even if there's no room in inventory to collect.
                    commands.entity(asteroid_entity).despawn_recursive();
                }
                AsteroidSize::Small => {
                    player.take_damage(1.0);
                    asteroid_collision = true;
                }
                AsteroidSize::Medium => {
                    player.take_damage(2.5);
                    asteroid_collision = true;
                }
                AsteroidSize::Large => {
                    player.take_damage(5.0);
                    asteroid_collision = true;
                }
            }

            if asteroid_collision {
                damage_particles_t.translation = (collision.manifolds[0].contacts[0].point1
                    * crate::PIXELS_PER_METER)
                    .extend(999.0);
                commands.entity(damage_particles_ent).insert(Playing);
            }
        }
    }
}

pub fn display_inventory_full_context_clue(
    mut context_clues_res: ResMut<ContextClues>,
    mut inventory_full_notification: ResMut<InventoryFullNotificationTimer>,
    time: Res<Time>,
) {
    if let Some(timer) = inventory_full_notification.0.as_mut() {
        timer.tick(time.delta());

        context_clues_res.0.insert(ContextClue::CargoBayFull);

        if timer.finished() {
            inventory_full_notification.0 = None;
        }
    } else {
        context_clues_res.0.remove(&ContextClue::CargoBayFull);
    }
}

pub fn remove_post_animation_text(
    mut commands: Commands,
    mut tween_completed: EventReader<TweenCompleted>,
) {
    for evt in tween_completed.read() {
        if evt.user_data == 111 {
            commands.entity(evt.entity).despawn_recursive();
        }
    }
}

pub fn ablate_asteroids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asteroids_query: Query<(Entity, &mut Asteroid, &GlobalTransform), With<Asteroid>>,
    mut ablate_event_reader: EventReader<AblateEvent>,
    mut spawn_asteroid_events: EventWriter<SpawnAsteroidEvent>
) {
    let font = asset_server.load("fonts/FiraMono-Regular.ttf");

    for ablate_event in ablate_event_reader.read() {
        let mut rng = rand::thread_rng();
        // let split_angle = rng.gen_range(0.0..PI / 4.0); TODO: Might keep splititng asteroids

        if let Ok((ent, mut asteroid_to_ablate, _g_trans)) = asteroids_query.get_mut(ablate_event.0)
        {
            let damaged_health = asteroid_to_ablate.health.current() - LASER_DAMAGE;
            asteroid_to_ablate.health.set_current(damaged_health);

            if damaged_health < 0.0 {
                commands.entity(ent).despawn_recursive();
            }

            let n: u8 = rng.gen();
            if n > 10 {
                return;
            }

            let tween = Tween::new(
                EaseFunction::ExponentialInOut,
                std::time::Duration::from_millis(3000),
                TextColorLens {
                    start: Color::Rgba {
                        red: 255.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 1.0,
                    },
                    end: Color::Rgba {
                        red: 255.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 0.0,
                    },
                    section: 0,
                },
            )
            .with_repeat_count(RepeatCount::Finite(1))
            .with_completed_event(111);

            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "-1HP",
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.0,
                            color: Color::RED,
                        },
                    ),
                    transform: Transform {
                        translation: (ablate_event.1 + ablate_event.2.normalize() * 100.0)
                            .extend(999.0),
                        ..default()
                    },
                    ..default()
                },
                Animator::new(tween),
            ));

            // TODO: The new comp distance shouldn't be constant it should update based on player distance from base
            let asteroid = Asteroid::new_with(
                AsteroidSize::OreChunk,
                AsteroidComposition::new_with_distance(100.0),
            );

            spawn_asteroid_events.send(SpawnAsteroidEvent(
                asteroid.clone(),
                Transform::from_translation(ablate_event.1.extend(0.0)),
                LinearVelocity(ablate_event.2))
            );

        }
    }
}

pub fn split_asteroids_over_split_ratio(
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &mut Asteroid, &GlobalTransform, &Splittable)>,
    mut split_astroid_events: EventWriter<SplitAsteroidEvent>
) {
    for (ent, asteroid, g_t, split) in asteroid_query.iter_mut() {
        if asteroid.health.current_percent() < split.0 {
            split_astroid_events.send(SplitAsteroidEvent(ent));
            // split_asteroid(&mut commands, ent, &asteroid, g_t.translation().truncate());
        }
    }
}



pub fn split_asteroid_events(
    mut commands: Commands,
    mut asteroid_q: Query<(&Asteroid, &Transform, &LinearVelocity)>,
    mut split_astroid_events: EventReader<SplitAsteroidEvent>,
    mut spawn_asteroid_events: EventWriter<SpawnAsteroidEvent>,
    // projectile_velocity: &LinearVelocity,
) {
    for evt in split_astroid_events.read() {
        let asteroid_ent = evt.0;
        if let Some((asteroid, transform, linear_velocity)) = asteroid_q.get_mut(asteroid_ent).ok() {
            let right_velocity = Vec2::ZERO;
        let left_velocity = Vec2::ZERO;

        match &asteroid.size {
            AsteroidSize::Small => {

                let left_asteroid = Asteroid::new_with(AsteroidSize::OreChunk, asteroid.composition.jitter());
                let right_asteroid = Asteroid::new_with(AsteroidSize::OreChunk, asteroid.composition.jitter());

                spawn_asteroid_events.send(SpawnAsteroidEvent(
                    left_asteroid,
                    *transform,
                    LinearVelocity::ZERO
                ));

                spawn_asteroid_events.send(SpawnAsteroidEvent(
                    right_asteroid,
                    *transform,
                    LinearVelocity::ZERO
                ));

            }
            AsteroidSize::Medium => {
                let left_asteroid = Asteroid::new_with(AsteroidSize::Small, asteroid.composition.jitter());
                let right_asteroid = Asteroid::new_with(AsteroidSize::Small, asteroid.composition.jitter());

                spawn_asteroid_events.send(SpawnAsteroidEvent(
                    left_asteroid,
                    *transform,
                    LinearVelocity::ZERO
                ));

                spawn_asteroid_events.send(SpawnAsteroidEvent(
                    right_asteroid,
                    *transform,
                    LinearVelocity::ZERO
                ));
            }
            AsteroidSize::Large => {
                let left_asteroid = Asteroid::new_with(AsteroidSize::Medium, asteroid.composition.jitter());
                let right_asteroid = Asteroid::new_with(AsteroidSize::Medium, asteroid.composition.jitter());

                spawn_asteroid_events.send(SpawnAsteroidEvent(
                    left_asteroid,
                    *transform,
                    LinearVelocity::ZERO
                ));

                spawn_asteroid_events.send(SpawnAsteroidEvent(
                    right_asteroid,
                    *transform,
                    LinearVelocity::ZERO
                ));
            }
            _ => {}
        }
        commands.entity(asteroid_ent).despawn_recursive();
        }
    }

}


pub fn spawn_asteroid_events(
    mut commands: Commands,
    mut spawn_asteroid_events: EventReader<SpawnAsteroidEvent>
) {
    for evt in spawn_asteroid_events.read() {

        let asteroid = evt.0.clone();
        let transform = evt.1;
        let linear_velocity = evt.2;

        let mut rng = rand::thread_rng();
        let splittable = Splittable(rng.gen_range(0.4..0.8));

        let asteroid_ent = commands
            .spawn(asteroid.clone())
            .insert((
                RigidBody::Dynamic,
                Collider::convex_hull(asteroid.polygon().points).unwrap(),
                linear_velocity,
                splittable,
                Name::new("Asteroid"),
                ShapeBundle {
                    path: GeometryBuilder::build_as(&asteroid.polygon()),
                    spatial: SpatialBundle::from_transform(transform),
                    ..default()
                },
                Fill::color(Color::DARK_GRAY),
            ))
            .id();

        // If the asteroid is an ore chunk, add Collectible Tag
        if asteroid.clone().size == AsteroidSize::OreChunk {
            commands.entity(asteroid_ent).insert(Collectible);
        }

    }
}
