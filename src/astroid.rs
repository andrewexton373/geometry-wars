use bevy::prelude::*;
use bevy::reflect::FromReflect;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::{self as lyon, DrawMode};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use rand::Rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::f32::consts::{PI};
use std::fmt;
use crate::inventory::{Inventory, InventoryPlugin, self};
use crate::{ Player, PIXELS_PER_METER };
use crate::player::Health;

pub struct AstroidPlugin;

#[derive(Component, Inspectable, Clone, Copy, Debug)]
pub struct Astroid {
    pub velocity: Vec2,
    pub size: AstroidSize,
    pub material: AstroidMaterial,
    pub health: Health
}

#[derive(Clone, Copy, Inspectable, Debug, PartialEq)]
pub enum AstroidSize {
    OreChunk,
    Small,
    Medium,
    Large
}

#[derive(Component)]
pub struct Collectible;

impl AstroidSize {
    fn radius(self) -> f32 {
        match self {
            Self::OreChunk => 1.0,
            Self::Small => 2.0,
            Self::Medium => 5.0,
            Self::Large => 10.0
        }
    }
}

// impl fmt::Display for AstroidSize {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({})", self)
//     }
// }

#[derive(Component, Inspectable, Reflect, FromReflect, Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
pub enum AstroidMaterial {
    #[default]
    Rock,
    Iron,
    Gold,
    Silver
}

impl fmt::Display for AstroidMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self)
    }
}

#[derive(Component)]
pub struct AstroidSpawner {
    timer: Timer
}

impl Plugin for AstroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AstroidSpawner{ timer: Timer::from_seconds(0.25, false)})
            .add_system(Self::spawn_astroids_aimed_at_ship)
            .add_system(Self::despawn_far_astroids)
            .add_system(Self::handle_astroid_collision_event)
            .add_system(Self::update_collectible_material_color)
            .register_inspectable::<Astroid>();
    }
}

impl AstroidPlugin {
    fn spawn_astroids_aimed_at_ship(
        mut commands: Commands,
        player_query: Query<(&Player, &Transform)>,
        mut astroid_spawner: ResMut<AstroidSpawner>,
        time: Res<Time>
    ) {
        astroid_spawner.timer.tick(time.delta());

        if astroid_spawner.timer.finished() {
            astroid_spawner.timer.reset();

            let mut rng = rand::thread_rng();
            let (_player, transform) = player_query.single();

            let player_position = transform.translation.truncate();

            let rand_x: f32 = rng.gen_range(-PI..PI);
            let rand_y: f32 = rng.gen_range(-PI..PI);
            let rand_direction = Vec2::new(rand_x.cos(), rand_y.sin()).normalize();

            const SPAWN_DISTANCE: f32 = 300.0;
            let random_spawn_position = player_position + (rand_direction * SPAWN_DISTANCE * crate::PIXELS_PER_METER);
            let direction_to_player = (player_position - random_spawn_position).normalize() * 20.0; // maybe?

            Self::spawn_astroid(&mut commands, AstroidSize::Large, AstroidMaterial::Rock, direction_to_player * crate::PIXELS_PER_METER, random_spawn_position);
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
            // println!("{}", player_position.distance(astroid_position));
            if player_position.distance(astroid_position) > DESPAWN_DISTANCE {
                // println!("DESPAWN: {}", player_position.distance(astroid_position));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
    
    pub fn spawn_astroid(
        commands: &mut Commands,
        size: AstroidSize,
        material: AstroidMaterial,
        velocity: Vec2,
        position: Vec2
    ) {
    
        let astroid_shape = match size {
            AstroidSize::OreChunk => {
                lyon::shapes::Polygon {
                    points: Self::make_valtr_convex_polygon_coords(5, 25.0),
                    closed: true
                }
            },
            AstroidSize::Small => {
                lyon::shapes::Polygon {
                    points: Self::make_valtr_convex_polygon_coords(7, 45.0),
                    closed: true
                }
            },
            AstroidSize::Medium => {
                lyon::shapes::Polygon {
                    points: Self::make_valtr_convex_polygon_coords(9, 85.0),
                    closed: true
                }
            },
            AstroidSize::Large => {
                lyon::shapes::Polygon {
                    points: Self::make_valtr_convex_polygon_coords(11, 100.0),
                    closed: true
                }
            }
        };

        let astroid = Astroid {
            velocity: velocity,
            size: size,
            material: material,
            health: Health {current: 50.0, maximum: 100.0}
        };
    
        let astroid_ent = commands.spawn()
            .insert(astroid)
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &astroid_shape,
                lyon::DrawMode::Fill(lyon::FillMode::color(Color::DARK_GRAY)),
                Default::default()
            ))
            // .insert(Collider) // do we still need? don't think so
            .insert(RigidBody::Dynamic)
            .insert(Velocity { linvel: astroid.velocity, angvel: 0.0 })
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(Collider::convex_hull(&astroid_shape.points).unwrap())
            .insert(Transform::from_xyz(position.x, position.y, 0.0))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ReadMassProperties(MassProperties::default()))
            .insert(Restitution::coefficient(0.01))
            .insert(Name::new("Astroid"))
            .id();

        // If the astroid is an ore chunk, add Collectible Tag
        if astroid.size == AstroidSize::OreChunk {
            commands.entity(astroid_ent)
                .insert(Collectible);
        }
            
    }

