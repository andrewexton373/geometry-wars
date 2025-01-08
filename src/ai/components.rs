use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Debug, Clone, Component, ScorerBuilder)]
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

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Attack {
    pub per_second: f32,
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveTowardsPlayer {
    pub speed: f32,
}

#[derive(Component)]
pub struct Enemy;
