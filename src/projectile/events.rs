use avian2d::prelude::LinearVelocity;
use bevy::{
    ecs::message::Message,
    prelude::{Entity, Event},
};

#[derive(Event, Message)]
pub struct FireProjectileEvent {
    pub entity: Entity,
    pub projectile_trajectory: LinearVelocity,
}
