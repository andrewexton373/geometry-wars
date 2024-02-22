use bevy::prelude::Event;
use bevy::math::Vec2;

#[derive(Event)]
pub struct LaserEvent(pub bool, pub Vec2, pub Vec2);