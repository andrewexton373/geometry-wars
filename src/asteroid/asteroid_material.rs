use bevy::prelude::{Component, Reflect};
use std::fmt;

#[derive(
    Component, Reflect, Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd,
)]
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
