use bevy::prelude::*;
use bevy_xpbd_2d::components::LinearVelocity;

#[derive(Event)]
pub struct FireProjectileEvent {
    pub entity: Entity,
    pub projectile_trajectory: LinearVelocity,
}
