use crate::{
    health::{components::Health, events::DamageEvent},
    inventory::components::{Inventory, InventoryItem},
    items::Amount,
    player::components::Player,
    ui::damage_indicator::events::DamageIndicatorEvent,
    GameLayer,
};
use bevy::{
    color::palettes::css::{DARK_GRAY, GOLD, GRAY, LIMEGREEN, RED, SILVER},
    ecs::entity,
    math::DVec2,
    prelude::*,
};
// use bevy_particle_systems::Playing;
use avian2d::{
    math::{Scalar, Vector, PI},
    prelude::*,
};
use ordered_float::OrderedFloat;
use rand::Rng;

use crate::{
    collectible::components::Collectible,
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
    player_query: Query<(&Player, &GlobalTransform)>,
    base_station_query: Query<(&SpaceStation, &GlobalTransform)>,
    mut asteroid_spawner: ResMut<AsteroidSpawner>,
    time: Res<Time>,
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

        let rand_x: f32 = rng.gen_range(-PI as f32..PI as f32);
        let rand_y: f32 = rng.gen_range(-PI as f32..PI as f32);
        let rand_direction = Vec2::new(rand_x.cos(), rand_y.sin()).normalize();

        let random_spawn_position =
            player_position + (rand_direction * SPAWN_DISTANCE * crate::PIXELS_PER_METER as f32);
        let direction_to_player = (player_position - random_spawn_position).normalize() * 200.0; // maybe?

        let asteroid = Asteroid::new_with(
            AsteroidSize::Large.radius(),
            AsteroidComposition::new_with_distance(distance_to_base_station),
        );
        let asteroid_transform = Transform::from_translation(random_spawn_position.extend(0.0));
        let asteroid_linear_velocity = LinearVelocity(direction_to_player.as_dvec2());

        commands.send_event(SpawnAsteroidEvent(
            asteroid,
            asteroid_transform,
            asteroid_linear_velocity,
        ));

        //     asteroid,
        //     asteroid_transform,
        //     asteroid_linear_velocity,
        // ));
    }
}

const THRESHOLD_COLLECTIBLE_MASS: f32 = 2500.0;

pub fn tag_small_asteroids_as_collectible(
    trigger: Trigger<OnAdd, Asteroid>,
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Mass), With<Asteroid>>,
) {
    let asteroid_ent = trigger.entity();

    if let Ok((_, mass)) = asteroid_query.get(asteroid_ent) {
        if mass.0 <= THRESHOLD_COLLECTIBLE_MASS {
            info!("{:?} THRESHOLD HIT -> COLLECTIBLE", mass.0);
            if let Some(mut ent_commands) = commands.get_entity(asteroid_ent) {
                ent_commands.insert(Collectible);
                ent_commands.insert(CollisionLayers::new(
                    [GameLayer::Collectible],
                    [GameLayer::Default],
                ));
            }
        }
    }
}

// TODO: Verify this is working... it's definitely not (12/17/2024)
pub fn update_collectible_material_color(
    trigger: Trigger<OnReplace, Asteroid>,
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &Asteroid), With<Collectible>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ent = trigger.entity();
    info!("{:?}", ent);

    info!("{:?}", asteroid_query.iter().count());

    if let Ok((ent, asteroid)) = asteroid_query.get(ent) {
        info!("ASTEROID: {:?}", asteroid);

        let color = match asteroid.primary_composition() {
            AsteroidMaterial::Iron => GRAY,
            AsteroidMaterial::Silver => SILVER,
            AsteroidMaterial::Gold => GOLD,
            _ => LIMEGREEN,
        };

        commands.entity(ent).try_insert((
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            DebugRender::default().with_collider_color(Color::from(LIMEGREEN)),
        ));
    } else {
        info!("ASTROID NOT IN QUERY!");
    }
}

pub fn despawn_far_asteroids(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &mut Asteroid, &mut Transform), With<Asteroid>>,
    player_query: Query<(&Player, &Transform), (With<Player>, Without<Asteroid>)>,
) {
    const DESPAWN_DISTANCE: f32 = 1000.0 * PIXELS_PER_METER as f32;
    let (_player, transform) = player_query.single();
    let player_position = transform.translation.truncate();

    for (entity, _asteroid, transform) in asteroid_query.iter() {
        let asteroid_position = transform.translation.truncate();
        if player_position.distance(asteroid_position) > DESPAWN_DISTANCE {
            commands.entity(entity).try_despawn_recursive();
        }
    }
}

