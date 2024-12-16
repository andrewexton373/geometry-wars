use avian2d::prelude::LinearVelocity;
use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct FireProjectileEvent {
    pub entity: Entity,
    pub projectile_trajectory: LinearVelocity,
}
