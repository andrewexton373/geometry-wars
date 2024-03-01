use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub timer: Timer,
}