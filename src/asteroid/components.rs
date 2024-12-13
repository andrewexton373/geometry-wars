use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_prototype_lyon::shapes::Polygon;
use rand::{distributions::Distribution, seq::SliceRandom};
use rand_distr::Normal;
use std::{cmp::Ordering, fmt};

#[derive(Component)]
pub struct Splittable(pub f32);

#[derive(Component, Clone, Debug)]
pub struct Asteroid {
    // pub size: AsteroidSize,
    // pub health: Health,
    pub composition: AsteroidComposition,
    pub polygon: Polygon,
    pub radius: f32,
}

impl Asteroid {
    pub fn polygon_area(verticies: Vec<Vec2>) -> f32 {
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

        let poly = Polygon::new(asteroid_polygon_tuple, vec![]);
        poly.signed_area()
    }

    pub fn new_with(radius: f32, comp: AsteroidComposition) -> Self {
        let asteroid_polygon = Self::generate_shape_from_size(radius);
        let poly_area = Self::polygon_area(asteroid_polygon.points.clone());

        // Compute Health from Generated Shape Mass?

        Self {
            // size,
            composition: comp,
            polygon: asteroid_polygon,
            radius,
        }
    }

    pub fn primary_composition(&self) -> AsteroidMaterial {
        self.composition.most_abundant()
    }

    pub fn polygon(&self) -> Polygon {
        self.polygon.clone()
    }

    fn generate_shape_from_size(radius: f32) -> Polygon {
        return Polygon {
            points: Self::make_valtr_convex_polygon_coords(6, radius),
            closed: true,
        };
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

        fn get_centroid(verticies: &[Vec2]) -> Vec2 {
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

#[derive(Component, Reflect, Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
pub enum AsteroidMaterial {
    #[default]
    Rock,
    Iron,
    Silver,
    Gold,
}

impl fmt::Display for AsteroidMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AsteroidMaterial::Rock => write!(f, "Rock"),
            AsteroidMaterial::Iron => write!(f, "Iron"),
            AsteroidMaterial::Silver => write!(f, "Silver"),
            AsteroidMaterial::Gold => write!(f, "Gold"),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsteroidSize {
    OreChunk,
    Small,
    Medium,
    Large,
}

impl AsteroidSize {
    pub fn radius(self) -> f32 {
        match self {
            Self::OreChunk => 25.0,
            Self::Small => 45.0,
            Self::Medium => 85.0,
            Self::Large => 100.0,
        }
    }

    pub fn num_sides(self) -> usize {
        match self {
            Self::OreChunk => 5,
            Self::Small => 7,
            Self::Medium => 9,
            Self::Large => 11,
        }
    }
}

#[derive(Component, Clone)]
pub struct AsteroidComposition {
    composition: HashMap<AsteroidMaterial, f32>,
}

impl AsteroidComposition {
    pub fn new_with_distance(distance: f32) -> Self {
        const MIN_DISTANCE: f32 = 0.0;
        const MAX_DISTANCE: f32 = 100000.0;

        let percentage =
            ((distance - MIN_DISTANCE) / (MAX_DISTANCE - MIN_DISTANCE)).clamp(0.0, 1.0);

        let mut near_composition: HashMap<AsteroidMaterial, f32> = HashMap::new();
        near_composition.insert(AsteroidMaterial::Iron, 0.95);
        near_composition.insert(AsteroidMaterial::Silver, 0.04);
        near_composition.insert(AsteroidMaterial::Gold, 0.01);

        let mut far_composition: HashMap<AsteroidMaterial, f32> = HashMap::new();
        far_composition.insert(AsteroidMaterial::Iron, 1.0);
        far_composition.insert(AsteroidMaterial::Silver, 2.0);
        far_composition.insert(AsteroidMaterial::Gold, 2.0);

        let mut composition = HashMap::new();

        for near in near_composition.iter() {
            let far = far_composition.get(near.0).unwrap();

            composition.insert(*near.0, near.1 + (far - near.1) * percentage);
        }

        Self { composition }
    }

    pub fn most_abundant(&self) -> AsteroidMaterial {
        self.composition
            .iter()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .map(|(k, _v)| *k)
            .unwrap()
    }

    pub fn percent_composition(&self) -> HashMap<AsteroidMaterial, f32> {
        let cloned: HashMap<AsteroidMaterial, f32> = self.composition.clone();
        let total_weights: f32 = cloned.iter().map(|e| e.1).sum();
        cloned
            .into_iter()
            .map(|e| (e.0, e.1 / total_weights))
            .collect::<HashMap<AsteroidMaterial, f32>>()
    }

    pub fn jitter(&self) -> AsteroidComposition {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.05).unwrap();

        AsteroidComposition {
            composition: self
                .percent_composition()
                .into_iter()
                .map(|(k, v)| (k, (v + normal.sample(&mut rng)).clamp(0.0, f32::MAX)))
                .collect(),
        }
    }
}

impl fmt::Debug for AsteroidComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for element in &self.composition {
            let _ = writeln!(f, "{:?}: {:.2}%", element.0, element.1 * 100.0);
        }
        write!(f, "")
    }
}

#[test]
fn test_most_abundant() {
    assert_eq!(
        AsteroidComposition::new_with_distance(0.0).most_abundant(),
        AsteroidMaterial::Iron
    );
    assert_eq!(
        AsteroidComposition::new_with_distance(10000.0).most_abundant(),
        AsteroidMaterial::Gold
    );
}
