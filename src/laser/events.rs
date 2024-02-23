use bevy::math::Vec2;
use bevy::prelude::Event;

#[derive(Event)]
pub struct LaserEvent(pub bool, pub Vec2, pub Vec2);