    fn update_collectible_material_color(
        mut astroid_query: Query<(&Astroid, &mut DrawMode), With<Astroid>>
    ) {

        for (astroid, mut draw_mode) in astroid_query.iter_mut() {

            match astroid.size {
                AstroidSize::OreChunk => {
                    if let DrawMode::Fill(ref mut fill_mode) = *draw_mode {

                        match astroid.material {
                            AstroidMaterial::Iron => {
                                fill_mode.color = Color::GRAY;
                            },
                            AstroidMaterial::Silver => {
                                fill_mode.color = Color::SILVER;
                            },
                            AstroidMaterial::Gold => {
                                fill_mode.color = Color::GOLD;
                            },
                            _ => {}
                        }

                    }
                },

                _ => {

                }
            }
        }

    }

    fn handle_astroid_collision_event(
        mut astroid_query: Query<(Entity, &Astroid, &ReadMassProperties), With<Astroid>>,
        mut player_query: Query<(Entity, &mut Player, &mut Inventory), With<Player>>,
        mut contact_events: EventReader<CollisionEvent>,
        // mut inventory_resource: ResMut<Inventory>,
        mut commands: Commands
    ) {
        let (player_ent, mut player, mut inventory) = player_query.single_mut();

        for contact_event in contact_events.iter() {
            for (astroid_entity, astroid, mass_properties) in astroid_query.iter_mut() {
                if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                    
                    // If player hit astroid
                    if player_ent == *h1 && astroid_entity == *h2 {

                        match astroid.size {
                            AstroidSize::OreChunk => {
                                println!("Hit ore chunk, let's collect it!");
                                // commands.entity(astroid_entity).despawn_recursive();
                                let ore_chunk_mass = mass_properties.0.mass;
                                // inventory_resource.add_to_inventory(astroid.material, ore_chunk_mass);

                                if inventory.add_to_inventory(astroid.material, ore_chunk_mass) {
                                    commands.entity(astroid_entity).despawn_recursive();
                                }
                            }
                            AstroidSize::Small => {
                                println!("Hit small Astroid");
                                player.take_damage(1.0);
                            },
                            AstroidSize::Medium => {
                                println!("Hit medium Astroid");
                                player.take_damage(2.5);
                                // commands.entity(ent).despawn_recursive();

                            },
                            AstroidSize::Large => {
                                println!("Hit large Astroid");
                                player.take_damage(5.0);
                                // commands.entity(ent).despawn_recursive();

                            }
                        }
                        return;
                    }
                    
                }
            }
        }
    }

    pub fn split_astroid(commands: &mut Commands, astroid_ent: Entity, astroid: &Astroid, astroid_translation: Vec2, projectile_velocity: &Velocity) {

        let mut rng = rand::thread_rng();
        let split_angle = rng.gen_range(0.0..PI/4.0);
        
        let right_velocity = projectile_velocity.linvel.rotate(Vec2::from_angle(split_angle)) * 0.5;
        let left_velocity = projectile_velocity.linvel.rotate(Vec2::from_angle(-split_angle)) * 0.5;

        match &astroid.size {
            AstroidSize::Small => {

                fn random_ore_material() -> AstroidMaterial {
                    let mut rng = rand::thread_rng();

                    // let mut material: AstroidMaterial;

                   match rng.gen::<u8>() % 3 {
                        0 => {
                            AstroidMaterial::Iron
                        },
                        1 => {
                            AstroidMaterial::Silver
                        },
                        2 => {
                            AstroidMaterial::Gold
                        },
                        _ => {
                            AstroidMaterial::Rock
                        }
                    }
                }

                AstroidPlugin::spawn_astroid(commands, AstroidSize::OreChunk, random_ore_material(), right_velocity, astroid_translation);
                AstroidPlugin::spawn_astroid(commands, AstroidSize::OreChunk, random_ore_material(), left_velocity, astroid_translation);
                commands.entity(astroid_ent).despawn_recursive();

            },
            AstroidSize::Medium => {
                AstroidPlugin::spawn_astroid(commands, AstroidSize::Small, AstroidMaterial::Rock, right_velocity, astroid_translation);
                AstroidPlugin::spawn_astroid(commands, AstroidSize::Small, AstroidMaterial::Rock, left_velocity, astroid_translation);
                commands.entity(astroid_ent).despawn_recursive();
            },
            AstroidSize::Large => {
                AstroidPlugin::spawn_astroid(commands, AstroidSize::Medium, AstroidMaterial::Rock, right_velocity,astroid_translation);
                AstroidPlugin::spawn_astroid(commands, AstroidSize::Medium, AstroidMaterial::Rock, left_velocity, astroid_translation);
                commands.entity(astroid_ent).despawn_recursive();
            }
            _ => {
                
            }
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
        xs.sort_by(|a, b| {a.partial_cmp(b).unwrap()});
        xs.sort_by(|a, b| {a.partial_cmp(b).unwrap()});


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

        let mut  poly_coords = vec![];
        let mut x = 0.0;
        let mut  y = 0.0;
        for vec in &vecs {
            x += vec.0 * 1.0;
            y += vec.1 * 1.0;
            poly_coords.push(Vec2 { x: x, y: y })
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
                let x1 = verticies[(i+1)%n].x;
                let y1 = verticies[(i+1)%n].y;

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
        poly_coords = poly_coords.iter().map(|e| {
            Vec2 { x: e.x - centroid.x, y: e.y - centroid.y }
        }).collect();

        poly_coords
    }
}