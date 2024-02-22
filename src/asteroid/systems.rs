use bevy::prelude::*;
use bevy_particle_systems::Playing;
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, geometry::GeometryBuilder, prelude::FillOptions};
use bevy_tweening::{lens::TextColorLens, Animator, EaseFunction, RepeatCount, Tween, TweenCompleted};
use bevy_xpbd_2d::{components::{Collider, LinearVelocity, Mass, RigidBody}, math::PI, plugins::{collision::Collisions, spatial_query::SpatialQuery}};
use ordered_float::OrderedFloat;
use rand::Rng;

use crate::{base_station::BaseStation, events::AblateEvent, game_ui::{ContextClue, ContextClues}, inventory::{Amount, Inventory, InventoryItem}, particles::ShipDamageParticleSystem, player::Player, PIXELS_PER_METER};

use super::{
    components::{
        Asteroid, AsteroidComposition, AsteroidMaterial, AsteroidSize, Collectible, Splittable
    },
    plugin::LASER_DAMAGE, resources::InventoryFullNotificationTimer,
    resources::AsteroidSpawner
};

// System to spawn asteroids at some distance away from the ship in random directions,
// each asteroid with an initial velocity aimed towards the players ship
pub fn spawn_asteroids_aimed_at_ship(
    mut commands: Commands,
    player_query: Query<(&Player, &GlobalTransform)>,
    base_station_query: Query<(&BaseStation, &GlobalTransform)>,
    mut asteroid_spawner: ResMut<AsteroidSpawner>,
    time: Res<Time>,
    _spatial: SpatialQuery,
    _query: Query<(&Collider, &Transform)>
) {
    const SPAWN_DISTANCE: f32 = 350.0;

    asteroid_spawner.timer.tick(time.delta());

    if asteroid_spawner.timer.finished() {
        asteroid_spawner.timer.reset();

        let mut rng = rand::thread_rng();
        let (_player, player_g_transform) = player_query.single();
        let (_base_station, base_station_g_transform) = base_station_query.single();

        let distance_to_base_station = (player_g_transform.translation()
            - base_station_g_transform.translation())
        .length();
        let player_position = player_g_transform.translation().truncate();

        let rand_x: f32 = rng.gen_range(-PI..PI);
        let rand_y: f32 = rng.gen_range(-PI..PI);
        let rand_direction = Vec2::new(rand_x.cos(), rand_y.sin()).normalize();

        let random_spawn_position =
            player_position + (rand_direction * SPAWN_DISTANCE * crate::PIXELS_PER_METER);
        let direction_to_player = (player_position - random_spawn_position).normalize() * 20.0; // maybe?

        Asteroid::spawn_asteroid(
            &mut commands,
            AsteroidSize::Large,
            AsteroidComposition::new_with_distance(distance_to_base_station),
            direction_to_player * crate::PIXELS_PER_METER,
            random_spawn_position,
        );
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
    mut player_damage_particle_query: Query<(
        Entity,
        &ShipDamageParticleSystem,
        &mut Transform,
    )>,
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
                damage_particles_t.translation =
                    (collision.manifolds[0].contacts[0].point1 * crate::PIXELS_PER_METER).extend(999.0);
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
) {
    let font = asset_server.load("fonts/FiraMono-Regular.ttf");

    for ablate_event in ablate_event_reader.read() {
        let mut rng = rand::thread_rng();
        // let split_angle = rng.gen_range(0.0..PI / 4.0); TODO: Might keep splititng asteroids

        if let Ok((ent, mut asteroid_to_ablate, _g_trans)) =
            asteroids_query.get_mut(ablate_event.0)
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

            let asteroid_ent = commands
                .spawn((
                    asteroid.clone(),
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&asteroid.polygon()),
                        spatial: Transform::from_xyz(ablate_event.1.x, ablate_event.1.y, 0.0).into(),
                        ..default()
                    },
                    Fill::color(Color::DARK_GRAY),
                    RigidBody::Dynamic,
                    LinearVelocity(ablate_event.2),
                    Collider::convex_hull(asteroid.polygon().points).unwrap(),
                    Name::new("Asteroid"),
                ))
                .id();

            // If the asteroid is an ore chunk, add Collectible Tag
            if asteroid.clone().size == AsteroidSize::OreChunk {
                commands.entity(asteroid_ent).insert(Collectible);
            }
        }
    }
}

pub fn split_asteroids_over_split_ratio(
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &mut Asteroid, &GlobalTransform, &Splittable)>,
        ) {
    for (ent, asteroid, g_t, split) in asteroid_query.iter_mut() {
        if asteroid.health.current_percent() < split.0 {
            split_asteroid(&mut commands, ent, &asteroid, g_t.translation().truncate());
        }
    }
}

pub fn split_asteroid(
    commands: &mut Commands,
    asteroid_ent: Entity,
    asteroid: &Asteroid,
    asteroid_translation: Vec2,
            // projectile_velocity: &LinearVelocity,
) {
    // let mut rng = rand::thread_rng();
    // let split_angle = rng.gen_range(0.0..PI / 4.0);

    // let right_velocity = projectile_velocity
    //     .linvel
    //     .rotate(Vec2::from_angle(split_angle))
    //     * 0.5;
    // let left_velocity = projectile_velocity
    //     .linvel
    //     .rotate(Vec2::from_angle(-split_angle))
    //     * 0.5;
    let right_velocity = Vec2::ZERO;
    let left_velocity = Vec2::ZERO;

    match &asteroid.size {
        AsteroidSize::Small => {
            let left_comp = asteroid.composition.jitter();
            let right_comp = asteroid.composition.jitter();

            Asteroid::spawn_asteroid(
                commands,
                AsteroidSize::OreChunk,
                right_comp,
                right_velocity,
                asteroid_translation,
            );
            Asteroid::spawn_asteroid(
                commands,
                AsteroidSize::OreChunk,
                left_comp,
                left_velocity,
                asteroid_translation,
            );
            commands.entity(asteroid_ent).despawn_recursive();
        }
        AsteroidSize::Medium => {
            Asteroid::spawn_asteroid(
                commands,
                AsteroidSize::Small,
                asteroid.composition.jitter(),
                right_velocity,
                asteroid_translation,
            );
            Asteroid::spawn_asteroid(
                commands,
                AsteroidSize::Small,
                asteroid.composition.jitter(),
                left_velocity,
                asteroid_translation,
            );
            commands.entity(asteroid_ent).despawn_recursive();
        }
        AsteroidSize::Large => {
            Asteroid::spawn_asteroid(
                commands,
                AsteroidSize::Medium,
                asteroid.composition.jitter(),
                right_velocity,
                asteroid_translation,
            );
            Asteroid::spawn_asteroid(
                commands,
                AsteroidSize::Medium,
                asteroid.composition.jitter(),
                left_velocity,
                asteroid_translation,
            );
            commands.entity(asteroid_ent).despawn_recursive();
        }
        _ => {}
    }
}
