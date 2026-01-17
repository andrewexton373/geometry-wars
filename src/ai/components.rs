use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Hostile;

#[derive(Component, Debug)]
pub struct Hostility {
    pub per_second: f32,
    pub hostility: f32,
}

impl Hostility {
    pub fn new(hostility: f32, per_second: f32) -> Self {
        Self {
            hostility,
            per_second,
        }
    }
}

#[derive(Clone, Component, Debug)]
pub struct Attack {
    pub per_second: f32,
}

#[derive(Clone, Component, Debug)]
pub struct MoveTowardsPlayer {
    pub speed: f32,
}

#[derive(Component)]
pub struct Enemy;