pub fn handle_collectible_collision_event(
    mut commands: Commands,
    collisions: Res<Collisions>,
    asteroid_query: Query<(Entity, &Asteroid, &Mass), With<Collectible>>,
    mut player_query: Query<(Entity, &mut Inventory), With<Player>>,
    mut inventory_full_notification: ResMut<InventoryFullNotificationTimer>,
) {
    let (player_ent, mut inventory) = player_query.single_mut();

    for (asteroid_ent, asteroid, mass) in asteroid_query.iter() {
        for _ in collisions.get(player_ent, asteroid_ent).iter() {
            for comp in asteroid.composition.percent_composition().iter() {
                if !inventory.add_to_inventory(&InventoryItem::Material(
                    *comp.0,
                    Amount::Weight(OrderedFloat(comp.1 * mass.0)),
                )) {
                    inventory_full_notification.0 = Some(Timer::from_seconds(3.0, TimerMode::Once));
                }
            }

            // FIXME: will despawn even if there's no room in inventory to collect.
            commands.entity(asteroid_ent).try_despawn_recursive();
        }
    }
}

pub fn handle_asteroid_collision_event(
    collisions: Res<Collisions>,
    mut asteroid_query: Query<(Entity, &Asteroid, &Mass), Without<Collectible>>,
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let (player_ent, player) = player_query.single_mut();

    for (asteroid_entity, asteroid, mass) in asteroid_query.iter_mut() {
        if let Some(collision) = collisions.get(player_ent, asteroid_entity) {
            let damage = -collision.manifolds[0].contacts[0].penetration;

            damage_events.send(DamageEvent {
                entity: player_ent,
                damage: damage as f32,
            });
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

pub fn ablate_asteroids_events(
    mut events: EventReader<AblateEvent>,
    mut commands: Commands,
    mut asteroids_query: Query<
        (Entity, &mut Asteroid, &mut Health, &GlobalTransform),
        With<Asteroid>,
    >,
    mut damage_indicator_events: EventWriter<DamageIndicatorEvent>,
) {
    for ablate_event in events.read() {
        // let ablate_event = trigger.event();
        let mut rng = rand::thread_rng();
        // let split_angle = rng.gen_range(0.0..PI / 4.0); TODO: Might keep splititng asteroids

        if let Ok((ent, asteroid_to_ablate, mut asteroid_health, _g_trans)) =
            asteroids_query.get_mut(ablate_event.entity)
        {
            let damaged_health = asteroid_health.current() - LASER_DAMAGE;
            asteroid_health.set_current(damaged_health);

            if damaged_health < 0.0 {
                commands.entity(ent).try_despawn_recursive();
            }

            let n: u8 = rng.gen();
            if n > 25 {
                return;
            }

            // Send Damage Indicator Event
            let translation = Transform {
                translation: (ablate_event.position + ablate_event.normal.normalize() * 100.0)
                    .extend(999.0),
                ..default()
            };

            damage_indicator_events.send(DamageIndicatorEvent {
                damage: 1.0,
                traslation: translation,
            });

            // TODO: The new comp distance shouldn't be constant it should update based on player distance from base
            let asteroid = Asteroid::new_with(
                AsteroidSize::OreChunk.radius(),
                AsteroidComposition::new_with_distance(100.0),
            );

            let max_jitter_angle = 60.0;

            let mut rng = rand::thread_rng();
            let jitter_angle =
                rng.gen_range(-max_jitter_angle..max_jitter_angle) * (PI / 180.0) as f32;
            let rotation_matrix = bevy::math::Mat2::from_angle(jitter_angle);
            let rotated_x = rotation_matrix.x_axis.dot(ablate_event.normal);
            let rotated_y = rotation_matrix.y_axis.dot(ablate_event.normal);
            let jitter_normal = DVec2::new(rotated_x as f64, rotated_y as f64);

            let jitter_velocity = rng.gen_range(400.0..800.0);

            commands.send_event(SpawnAsteroidEvent(
                asteroid.clone(),
                Transform::from_translation(ablate_event.position.extend(0.0)),
                LinearVelocity(jitter_normal * jitter_velocity),
            ));
        }
    }
}

pub fn split_asteroids_over_split_ratio(
    mut asteroid_query: Query<(Entity, &mut Asteroid, &Health, &Splittable)>,
    mut split_astroid_events: EventWriter<SplitAsteroidEvent>,
) {
    for (ent, asteroid, asteroid_health, split) in asteroid_query.iter_mut() {
        if asteroid_health.current_percent() < split.0 {
            split_astroid_events.send(SplitAsteroidEvent(ent));
        }
    }
}
fn random_normalized_vec2() -> DVec2 {
    // Generate random components for x and y
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen_range(-1.0..1.0); // Random x between -1 and 1
    let y: f64 = rng.gen_range(-1.0..1.0); // Random y between -1 and 1

    // Create a random vector
    let random_vec = DVec2::new(x, y);

    // Normalize the vector
    random_vec.normalize()
}
pub fn split_asteroid_events(
    mut events: EventReader<SplitAsteroidEvent>,
    mut commands: Commands,
    mut asteroid_q: Query<(&Asteroid, &Transform, &LinearVelocity)>,
) {
    for evt in events.read() {
        let asteroid_ent = evt.0;
        if let Ok((asteroid, transform, linear_velocity)) = asteroid_q.get_mut(asteroid_ent) {
            let speed_scale = 1000.0;

            let right_velocity = random_normalized_vec2() * speed_scale;
            let left_velocity = -random_normalized_vec2();

            let half_radius = asteroid.radius / 2.0;

            let left_asteroid = Asteroid::new_with(half_radius, asteroid.composition.jitter());
            let right_asteroid = Asteroid::new_with(half_radius, asteroid.composition.jitter());

            commands.send_event(SpawnAsteroidEvent(
                left_asteroid,
                *transform,
                LinearVelocity(left_velocity),
            ));

            commands.send_event(SpawnAsteroidEvent(
                right_asteroid,
                *transform,
                LinearVelocity(right_velocity),
            ));

            commands.entity(asteroid_ent).try_despawn_recursive();
        }
    }
}

pub fn handle_spawn_asteroid_events(
    mut spawn_events: EventReader<SpawnAsteroidEvent>,
    mut commands: Commands,
    spatial: SpatialQuery,
    query: Query<(&Collider, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for evt in spawn_events.read() {
        let asteroid = evt.0.clone();
        let target_transform = evt.1;
        let linear_velocity = evt.2;
        let collider = Collider::convex_hull(
            asteroid
                .polygon()
                .vertices
                .iter()
                .map(|point| Vector {
                    x: point.x as f64,
                    y: point.y as f64,
                })
                .collect(),
        )
        .unwrap();
        let health_pool = collider.mass_properties(1.0).mass; // Set Healthpool to mass?

        let mut rng = rand::thread_rng();
        let splittable = Splittable(rng.gen_range(0.4..0.8));

        if let Some(transform) =
            find_free_space(&spatial, &query, target_transform, &collider, 0.1, 10)
        {
            commands.spawn((
                asteroid.clone(),
                RigidBody::Dynamic,
                collider,
                Mass(health_pool),
                linear_velocity,
                splittable,
                Name::new("Asteroid"),
                Mesh2d(meshes.add(asteroid.generate_mesh())),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(DARK_GRAY))),
                transform,
                Health::with_maximum(health_pool),
            ));
        }
    }
}

fn find_free_space(
    spatial: &SpatialQuery,
    query: &Query<(&Collider, &Transform)>,
    target_transform: Transform,
    collider: &Collider,
    margin: Scalar,
    max_iterations: usize,
) -> Option<Transform> {
    let mut target_position: Position =
        Position::new(target_transform.translation.truncate().as_dvec2());
    let rotation = Rotation::from(target_transform.rotation);

    // Scale collider by margin
    let mut collider = collider.clone();
    collider.set_scale(Vector::ONE + margin, 8);

    let filter = SpatialQueryFilter::default();

    // Iteratively update the position by computing contacts against intersecting colliders
    // and moving the target position based on the data.
    // The algorithm stops once there are no intersections or `max_iterations` is reached.
    for _ in 0..max_iterations {
        // Get entities intersecting the space
        let intersections = spatial.shape_intersections(
            &collider,
            target_position.as_vec2().as_dvec2(),
            rotation.as_radians(),
            &filter.clone(),
        );

        if intersections.is_empty() {
            // No intersections, free space found
            return Some(target_transform.with_translation(target_position.extend(0.0).as_vec3()));
        } else {
            // Iterate over intersections and move the target position
            // based on computed contact data.
            for entity in intersections {
                // Get collider of intersecting entity
                let Ok((hit_collider, hit_transform)) = query.get(entity) else {
                    continue;
                };
                let hit_translation: Position =
                    Position::new(hit_transform.translation.truncate().as_dvec2());

                // Compute contact between the entity to spawn and the intersecting entity
                if let Ok(Some(contact)) = contact_query::contact(
                    &collider,
                    target_position,
                    rotation,
                    hit_collider,
                    hit_translation,
                    hit_transform.rotation,
                    0.0,
                ) {
                    let normal = contact.global_normal2(&hit_transform.rotation.into());

                    // Epsilon to avoid floating point precision issues
                    let delta = normal * (contact.penetration + 0.00001);

                    // Move target position to solve overlap
                    target_position = Position::new(target_position.as_vec2().as_dvec2() + delta);
                }
            }
        }
    }

    None
}
