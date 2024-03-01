use bevy::{
    math::Vec2,
    prelude::{Entity, Event},
    transform::components::Transform,
};
use bevy_xpbd_2d::components::LinearVelocity;

use super::components::Asteroid;

#[derive(Event)]
pub struct AblateEvent(pub Entity, pub Vec2, pub Vec2);

#[derive(Event)]
pub struct SpawnAsteroidEvent(pub Asteroid, pub Transform, pub LinearVelocity);

#[derive(Event)]
pub struct SplitAsteroidEvent(pub Entity);
