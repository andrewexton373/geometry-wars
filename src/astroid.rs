use crate::base_station::BaseStation;
use crate::game_ui::{ContextClue, ContextClues};
use crate::health::Health;
use crate::inventory::{Amount, Inventory, InventoryItem};
use crate::particles::ShipAstroidImpactParticles;
use crate::{Player, PIXELS_PER_METER};
use bevy::prelude::*;
use bevy::reflect::FromReflect;
use bevy::utils::HashMap;
use bevy_hanabi::ParticleEffect;
use bevy_prototype_lyon::prelude::{self as lyon, GeometryBuilder, Fill, ShapeBundle, FillOptions};
use bevy_rapier2d::prelude::*;
use ordered_float::OrderedFloat;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::cmp::Ordering;
use std::f32::consts::PI;
use std::fmt;

#[derive(Resource)]
pub struct InventoryFullNotificationTimer(pub Option<Timer>);

pub struct AstroidPlugin;

#[derive(Component, Clone, Debug)]
pub struct Astroid {
    pub velocity: Vec2,
    pub size: AstroidSize,
    pub health: Health,
    pub composition: Composition,
}

impl Astroid {
    pub fn primary_composition(&self) -> AstroidMaterial {
        self.composition.most_abundant()
    }
}
#[derive(Component, Clone, Debug)]
pub struct Composition {
    composition: HashMap<AstroidMaterial, f32>,
}

impl Composition {
    pub fn new_with_distance(distance: f32) -> Self {
        const MIN_DISTANCE: f32 = 0.0;
        const MAX_DISTANCE: f32 = 100000.0;

        let percentage =
            ((distance - MIN_DISTANCE) / (MAX_DISTANCE - MIN_DISTANCE)).clamp(0.0, 1.0);

        let mut near_composition: HashMap<AstroidMaterial, f32> = HashMap::new();
        near_composition.insert(AstroidMaterial::Iron, 0.95);
        near_composition.insert(AstroidMaterial::Silver, 0.04);
        near_composition.insert(AstroidMaterial::Gold, 0.01);

        let mut far_composition: HashMap<AstroidMaterial, f32> = HashMap::new();
        far_composition.insert(AstroidMaterial::Iron, 1.0);
        far_composition.insert(AstroidMaterial::Silver, 2.0);
        far_composition.insert(AstroidMaterial::Gold, 2.0);

        let mut composition = HashMap::new();

        for near in near_composition.iter() {
            let far = far_composition.get(near.0).unwrap();

            composition.insert(*near.0, near.1 + (far - near.1) * percentage);
        }

        Self { composition }
    }

    pub fn most_abundant(&self) -> AstroidMaterial {
        self.composition
            .iter()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(k, _v)| k.clone())
            .unwrap()
    }

    pub fn percent_composition(&self) -> HashMap<AstroidMaterial, f32> {
        let cloned: HashMap<AstroidMaterial, f32> = self.composition.clone();
        let total_weights: f32 = cloned.iter().map(|e| e.1).sum();
        cloned
            .into_iter()
            .map(|e| (e.0, e.1 / total_weights))
            .collect::<HashMap<AstroidMaterial, f32>>()
    }

    pub fn jitter(&self) -> Composition {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.05).unwrap();

        Composition {
            composition: self
                .percent_composition()
                .into_iter()
                .map(|(k, v)| (k, (v + normal.sample(&mut rng)).clamp(0.0, f32::MAX)))
                .collect(),
        }
    }
}

#[test]
fn test_most_abundant() {
    assert_eq!(
        Composition::new_with_distance(0.0).most_abundant(),
        AstroidMaterial::Iron
    );
    assert_eq!(
        Composition::new_with_distance(10000.0).most_abundant(),
        AstroidMaterial::Gold
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AstroidSize {
    OreChunk,
    Small,
    Medium,
    Large,
}

#[derive(Component)]
pub struct Collectible;

impl AstroidSize {
    fn radius(self) -> f32 {
        match self {
            Self::OreChunk => 25.0,
            Self::Small => 45.0,
            Self::Medium => 85.0,
            Self::Large => 100.0,
        }
    }

    fn num_sides(self) -> usize {
        match self {
            Self::OreChunk => 5,
            Self::Small => 7,
            Self::Medium => 9,
            Self::Large => 11,
        }
    }
}

#[derive(
    Component,
    Reflect,
    FromReflect,
    Default,
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
)]
pub enum AstroidMaterial {
    #[default]
    Rock,
    Iron,
    Silver,
    Gold,
}

