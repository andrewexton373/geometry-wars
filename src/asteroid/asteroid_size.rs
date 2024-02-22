use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsteroidSize {
    OreChunk,
    Small,
    Medium,
    Large,
}

#[derive(Component)]
pub struct Collectible;

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
