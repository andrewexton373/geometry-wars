use crate::asteroid::asteroid_composition::AsteroidComposition;
use crate::asteroid::asteroid_material::AsteroidMaterial;

use crate::asteroid::asteroid_size::{AsteroidSize, Collectible};
use crate::health::Health;
use bevy::prelude::*;
use bevy_prototype_lyon::draw::Fill;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::prelude::{self as lyon};
use bevy_prototype_lyon::shapes::Polygon;
use bevy_xpbd_2d::components::{Collider, LinearVelocity, RigidBody};
use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp::Ordering;

use super::plugin::Splittable; // Contains the ratio the asteroid will split at


#[derive(Component, Clone, Debug)]
pub struct Asteroid {
    pub size: AsteroidSize,
    pub health: Health,
    pub composition: AsteroidComposition,
    polygon: Polygon,
}


impl Asteroid {
    fn polygon_area(verticies: Vec<Vec2>) -> f32 {
        use geo::{Area, Coord, LineString, Point, Polygon};

        let asteroid_polygon_tuple = verticies
            .into_iter()
            .map(|item| {
                Point(Coord {
                    x: item.x,
                    y: item.y,
                })
            })
            .collect::<LineString<f32>>();

        let poly = Polygon::new(LineString::from(asteroid_polygon_tuple), vec![]);
        poly.signed_area()
    }

    pub fn new_with(size: AsteroidSize, comp: AsteroidComposition) -> Self {
        let asteroid_polygon = Self::generate_shape_from_size(size);

        let poly_area = Self::polygon_area(asteroid_polygon.points.clone());

        // Compute Health from Generated Shape Mass?

        Self {
            size: size,
            health: Health {
                current: poly_area,
                maximum: poly_area,
                upgrade_level: crate::upgrades::UpgradeLevel::Level0,
            },
            composition: comp,
            polygon: asteroid_polygon,
        }
    }

    pub fn spawn_asteroid(
        commands: &mut Commands,
        size: AsteroidSize,
        composition: AsteroidComposition,
        velocity: Vec2,
        position: Vec2,
    ) {
        let asteroid = Asteroid::new_with(size, composition);
        
        let mut rng = rand::thread_rng();

        let splittable = Splittable(rng.gen_range(0.4..0.8));

        let asteroid_ent = commands
            .spawn(asteroid.clone())
            .insert((
                RigidBody::Dynamic,
                LinearVelocity(velocity),
                Collider::convex_hull(asteroid.polygon().points).unwrap(),
                splittable,
                Name::new("Asteroid"),
            )).insert((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&asteroid.polygon()),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.0)),
                    ..default()
                },                        
                Fill::color(Color::DARK_GRAY),
            )).id();

        // If the asteroid is an ore chunk, add Collectible Tag
        if asteroid.clone().size == AsteroidSize::OreChunk {
            commands.entity(asteroid_ent).insert(Collectible);
        }
        
    }

    pub fn primary_composition(&self) -> AsteroidMaterial {
        self.composition.most_abundant()
    }

    pub fn polygon(&self) -> Polygon {
        self.polygon.clone()
    }

    fn generate_shape_from_size(size: AsteroidSize) -> Polygon {
        match size {
            AsteroidSize::OreChunk => lyon::shapes::Polygon {
                points: Self::make_valtr_convex_polygon_coords(
                    AsteroidSize::OreChunk.num_sides(),
                    AsteroidSize::OreChunk.radius(),
                ),
                closed: true,
            },
            AsteroidSize::Small => lyon::shapes::Polygon {
                points: Self::make_valtr_convex_polygon_coords(
                    AsteroidSize::Small.num_sides(),
                    AsteroidSize::Small.radius(),
                ),
                closed: true,
            },
            AsteroidSize::Medium => lyon::shapes::Polygon {
                points: Self::make_valtr_convex_polygon_coords(
                    AsteroidSize::Medium.num_sides(),
                    AsteroidSize::Medium.radius(),
                ),
                closed: true,
            },
            AsteroidSize::Large => lyon::shapes::Polygon {
                points: Self::make_valtr_convex_polygon_coords(
                    AsteroidSize::Large.num_sides(),
                    AsteroidSize::Large.radius(),
                ),
                closed: true,
            },
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
            .into_iter()
            .map(|e| Vec2 {
                x: e.x - centroid.x,
                y: e.y - centroid.y,
            })
            .collect();

        poly_coords
    }
}