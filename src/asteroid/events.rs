use avian2d::prelude::LinearVelocity;
use bevy::{
    ecs::message::Message,
    math::Vec2,
    prelude::{Entity, Event},
    transform::components::Transform,
};

use super::components::Asteroid;

#[derive(Event, Message)]
pub struct AblateEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub normal: Vec2,
}

#[derive(Event, Message)]
pub struct SpawnAsteroidEvent(pub Asteroid, pub Transform, pub LinearVelocity);

#[derive(Event, Message)]
pub struct SplitAsteroidEvent(pub Entity);
