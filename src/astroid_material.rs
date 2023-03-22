use std::fmt;
use bevy::prelude::{Component, FromReflect, Reflect};

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