impl fmt::Display for AstroidMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AstroidMaterial::Rock => write!(f, "Rock"),
            AstroidMaterial::Iron => write!(f, "Iron"),
            AstroidMaterial::Silver => write!(f, "Silver"),
            AstroidMaterial::Gold => write!(f, "Gold"),
        }
    }
}

#[derive(Component, Resource)]
pub struct AstroidSpawner {
    timer: Timer,
}

pub struct AblateEvent(pub Entity, pub Vec2, pub Vec2);

impl Plugin for AstroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AstroidSpawner {
            timer: Timer::from_seconds(0.25, TimerMode::Once),
        })
        .insert_resource(InventoryFullNotificationTimer(None))
        .add_event::<AblateEvent>()
        .add_system(Self::spawn_astroids_aimed_at_ship)
        .add_system(Self::despawn_far_astroids)
        .add_system(Self::handle_astroid_collision_event)
        .add_system(Self::ablate_astroids)
        .add_system(Self::display_inventory_full_context_clue)
        .add_system(Self::update_collectible_material_color);
        // .register_inspectable::<Astroid>();
    }
}

impl AstroidPlugin {
    fn spawn_astroids_aimed_at_ship(
        mut commands: Commands,
        player_query: Query<(&Player, &GlobalTransform)>,
        base_station_query: Query<(&BaseStation, &GlobalTransform)>,
        mut astroid_spawner: ResMut<AstroidSpawner>,
        time: Res<Time>,
    ) {
        astroid_spawner.timer.tick(time.delta());

        if astroid_spawner.timer.finished() {
            astroid_spawner.timer.reset();

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

            const SPAWN_DISTANCE: f32 = 350.0;
            let random_spawn_position =
                player_position + (rand_direction * SPAWN_DISTANCE * crate::PIXELS_PER_METER);
            let direction_to_player = (player_position - random_spawn_position).normalize() * 20.0; // maybe?

            Self::spawn_astroid(
                &mut commands,
                AstroidSize::Large,
                Composition::new_with_distance(distance_to_base_station),
                direction_to_player * crate::PIXELS_PER_METER,
                random_spawn_position,
            );
        }
    }

