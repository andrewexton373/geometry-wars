use crate::asteroid::asteroid_material::AsteroidMaterial;
use bevy::prelude::Component;
use bevy::utils::HashMap;
use rand::distributions::Distribution;
use rand_distr::Normal;
use std::fmt;

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
