use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

#[derive(Event)]
pub struct FireProjectileEvent {
    pub entity: Entity,
    pub projectile_trajectory: LinearVelocity,
}