    fn despawn_far_astroids(
        mut commands: Commands,
        astroid_query: Query<(Entity, &mut Astroid, &mut Transform), With<Astroid>>,
        player_query: Query<(&Player, &Transform), (With<Player>, Without<Astroid>)>,
    ) {
        const DESPAWN_DISTANCE: f32 = 1000.0 * PIXELS_PER_METER;
        let (_player, transform) = player_query.single();
        let player_position = transform.translation.truncate();

        for (entity, _astroid, transform) in astroid_query.iter() {
            let astroid_position = transform.translation.truncate();
            if player_position.distance(astroid_position) > DESPAWN_DISTANCE {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    pub fn spawn_astroid(
        commands: &mut Commands,
        size: AstroidSize,
        composition: Composition,
        velocity: Vec2,
        position: Vec2,
    ) {
        let astroid_shape = match size {
            AstroidSize::OreChunk => lyon::shapes::Polygon {
                points: Self::make_valtr_convex_polygon_coords(
                    AstroidSize::OreChunk.num_sides(),
                    AstroidSize::OreChunk.radius(),
                ),
                closed: true,
            },
            AstroidSize::Small => lyon::shapes::Polygon {
                points: Self::make_valtr_convex_polygon_coords(
                    AstroidSize::Small.num_sides(),
                    AstroidSize::Small.radius(),
                ),
                closed: true,
            },
            AstroidSize::Medium => lyon::shapes::Polygon {
                points: Self::make_valtr_convex_polygon_coords(
                    AstroidSize::Medium.num_sides(),
                    AstroidSize::Medium.radius(),
                ),
                closed: true,
            },
            AstroidSize::Large => lyon::shapes::Polygon {
                points: Self::make_valtr_convex_polygon_coords(
                    AstroidSize::Large.num_sides(),
                    AstroidSize::Large.radius(),
                ),
                closed: true,
            },
        };

        let astroid = Astroid {
            velocity,
            size,
            health: Health {
                current: 100.0,
                maximum: 100.0,
                upgrade_level: crate::upgrades::UpgradeLevel::Level0,
            },
            composition: composition,
        };

        let astroid_ent = commands
            .spawn((
                astroid.clone(),
                ShapeBundle {
                    path: GeometryBuilder::build_as(&astroid_shape),
                    transform: Transform::from_xyz(position.x, position.y, 0.0),
                    ..default()
                },
                Fill::color(Color::DARK_GRAY),
                RigidBody::Dynamic,
                Velocity {
                    linvel: astroid.velocity,
                    angvel: 0.0,
                },
                Sleeping::disabled(),
                Ccd::enabled(),
                Collider::convex_hull(&astroid_shape.points).unwrap(),
                ActiveEvents::COLLISION_EVENTS,
                ReadMassProperties(MassProperties::default()),
                Restitution::coefficient(0.01),
                Name::new("Astroid"),
            )).id();

        // If the astroid is an ore chunk, add Collectible Tag
        if astroid.clone().size == AstroidSize::OreChunk {
            commands.entity(astroid_ent).insert(Collectible);
        }
    }

    fn update_collectible_material_color(
        mut astroid_query: Query<(&Astroid, &mut Fill), With<Astroid>>,
    ) {
        for (astroid, mut fill) in astroid_query.iter_mut() {
            if astroid.size == AstroidSize::OreChunk {
                // if let DrawMode::Fill(ref mut fill_mode) = *draw_mode {
                    match astroid.primary_composition() {
                        AstroidMaterial::Iron => {
                            *fill = Fill{
                                color: Color::GRAY,
                                options: FillOptions::default(),
                            };
                            // fill_mode.color = Color::GRAY;
                        }
                        AstroidMaterial::Silver => {
                            *fill = Fill{
                                color: Color::SILVER,
                                options: FillOptions::default(),
                            };

                            // fill_mode.color = Color::SILVER;
                        }
                        AstroidMaterial::Gold => {
                            *fill = Fill{
                                color: Color::GOLD,
                                options: FillOptions::default()
                            };

                            // fill_mode.color = Color::GOLD;
                        }
                        _ => {}
                    }
                // }
            }
        }
    }

    fn handle_astroid_collision_event(
        mut commands: Commands,
        rapier_context: Res<RapierContext>,
        mut astroid_query: Query<(Entity, &Astroid, &ReadMassProperties), With<Astroid>>,
        mut player_query: Query<(Entity, &mut Player, &mut Inventory), With<Player>>,
        mut effect: Query<
            (&mut ParticleEffect, &mut Transform),
            (
                With<ShipAstroidImpactParticles>,
                Without<Astroid>,
                Without<Player>,
            ),
        >,
        mut inventory_full_notification: ResMut<InventoryFullNotificationTimer>,
    ) {
        let (player_ent, mut player, mut inventory) = player_query.single_mut();

        for (astroid_entity, astroid, mass_properties) in astroid_query.iter_mut() {
            if let Some(contact_pair_view) = rapier_context.contact_pair(player_ent, astroid_entity)
            {
                for manifold in contact_pair_view.manifolds() {
                    // Read the solver contacts.

                    for solver_contact in manifold.solver_contacts() {
                        // Keep in mind that all the solver contact data are expressed in world-space.

                        let mut astroid_collision = false;

                        match astroid.size {
                            AstroidSize::OreChunk => {
                                println!("Hit ore chunk, let's collect it!");
                                let ore_chunk_mass = mass_properties.0.mass;
                                for comp in astroid.composition.percent_composition().iter() {
                                    if !inventory.add_to_inventory(&InventoryItem::Material(
                                        *comp.0,
                                        Amount::Weight(OrderedFloat(comp.1 * ore_chunk_mass)),
                                    )) {
                                        inventory_full_notification.0 =
                                            Some(Timer::from_seconds(3.0, TimerMode::Once));
                                    }
                                }

                                // FIXME: will despawn even if there's no room in inventory to collect.
                                commands.entity(astroid_entity).despawn_recursive();
                            }
                            AstroidSize::Small => {
                                println!("Hit small Astroid");
                                player.take_damage(1.0);
                                astroid_collision = true;
                            }
                            AstroidSize::Medium => {
                                println!("Hit medium Astroid");
                                player.take_damage(2.5);
                                astroid_collision = true;
                            }
                            AstroidSize::Large => {
                                println!("Hit large Astroid");
                                player.take_damage(5.0);
                                astroid_collision = true;
                            }
                        }

                        if astroid_collision {
                            let (mut effect, mut effect_translation) = effect.single_mut();
                            effect_translation.translation =
                                (solver_contact.point() * crate::PIXELS_PER_METER).extend(200.0);
                            effect.maybe_spawner().unwrap().reset();
                        }
                    }
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

    pub fn ablate_astroids(
        mut commands: Commands,
        mut astroids_query: Query<(Entity, &mut Astroid), With<Astroid>>,
        mut ablate_event_reader: EventReader<AblateEvent>,
        // mut spawn_astroid_writer: EventWriter<SpawnAstroid>
    ) {

        for ablate_event in ablate_event_reader.iter() {

            let mut rng = rand::thread_rng();
            // let split_angle = rng.gen_range(0.0..PI / 4.0); TODO: Might keep splititng astroids

            match astroids_query.get_mut(ablate_event.0) {
                Ok((ent, mut astroid_to_ablate)) => {

                    let damaged_health = astroid_to_ablate.health.current() - 1.0;
                    astroid_to_ablate.health.set_current(damaged_health);
                    println!("CURRENT HEALTH: {}", damaged_health);

                    if damaged_health < 0.0 {
                        commands.entity(ent).despawn_recursive();                    }

                    let n: u8 = rng.gen();
                    if n > 10 {
                        return;
                    }

                    let astroid_shape = lyon::shapes::Polygon {
                        points: Self::make_valtr_convex_polygon_coords(
                            AstroidSize::OreChunk.num_sides(),
                            AstroidSize::OreChunk.radius(),
                        ),
                        closed: true,
                    };
            
                
                    let astroid = Astroid {
                        velocity: ablate_event.2,
                        size: AstroidSize::OreChunk,
                        health: Health {
                            current: 100.0,
                            maximum: 100.0,
                            upgrade_level: crate::upgrades::UpgradeLevel::Level0,
                        },
                        composition: Composition::new_with_distance(100.0),
                    };
            
                    let astroid_ent = commands
                        .spawn((
                            astroid.clone(),
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&astroid_shape),
                                transform: Transform::from_xyz(ablate_event.1.x, ablate_event.1.y, 0.0),
                                ..default()
                            },
                            Fill::color(Color::DARK_GRAY),
                            RigidBody::Dynamic,
                            Velocity {
                                linvel: ablate_event.2,
                                angvel: 0.0,
                            },
                            Sleeping::disabled(),
                            Ccd::enabled(),
                            Collider::convex_hull(&astroid_shape.points).unwrap(),
                            ActiveEvents::COLLISION_EVENTS,
                            ReadMassProperties(MassProperties::default()),
                            Restitution::coefficient(0.01),
                            Name::new("Astroid"),
                        )).id();
            
                    // If the astroid is an ore chunk, add Collectible Tag
                    if astroid.clone().size == AstroidSize::OreChunk {
                        commands.entity(astroid_ent).insert(Collectible);
                    }

                },
                _ => {

                }
            }


        }
        
    }

    pub fn split_astroid(
        commands: &mut Commands,
        astroid_ent: Entity,
        astroid: &Astroid,
        astroid_translation: Vec2,
        projectile_velocity: &Velocity,
    ) {
        let mut rng = rand::thread_rng();
        let split_angle = rng.gen_range(0.0..PI / 4.0);

        let right_velocity = projectile_velocity
            .linvel
            .rotate(Vec2::from_angle(split_angle))
            * 0.5;
        let left_velocity = projectile_velocity
            .linvel
            .rotate(Vec2::from_angle(-split_angle))
            * 0.5;

        match &astroid.size {
            AstroidSize::Small => {
                let left_comp = astroid.composition.jitter();
                let right_comp = astroid.composition.jitter();

                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::OreChunk,
                    right_comp,
                    right_velocity,
                    astroid_translation,
                );
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::OreChunk,
                    left_comp,
                    left_velocity,
                    astroid_translation,
                );
                commands.entity(astroid_ent).despawn_recursive();
            }
            AstroidSize::Medium => {
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::Small,
                    astroid.composition.jitter(),
                    right_velocity,
                    astroid_translation,
                );
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::Small,
                    astroid.composition.jitter(),
                    left_velocity,
                    astroid_translation,
                );
                commands.entity(astroid_ent).despawn_recursive();
            }
            AstroidSize::Large => {
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::Medium,
                    astroid.composition.jitter(),
                    right_velocity,
                    astroid_translation,
                );
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::Medium,
                    astroid.composition.jitter(),
                    left_velocity,
                    astroid_translation,
                );
                commands.entity(astroid_ent).despawn_recursive();
            }
            _ => {}
        }
    }

    // TODO: comment this well...
    fn make_valtr_convex_polygon_coords(num_sides: usize, radius: f32) -> Vec<Vec2> {
        let mut xs: Vec<f32> = vec![];
        let mut ys: Vec<f32> = vec![];

        for _ in 0..num_sides {
            xs.push(2.0 * radius * rand::random::<f32>());
            ys.push(2.0 * radius * rand::random::<f32>());
        }

        // might be different than guide...
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min_xs = xs[0];
        let max_xs = xs[xs.len() - 1];
        let min_ys = ys[0];
        let max_ys = ys[ys.len() - 1];

        let vec_xs = make_vector_chain(xs, min_xs, max_xs);
        let mut vec_ys = make_vector_chain(ys, min_ys, max_ys);

        vec_ys.shuffle(&mut rand::thread_rng());

        let mut vecs: Vec<(f32, f32)> = vec_xs.into_iter().zip(vec_ys).collect();

        vecs.sort_by(|a, b| {
            let a_ang = a.1.atan2(a.0);
            let b_ang = b.1.atan2(b.0);

            if a_ang - b_ang < 0.0 {
                Ordering::Less
            } else if a_ang - b_ang == 0.0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });

        let mut vec_angs2: Vec<f32> = vec![];

        for vec in &vecs {
            let a = vec.1.atan2(vec.0);
            vec_angs2.push(a);
        }

        let mut poly_coords = vec![];
        let mut x = 0.0;
        let mut y = 0.0;
        for vec in &vecs {
            x += vec.0 * 1.0;
            y += vec.1 * 1.0;
            poly_coords.push(Vec2 { x, y })
        }

        fn make_vector_chain(values_array: Vec<f32>, min_value: f32, max_value: f32) -> Vec<f32> {
            let mut vector_chain: Vec<f32> = vec![];

            let mut last_min = min_value;
            let mut last_max = max_value;

            for value in values_array {
                if rand::random::<f32>() > 0.5 {
                    vector_chain.push(value - last_min);
                    last_min = value;
                } else {
                    vector_chain.push(last_max - value);
                    last_max = value;
                }
            }

            vector_chain.push(max_value - last_min);
            vector_chain.push(last_max - max_value);

            vector_chain
        }

        fn get_centroid(verticies: &Vec<Vec2>) -> Vec2 {
            let mut centroid: Vec2 = Vec2 { x: 0.0, y: 0.0 };
            let n = verticies.len();
            let mut signed_area = 0.0;

            for i in 0..n {
                let x0 = verticies[i].x;
                let y0 = verticies[i].y;
                let x1 = verticies[(i + 1) % n].x;
                let y1 = verticies[(i + 1) % n].y;

                let area = (x0 * y1) - (x1 * y0);
                signed_area += area;

                centroid.x += (x0 + x1) * area;
                centroid.y += (y0 + y1) * area;
            }

            signed_area *= 0.5;

            // what... why 6.0?
            centroid.x /= 6.0 * signed_area;
            centroid.y /= 6.0 * signed_area;

            centroid
        }

        let centroid = get_centroid(&poly_coords);
        poly_coords = poly_coords
            .iter()
            .map(|e| Vec2 {
                x: e.x - centroid.x,
                y: e.y - centroid.y,
            })
            .collect();

        poly_coords
    }
}
