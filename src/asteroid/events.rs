use avian2d::prelude::LinearVelocity;
use bevy::{
    math::Vec2,
    prelude::{Entity, Event},
    transform::components::Transform,
};

use super::components::Asteroid;

#[derive(Event)]
pub struct AblateEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub normal: Vec2,
}

#[derive(Event)]
pub struct SpawnAsteroidEvent(pub Asteroid, pub Transform, pub LinearVelocity);

#[derive(Event)]
pub struct SplitAsteroidEvent(pub Entity);
