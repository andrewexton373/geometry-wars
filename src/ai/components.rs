use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Fill, entity::{Path, ShapeBundle}, geometry::{Geometry, GeometryBuilder}, shapes};
use bevy_xpbd_2d::components::Collider;
use big_brain::prelude::*;

#[derive(Debug, Clone, Component, ScorerBuilder)]
pub struct Hostile;

#[derive(Component, Debug)]
pub struct Hostility {
    pub per_second: f32,
    pub hostility: f32
}

impl Hostility {
    pub fn new(hostility: f32, per_second: f32) -> Self {
        Self {hostility, per_second}
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Attack {
    pub until: f32,
    pub per_second: f32
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveTowardsPlayer {
    pub speed: f32,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Position {
    pub position: Vec2,
}

#[derive(Component)]
pub struct Enemy;







