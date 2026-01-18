use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use geo::{ConvexHull, MultiPoint, Point};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::fmt;

#[derive(Component)]
pub struct Splittable(pub f32);

#[derive(Component, Clone, Debug)]
pub struct Asteroid {
    pub composition: AsteroidComposition,
    pub polygon: ConvexPolygon,
    pub radius: f32,
}

impl Asteroid {
    pub fn new_with(radius: f32, comp: AsteroidComposition) -> Self {
        let asteroid_polygon = Self::generate_shape_from_size(radius);

        Self {
            composition: comp,
            polygon: asteroid_polygon,
            radius,
        }
    }

    pub fn primary_composition(&self) -> AsteroidMaterial {
        self.composition.most_abundant()
    }

    pub fn polygon(&self) -> ConvexPolygon {
        self.polygon.clone()
    }

    pub fn generate_mesh(&self) -> Mesh {
        let verticies: Vec<Vec3> = self
            .polygon()
            .vertices()
            .to_vec()
            .iter()
            .map(|v| v.extend(0.0))
            .collect();
        let indicies = Self::create_triangles_for_mesh(&self.polygon().vertices());

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verticies);
        mesh.insert_indices(Indices::U32(indicies));

        mesh
    }

    fn create_triangles_for_mesh(verticies: &[Vec2]) -> Vec<u32> {
        let mut indicies = vec![];
        let n = verticies.len();

        for i in 1..n - 1 {
            indicies.push(0);
            indicies.push(i as u32);
            indicies.push((i + 1) as u32);
        }

        indicies
    }

    fn generate_shape_from_size(radius: f32) -> ConvexPolygon {
        let rand_side_count = rand::rng().random_range(6..20);
        match ConvexPolygon::new(
            Self::random_convex_polygon(
                &mut rand::rng(),
                rand_side_count,
                (0.0, 0.0),
                radius as f64,
            )
            .vertices,
        ) {
            Ok(polygon) => polygon,
            Err(e) => panic!("Failed to generate convex polygon: {:?}", e),
        }
    }

    pub fn random_convex_polygon<R: Rng>(
        rng: &mut R,
        n_points: usize,
        center: (f64, f64),
        radius: f64,
    ) -> Polygon {
        assert!(n_points >= 3);

        // Random points in a disk (uniform-ish area: r = sqrt(u))
        let mut pts = Vec::with_capacity(n_points);
        for _ in 0..n_points {
            let a = rng.random_range(0.0..std::f64::consts::TAU);
            let r = radius * rng.random::<f64>().sqrt();
            let x = center.0 + r * a.cos();
            let y = center.1 + r * a.sin();
            pts.push(Point::new(x, y));
        }

        let mp: MultiPoint<f64> = pts.into();

        fn geo_to_bevy_polygon(poly: &geo::Polygon<f64>) -> Polygon {
            // geo rings repeat the first point at the end; Bevy's Polygon expects unique vertices.
            let coords = &poly.exterior().0;

            let vertices: Vec<Vec2> = coords
                .iter()
                .take(coords.len().saturating_sub(1))
                .map(|c| Vec2::new(c.x as f32, c.y as f32))
                .collect();

            Polygon::new(vertices)
        }
        geo_to_bevy_polygon(&mp.convex_hull())
        // mp.convex_hull().into()
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
    Large,
}

impl AsteroidSize {
    pub fn radius(self) -> f32 {
        match self {
            Self::OreChunk => 25.0,
            Self::Large => 100.0,
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
        let mut rng = rand::rng();
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
        for element in self.composition.iter() {
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
